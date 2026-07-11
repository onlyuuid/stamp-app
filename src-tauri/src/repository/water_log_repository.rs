use sea_orm::{
    ActiveModelTrait, ColumnTrait, 
    DatabaseConnection, DbErr, 
    DeleteResult, EntityTrait, 
    QuerySelect,
    QueryOrder,
    QueryFilter,
};

use crate::entity::{water_log,day_query};
use chrono::NaiveDate;

/**
 * 查询水滴日志列表
 */
pub async fn list_water_log(
    db:&DatabaseConnection
)->Result<Vec<water_log::Model>,DbErr>{

    water_log::Entity::find()
        .all(db)
        .await
}

/**
 * 根据id查询水滴日志
 */
pub async fn find_by_id(
    db: &DatabaseConnection,
    water_id: i32,
)->Result<water_log::Model,DbErr>{

     let model =
        water_log::Entity::find_by_id(water_id)
            .one(db)
            .await?
            .ok_or(
                DbErr::RecordNotFound(
                    "task not found".to_string()
                )
            )?;
    Ok(model)
}

/**
 * 根据任务id查询水滴日志
 */
pub async fn find_by_task_id(
    db: &DatabaseConnection,
    task_id: i32,
)->Result<Vec<water_log::Model>,DbErr>{

     let model =
        water_log::Entity::find()
            .filter(water_log::Column::TaskId.eq(task_id))
            .all(db)
            .await?;

    Ok(model)
}



/**
 * 根据日期日志
 */
pub async fn find_by_date(
    db: &DatabaseConnection,
    date: NaiveDate,
)->Result<Vec<water_log::Model>,DbErr>{

     let model =
        water_log::Entity::find()
            .filter(water_log::Column::CreatedDate.eq(date))
            .all(db)
            .await?;

    Ok(model)
}

/**
 * 新建水滴日志
 */
pub async fn insert(
    db: &DatabaseConnection,
    model:water_log::ActiveModel
)-> Result<water_log::Model, DbErr> {

    model.insert(db).await

}

/**
 * 更新水滴日志
 */
pub async fn update(
    db: &DatabaseConnection,
    model:water_log::ActiveModel
)-> Result<water_log::Model, DbErr>{

    model.update(db).await
}

/**
 * 删除水滴日志
 */
pub async fn delete(
    db: &DatabaseConnection,
    task_id: i32,
)-> Result<DeleteResult, DbErr>{

    water_log::Entity::delete_many()
        .filter(water_log::Column::TaskId.eq(task_id))
        .exec(db)
        .await
}

// 数据统计

/**
 * 查询本周总投入
 */
pub async fn week_total(
    db: &DatabaseConnection,
    start: &str,
    end: &str,
) -> Result<i64, DbErr> {

    let total = water_log::Entity::find()
        .select_only()
        .column_as(
            water_log::Column::WaterNum.sum(),
            "total",
        )
        .filter(water_log::Column::CreatedDate.between(start, end))
        .into_tuple::<Option<i64>>()
        .one(db)
        .await?;

    Ok(total.flatten().unwrap_or(0))
}


/**
 * 连续坚持天数
 */
pub async fn get_record_dates(
    db: &DatabaseConnection,
) -> Result<Vec<String>, DbErr> {
    water_log::Entity::find()
        .select_only()
        .column(water_log::Column::CreatedDate)
        .group_by(water_log::Column::CreatedDate)
        .order_by_desc(water_log::Column::CreatedDate)
        .into_tuple::<String>()
        .all(db)
        .await
}

/**
 * 查询指定年份的年度热力图数据（自动补全全年 365 天）
 */
pub async fn get_annual_heatmap_stats(
    db: &DatabaseConnection,
    year: i32,
) -> Result<Vec<day_query::Model>, DbErr> {
    // 1. 计算这一年的开始和结束日期
    let start_date = NaiveDate::from_ymd_opt(year, 1, 1)
        .ok_or_else(|| DbErr::Custom("无效的开始年份".to_string()))?;
    let end_date = NaiveDate::from_ymd_opt(year, 12, 31)
        .ok_or_else(|| DbErr::Custom("无效的结束年份".to_string()))?;

    // 2. 分组聚合查询：按天统计 water_num 的总和
    // 使用元组 (NaiveDate, Option<i32>) 直接接收结果，保持代码极简
    let logs = water_log::Entity::find()
        .filter(water_log::Column::CreatedDate.between(start_date, end_date))
        .select_only()
        .column(water_log::Column::CreatedDate)
        .column_as(water_log::Column::WaterNum.sum(), "total_water")
        .group_by(water_log::Column::CreatedDate)
        .into_tuple::<(NaiveDate, Option<i32>)>()
        .all(db)
        .await?;

    // 3. 转换为 HashMap 方便后续 O(1) 检索
    let db_data_map: std::collections::HashMap<NaiveDate, i32> = logs
        .into_iter()
        .map(|(date, total)| (date, total.unwrap_or(0)))
        .collect();

    // 4. 内存补全全年日期，确保前端热力图格子不缺失
    let mut result = Vec::new();
    let mut current_date = start_date;

    while current_date <= end_date {
        let total_water = *db_data_map.get(&current_date).unwrap_or(&0);

        result.push(day_query::Model {
            date: current_date.format("%Y-%m-%d").to_string(),
            total_water,
        });

        if let Some(next_date) = current_date.succ_opt() {
            current_date = next_date;
        } else {
            break;
        }
    }

    Ok(result)
}


