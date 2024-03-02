//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::{entity::prelude::*, FromJsonQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct EpisodeFileMeta {
    pub media_path: String,
    pub group: Option<String>,
    pub title: String,
    pub season: i32,
    pub episode_index: Option<i32>,
    pub extension: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct SubtitleFileMeta {
    pub episode_file_meta: EpisodeFileMeta,
    pub extension: String,
    pub lang: Option<String>,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "episodes")]
pub struct Model {
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[sea_orm(primary_key)]
    pub id: i32,
    pub raw_name: String,
    pub display_name: String,
    pub bangumi_id: i32,
    pub download_id: i32,
    pub save_path: String,
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