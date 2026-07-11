use serde::{ Serialize};

/// 单个 task 的周统计数据
#[derive(Serialize)]
pub struct TaskWeeklyData {
    pub task_id: i32,
    pub task_name: String,
    /// 0=周一 ... 6=周日
    pub daily_water: [i32; 7],
}