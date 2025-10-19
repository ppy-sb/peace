//! Consolidated `leaderboard` entity for all game modes

use super::sea_orm_active_enums::GameMode;
use super::sea_orm_active_enums::RankingType;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "leaderboard")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub beatmap_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub mode: GameMode,
    #[sea_orm(primary_key, auto_increment = false)]
    pub ranking_type: RankingType,
    pub user_id: i32,
    pub score_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::beatmaps::Entity",
        from = "Column::BeatmapId",
        to = "super::beatmaps::Column::Bid",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Beatmaps,
    #[sea_orm(
        belongs_to = "super::scores::Entity",
        from = "Column::ScoreId",
        to = "super::scores::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Scores,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Users,
}

impl Related<super::beatmaps::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Beatmaps.def()
    }
}

impl Related<super::scores::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Scores.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
