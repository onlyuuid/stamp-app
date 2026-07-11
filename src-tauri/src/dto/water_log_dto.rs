use serde::{Serialize,Deserialize};
use crate::entity::water_log::Model;

use chrono::NaiveDate;

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WaterLogDTO {

    pub log_id: i32,
    pub task_id: i32,
    pub water_num: i32,
    pub created_date: NaiveDate
}

impl From<Model> for WaterLogDTO {
    fn from(model: Model) -> Self {
        Self {
            log_id: model.log_id,
            task_id: model.task_id,
            water_num: model.water_num,
            created_date: model.created_date
        }
    }
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WaterCreateLogDTO {

    pub task_id: i32,
    pub water_num: i32,
    pub created_date: NaiveDate
}