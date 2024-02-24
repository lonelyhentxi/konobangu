//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "episodes")]
pub struct Model {
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[sea_orm(primary_key)]
    pub id: i32,
    pub display_name: String,
    pub bangumi_id: i32,
    pub output_name: String,
    pub download_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
    belongs_to = "super::bangumi::Entity",
    from = "Column::BangumiId",
    to = "super::bangumi::Column::Id"
    )]
    Bangumi,
    #[sea_orm(
    belongs_to = "super::downloads::Entity",
    from = "Column::DownloadId",
    to = "super::downloads::Column::Id"
    )]
    Downloads,
}

impl Related<super::bangumi::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bangumi.def()
    }
}

impl Related<super::downloads::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Downloads.def()
    }
}
