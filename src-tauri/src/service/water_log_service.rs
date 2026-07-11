use sea_orm::{DatabaseConnection, DbErr, Set};
use chrono::{Datelike, Duration, Local,NaiveDate};
use crate::{
    dto::water_log_dto::WaterLogDTO,
    dto::water_log_dto::WaterCreateLogDTO,
    repository::water_log_repository,
};
use crate::entity::{water_log,day_query};
/** 
 * 查询水滴记录日志
 */
pub async fn list_water_log(
    db: &DatabaseConnection,
) -> Result<Vec<WaterLogDTO>, sea_orm::DbErr> {
    let list: Vec<water_log::Model>  =  water_log_repository::list_water_log(db).await?;
    Ok(
        list.into_iter()
            .map(WaterLogDTO::from)
            .collect()
    )
}

/**
 * 根据id查询日志
 */
pub async fn find_by_id(
    db: &DatabaseConnection,
    log_id: i32,
)->Result<WaterLogDTO,DbErr>{
    let model = water_log_repository::find_by_id(db,log_id).await?;
    Ok(WaterLogDTO::from(model))
}

/**
 * 根据任务id查询日志
 */
pub async fn find_by_task_id(
    db: &DatabaseConnection,
    task_id: i32,
)->Result<Vec<WaterLogDTO>,DbErr>{
    let model = water_log_repository::find_by_task_id(db,task_id).await?;
    
     Ok(
        model.into_iter()
            .map(WaterLogDTO::from)
            .collect()
    )
}


/**
 * 根据日期日志
 */
pub async fn find_by_date(
    db: &DatabaseConnection,
    date: NaiveDate,
)->Result<Vec<WaterLogDTO>,DbErr>{
    let model = water_log_repository::find_by_date(db,date).await?;
    
     Ok(
        model.into_iter()
            .map(WaterLogDTO::from)
            .collect()
    )
}


/**
 * 添加日志
 */
pub async fn create_water_log(
    db:&DatabaseConnection,
    dto:WaterCreateLogDTO
)->Result<WaterLogDTO,DbErr>{

    let active_model = water_log::ActiveModel{
        task_id:Set(dto.task_id),
        water_num:Set(dto.water_num),
        created_date:Set(dto.created_date),
        ..Default::default()
    };

    let model =
        water_log_repository::insert(
            db,
            active_model
        )
        .await?;
    Ok(WaterLogDTO::from(model))
}


/**
 * 更新日志
 */
pub async fn update_water_log(
    db:&DatabaseConnection,
    dto:WaterLogDTO
)->Result<WaterLogDTO,DbErr>{

    let active_model = water_log::ActiveModel{
        log_id:Set(dto.log_id),
        task_id:Set(dto.task_id),
        water_num:Set(dto.water_num),
        created_date:Set(dto.created_date),
        ..Default::default()
    };

    let model =
        water_log_repository::update(
            db,
            active_model
        )
        .await?;
    Ok(WaterLogDTO::from(model))
}

/**
 * 删除任务
 */
pub async fn delete_water_log(
    db:&DatabaseConnection,
    task_id:i32
)->Result<u64,DbErr>{

    let result =
        water_log_repository::delete(
            db,
            task_id
        )
        .await?;

    Ok(result.rows_affected)
}


/**
 * 查询本周总投入
 */
pub async fn week_total(
    db: &DatabaseConnection,
) -> Result<i64, DbErr> {

    let today = Local::now().date_naive();

    // 本周星期一
    let monday =
        today - Duration::days(today.weekday().num_days_from_monday() as i64);

    // 本周星期日
    let sunday =
        monday + Duration::days(6);

    let start = monday.format("%Y-%m-%d").to_string();
    let end = sunday.format("%Y-%m-%d").to_string();

    water_log_repository::week_total(
        db,
        &start,
        &end,
    ).await
}

/**
 * 连续坚持天数
 */
pub async fn streak_days(
    db: &DatabaseConnection,
) -> Result<i32, DbErr> {
    let dates = water_log_repository::get_record_dates(db).await?;
    if dates.is_empty() {
        return Ok(0);
    }
    let mut streak = 0;
    // 今天开始检查
    let mut expect = Local::now().date_naive();
    // 如果今天没有记录，则从昨天开始算
    if dates[0] != expect.format("%Y-%m-%d").to_string() {
        expect -= Duration::days(1);
    }
    for date_str in dates {
        let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap();

        if date == expect {
            streak += 1;
            expect -= Duration::days(1);
        } else if date < expect {
            // 中间断了一天
            break;
        }
        // 如果 date > expect（理论上不会出现，因为已倒序），直接忽略
    }
    Ok(streak)
}

/**
 * 获取年度热力图统计数据
 * year: 需要查询的年份，例如 2026
 */
pub async fn get_annual_heatmap_stats(
    db: &DatabaseConnection,
    year: i32,
) -> Result<Vec<day_query::Model>, DbErr> {
    
    // 直接调用 repository 层的处理逻辑（它已经做好了 365 天数据的内存补全）
    let heatmap_data = water_log_repository::get_annual_heatmap_stats(db, year).await?;
    
    // 这里的业务比较纯粹，直接将结果透传给 Controller 即可
    Ok(heatmap_data)
}