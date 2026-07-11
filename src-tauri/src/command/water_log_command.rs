use tauri::State;
use chrono::NaiveDate;
use crate::{
    service::water_log_service,
    state::AppState,
    dto::water_log_dto::WaterLogDTO,
    dto::water_log_dto::WaterCreateLogDTO
}; 
use crate::entity::{day_query};

/**
 * 查询日志列表
 */
#[tauri::command]
pub async fn list_water_log(
    state: State<'_, AppState>,
) -> Result<Vec<WaterLogDTO>, String> {
     water_log_service::list_water_log(&state.db)
        .await
        .map_err(|e| e.to_string())
}


/**
 * 根据id查询日志
 */
#[tauri::command]
pub async fn find_by_log_id(
    state: State<'_, AppState>,
    log_id: i32,
)->Result<WaterLogDTO,String>{
    let model = water_log_service::find_by_id(
        &state.db,
        log_id).await
        .map_err(|e| e.to_string())?;
    Ok(model)
}

/**
 * 根据任务id查询日志
 */
#[tauri::command]
pub async fn find_by_task_id(
    state: State<'_, AppState>,
    task_id: i32,
)->Result<Vec<WaterLogDTO>,String>{
        water_log_service::find_by_task_id(
        &state.db,
        task_id).await
        .map_err(|e| e.to_string())
   
}

/**
 * 根据日期查询日志
 */
#[tauri::command]
pub async fn find_by_date(
    state: State<'_, AppState>,
    date: NaiveDate,
)->Result<Vec<WaterLogDTO>,String>{

        water_log_service::find_by_date(&state.db,date).await
        .map_err(|e| e.to_string())
   
}


/**
 * 创建日志
 */
#[tauri::command]
pub async fn create_water_log(
    state: State<'_, AppState>,
    log: WaterCreateLogDTO,
) -> Result<WaterLogDTO, String> {
     let result =
        water_log_service::create_water_log(
            &state.db,
            log
        )
        .await
        .map_err(|e| e.to_string())?;
    Ok(result)
}

/**
 * 更新日志
 */
#[tauri::command]
pub async fn update_water_log(
    state: State<'_, AppState>,
    log: WaterLogDTO,
) -> Result<WaterLogDTO, String> {
     let result =
        water_log_service::update_water_log(
            &state.db,
            log
        )
        .await
        .map_err(|e| e.to_string())?;
    Ok(result)
}

/**
 * 删除日志
 */
#[tauri::command]
pub async fn delete_water_log(
    state:State<'_,AppState>,
    task_id:i32
)->Result<u64,String>{
    let count =
        water_log_service::delete_water_log(
            &state.db,
            task_id
        )
        .await
        .map_err(|e|e.to_string())?;
    Ok(count)
}

/**
 * 查询本周总投入
 */
#[tauri::command]
pub async fn week_total(
    state: State<'_, AppState>,
) -> Result<i64, String> {

    let total = water_log_service::week_total(
        &state.db,
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(total)
}

/**
 * 连续坚持天数
 */
#[tauri::command]
pub async fn streak_days(
    state: State<'_, AppState>,
) -> Result<i32, String> {

    let streak = water_log_service::streak_days(
        &state.db
    )
    .await.map_err(|e| e.to_string())?;

    Ok(streak)
}

/**
 * 获取年度热力图统计数据
 * year: 需要查询的年份，例如 2026
 */
#[tauri::command]
pub async fn get_heatmap(
   state: State<'_, AppState>,
   year: i32,
) -> Result<Vec<day_query::Model>, String> {
    
    // 2. 调用 service
    let data = water_log_service::get_annual_heatmap_stats(
        &state.db, year).await.map_err(|e| e.to_string())?;
    
    
    Ok(data)
}



