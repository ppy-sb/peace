//! `scores_generic` entity holding generic-mode-specific score fields.

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "scores_generic")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub score: i32,
    pub json: Json,
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
