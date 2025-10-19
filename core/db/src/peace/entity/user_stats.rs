//! Consolidated `user_stats` entity for all game modes

use super::sea_orm_active_enums::GameMode;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_stats")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub mode: GameMode,
    pub total_score: i64,
    pub ranked_score: i64,
    pub playcount: i32,
    pub total_hits: i32,
    #[sea_orm(column_type = "Decimal(Some((6, 2)))")]
    pub accuracy: Decimal,
    pub max_combo: i32,
    pub total_seconds_played: i32,
    pub count300: i32,
    pub count100: i32,
    pub count50: i32,
    pub count_miss: i32,
    pub count_failed: i32,
    pub count_quit: i32,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Users,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
