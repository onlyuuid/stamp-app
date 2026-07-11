use serde::{Serialize,Deserialize};
use crate::entity::task::Model;

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskDTO {
    pub task_id: i32,
    pub task_name: String,
    pub description: Option<String>,
    pub status: Option<i32>,
    pub add_up_num: Option<i32>,
    pub day_up_num: Option<i32>,
    pub add_day_num: Option<i32>,
    pub create_date: Option<String>,
    pub last_record_date: Option<String>
}

impl From<Model> for TaskDTO {
    fn from(model: Model) -> Self {
        Self {
            task_id: model.task_id,
            task_name: model.task_name,
            description: model.description,
            status: model.status,
            add_up_num: model.add_up_num,
            day_up_num: model.day_up_num,
            add_day_num: model.add_day_num,
            create_date: model.create_date,
            last_record_date: model.last_record_date,
        }
    }
}



