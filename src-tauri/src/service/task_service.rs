use sea_orm::{DatabaseConnection, DbErr, Set};

use crate::entity::task;
use crate::{
    dto::task_dto::TaskDTO,
    entity::weekly_query::TaskWeeklyData,
    repository::task_repository,
};

/** 
 * 查询任务列表
 */
pub async fn list_task(
    db: &DatabaseConnection,
) -> Result<Vec<TaskDTO>, sea_orm::DbErr> {

    let list  =  task_repository::list_task(db).await?;

    Ok(
        list.into_iter()
            .map(TaskDTO::from)
            .collect()
    )
}

/**
 * 根据id查询任务
 */
pub async fn find_by_id(
    db: &DatabaseConnection,
    task_id: i32,
)->Result<TaskDTO,DbErr>{

    let model = task_repository::find_by_id(db,task_id).await?;

    Ok(TaskDTO::from(model))
}

/**
 * 添加任务列表
 */
pub async fn create_task(
    db:&DatabaseConnection,
    dto:TaskDTO
)->Result<TaskDTO,DbErr>{

    let active_model = task::ActiveModel{
        task_name:Set(dto.task_name),
        description:Set(dto.description),
        status:Set(Some(0)),
        add_up_num:Set(Some(0)),
        day_up_num:Set(Some(0)),
        add_day_num:Set(Some(0)),
        create_date:Set(dto.create_date),
        last_record_date:Set(dto.last_record_date),
        ..Default::default()
    };

    let model =
        task_repository::insert(
            db,
            active_model
        )
        .await?;
    Ok(TaskDTO::from(model))
}


/**
 * 更新任务列表
 */
pub async fn update_task(
    db:&DatabaseConnection,
    dto:TaskDTO
)->Result<TaskDTO,DbErr>{

    let active_model = task::ActiveModel{
        task_id:Set(dto.task_id),
        task_name:Set(dto.task_name),
        description:Set(dto.description),
        status:Set(dto.status),
        add_up_num:Set(dto.add_up_num),
        day_up_num:Set(dto.day_up_num),
        add_day_num:Set(dto.add_day_num),
        last_record_date:Set(dto.last_record_date),
        ..Default::default()
    };

    let model =
        task_repository::update(
            db,
            active_model
        )
        .await?;
    Ok(TaskDTO::from(model))
}

/**
 * 删除任务
 */
pub async fn delete_task(
    db:&DatabaseConnection,
    task_id:i32
)->Result<u64,DbErr>{

    let result =
        task_repository::delete(
            db,
            task_id
        )
        .await?;

    Ok(result.rows_affected)
}


use chrono::{Datelike, Duration, Local, NaiveDate};
use std::collections::HashMap;

use crate::repository::task_repository::TaskRepository;

pub struct TaskService;

impl TaskService {
    /// 计算给定日期所在周的 [周一, 周日]
    fn week_range(date: NaiveDate) -> (NaiveDate, NaiveDate) {
        let days_since_monday = date.weekday().num_days_from_monday() as i64;
        let monday = date - Duration::days(days_since_monday);
        let sunday = monday + Duration::days(6);
        (monday, sunday)
    }

    // 按周统计每个 task 的 water_num
    pub async fn get_weekly_stats(
        db: &DatabaseConnection,
        date: Option<NaiveDate>,
    ) -> Result<Vec<TaskWeeklyData>, String> {
        // 1. 确定周范围
        let target_date = date.unwrap_or_else(|| Local::now().date_naive());
        let (monday, sunday) = Self::week_range(target_date);

        // 2. 查 task 列表
        let tasks = TaskRepository::find_all_tasks(db)
            .await
            .map_err(|e| format!("查询 task 失败: {}", e))?;

        // 3. 查本周 water_log
        let logs = TaskRepository::find_water_logs_by_date_range(db, monday, sunday)
            .await
            .map_err(|e| format!("查询 water_log 失败: {}", e))?;

        // 4. 内存聚合：task_id → [Mon..Sun]
        let mut agg: HashMap<i32, [i32; 7]> = HashMap::new();
        for log in &logs {
            let idx = log.created_date.weekday().num_days_from_monday() as usize;
            *agg.entry(log.task_id).or_insert([0; 7])
                .get_mut(idx)
                .unwrap() += log.water_num;
        }

        // 5. 组装结果
        let result = tasks
            .into_iter()
            .map(|t| TaskWeeklyData {
                task_id: t.task_id,
                task_name: t.task_name,
                daily_water: *agg.get(&t.task_id).unwrap_or(&[0; 7]),
            })
            .collect();

        Ok(result)
    }
}