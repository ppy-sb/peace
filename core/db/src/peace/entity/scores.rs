//! Base `scores` entity (common fields only). Extended per-mode data is in
//! `scores_classic` for classic game modes.
use crate::peace::entity::sea_orm_active_enums::ScoreKind;

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "scores")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    // map md5
    pub map_hash: String,
    pub user_id: i32,
    // score checksum
    #[sea_orm(unique)]
    pub cksm: String,
    // discriminator for which extended table owns this score row (e.g. "classic")
    pub kind: ScoreKind,

    pub playtime: i32,
    pub completed: bool,

    pub invisible: bool,
    // pub verified: bool,
    pub verified_at: Option<DateTimeWithTimeZone>, // Some value implicitly means verified
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
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
    #[sea_orm(has_one = "super::scores_classic::Entity")]
    ScoresClassic,
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

impl Related<super::scores_classic::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ScoresClassic.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
