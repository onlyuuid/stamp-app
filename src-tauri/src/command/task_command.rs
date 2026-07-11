use tauri::State;

use crate::{
    service::task_service,
    state::AppState,
    dto::task_dto::TaskDTO
}; 

use chrono::NaiveDate;
use crate::entity::weekly_query::{TaskWeeklyData};
use crate::service::task_service::TaskService;

/**
 * 查询任务列表
 */
#[tauri::command]
pub async fn list_task(
    state: State<'_, AppState>,
) -> Result<Vec<TaskDTO>, String> {
     task_service::list_task(&state.db)
        .await
        .map_err(|e| e.to_string())
}


/**
 * 根据id查询任务
 */
#[tauri::command]
pub async fn find_by_id(
    state: State<'_, AppState>,
    task_id: i32,
)->Result<TaskDTO,String>{

    let model = task_service::find_by_id(
        &state.db,
        task_id).await
        .map_err(|e| e.to_string())?;

    Ok(model)
}


/**
 * 创建任务
 */
#[tauri::command]
pub async fn create_task(
    state: State<'_, AppState>,
    task: TaskDTO,
) -> Result<TaskDTO, String> {
     let result =
        task_service::create_task(
            &state.db,
            task
        )
        .await
        .map_err(|e| e.to_string())?;
    Ok(result)
}

/**
 * 更新任务
 */
#[tauri::command]
pub async fn update_task(
    state: State<'_, AppState>,
    task: TaskDTO,
) -> Result<TaskDTO, String> {
     let result =
        task_service::update_task(
            &state.db,
            task
        )
        .await
        .map_err(|e| e.to_string())?;
    Ok(result)
}

/**
 * 删除任务
 */
#[tauri::command]
pub async fn delete_task(
    state:State<'_,AppState>,
    task_id:i32
)->Result<u64,String>{

    let count =
        task_service::delete_task(
            &state.db,
            task_id
        )
        .await
        .map_err(|e|e.to_string())?;

    Ok(count)
}

/**
 * 本周任务数据统计
 */
#[tauri::command]
pub async fn weekly_stats(
    state:State<'_,AppState>,
    date: Option<NaiveDate>,
) -> Result<Vec<TaskWeeklyData>,String> {
    let data  =  TaskService::get_weekly_stats(&state.db, date).await?;

    Ok(data)
}






