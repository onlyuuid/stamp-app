
use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel,Serialize, Deserialize, Eq)]
#[sea_orm(table_name = "water_log")]
pub struct Model{

    #[sea_orm(primary_key)]
    pub log_id: i32,
    pub task_id: i32,
    pub water_num: i32,
    pub created_date: NaiveDate

}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}