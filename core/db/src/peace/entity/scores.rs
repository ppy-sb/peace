//! Consolidated `scores` entity for all game modes

use super::sea_orm_active_enums::GameMode;
use super::sea_orm_active_enums::ScoreGrade;
use super::sea_orm_active_enums::ScoreStatus;
use super::sea_orm_active_enums::ScoreVersion;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "scores")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: i32,
    #[sea_orm(unique)]
    pub score_md5: String,
    pub map_md5: String,
    pub score_version: ScoreVersion,
    pub score: i32,
    #[sea_orm(column_type = "Decimal(Some((6, 2)))")]
    pub accuracy: Decimal,
    pub combo: i32,
    pub mods: i32,
    pub n300: i32,
    pub n100: i32,
    pub n50: i32,
    pub miss: i32,
    pub geki: i32,
    pub katu: i32,
    pub playtime: i32,
    pub perfect: bool,
    pub status: ScoreStatus,
    pub grade: ScoreGrade,
    pub client_flags: i32,
    pub client_version: String,
    pub confidence: Option<i32>,
    pub verified: bool,
    pub invisible: bool,
    pub verify_at: Option<DateTimeWithTimeZone>,
    pub create_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    // distinguish mode (Standard, Taiko, Fruits, Mania, and their relax/autopilot variants)
    pub mode: GameMode,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::leaderboard::Entity")]
    Leaderboard,
    #[sea_orm(has_many = "super::score_pp::Entity")]
    ScorePp,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Users,
}

impl Related<super::leaderboard::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Leaderboard.def()
    }
}

impl Related<super::score_pp::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ScorePp.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
