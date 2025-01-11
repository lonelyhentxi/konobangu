use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponse as _, Response},
};
use jwt_authorizer::{JwtAuthorizer, Validation};
use loco_rs::app::{AppContext, Initializer};
use once_cell::sync::OnceCell;
use reqwest::header::HeaderValue;

use super::{
    basic::BasicAuthService,
    errors::AuthError,
    oidc::{OidcAuthClaims, OidcAuthService},
    AppAuthConfig,
};
use crate::{app::AppContextExt as _, config::AppConfigExt, models::auth::AuthType};

#[derive(Clone, Debug)]
pub struct AuthUserInfo {
    pub user_pid: String,
    pub auth_type: AuthType,
}

impl FromRequestParts<AppContext> for AuthUserInfo {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppContext,
    ) -> Result<Self, Self::Rejection> {
        let auth_service = state.get_auth_service();

        auth_service
            .extract_user_info(parts)
            .await
            .map_err(|err| err.into_response())
    }
}

#[async_trait]
pub trait AuthService {
    async fn extract_user_info(&self, request: &mut Parts) -> Result<AuthUserInfo, AuthError>;
    fn www_authenticate_header_value(&self) -> Option<HeaderValue>;
}

pub enum AppAuthService {
    Basic(BasicAuthService),
    Oidc(OidcAuthService),
}

static APP_AUTH_SERVICE: OnceCell<AppAuthService> = OnceCell::new();

impl AppAuthService {
    pub fn app_instance() -> &'static Self {
        APP_AUTH_SERVICE
            .get()
            .expect("AppAuthService is not initialized")
    }

    pub async fn from_conf(config: AppAuthConfig) -> Result<Self, AuthError> {
        let result = match config {
            AppAuthConfig::Basic(config) => AppAuthService::Basic(BasicAuthService { config }),
            AppAuthConfig::Oidc(config) => {
                let validation = Validation::new()
                    .iss(&[&config.issuer])
                    .aud(&[&config.audience]);

                let jwt_auth = JwtAuthorizer::<OidcAuthClaims>::from_oidc(&config.issuer)
                    .validation(validation)
                    .build()
                    .await?;

                AppAuthService::Oidc(OidcAuthService {
                    config,
                    authorizer: jwt_auth,
                })
            }
        };
        Ok(result)
    }
}

#[async_trait]
impl AuthService for AppAuthService {
    async fn extract_user_info(&self, request: &mut Parts) -> Result<AuthUserInfo, AuthError> {
        match self {
            AppAuthService::Basic(service) => service.extract_user_info(request).await,
            AppAuthService::Oidc(service) => service.extract_user_info(request).await,
        }
    }

    fn www_authenticate_header_value(&self) -> Option<HeaderValue> {
        match self {
            AppAuthService::Basic(service) => service.www_authenticate_header_value(),
            AppAuthService::Oidc(service) => service.www_authenticate_header_value(),
        }
    }
}

pub struct AppAuthServiceInitializer;

#[async_trait]
impl Initializer for AppAuthServiceInitializer {
    fn name(&self) -> String {
        String::from("AppAuthServiceInitializer")
    }

    async fn before_run(&self, ctx: &AppContext) -> Result<(), loco_rs::Error> {
        let auth_conf = ctx.config.get_app_conf()?.auth;

        let service = AppAuthService::from_conf(auth_conf)
            .await
            .map_err(loco_rs::Error::wrap)?;

        APP_AUTH_SERVICE.get_or_init(|| service);

        Ok(())
    }
}
