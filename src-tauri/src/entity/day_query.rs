use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub date: String,    // 格式化为 "2026-07-11"
    pub total_water: i32, // 当天所有任务加水的总和
}