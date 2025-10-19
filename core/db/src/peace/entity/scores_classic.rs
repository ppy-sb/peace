//! Extended `scores_classic` entity holding classic-mode-specific score fields.

use super::sea_orm_active_enums::GameMode;
use super::sea_orm_active_enums::ScoreGrade;
use super::sea_orm_active_enums::ScoreVersion;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "scores_classic")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    // distinguish mode (Standard, Taiko, Fruits, Mania)
    pub mode: GameMode,
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
    pub perfect: bool,
    pub grade: ScoreGrade,
    pub client_flags: i32,
    pub client_version: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::scores::Entity",
        from = "Column::Id",
        to = "super::scores::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Scores,
}

impl Related<super::scores::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Scores.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
