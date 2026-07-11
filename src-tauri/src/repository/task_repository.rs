use sea_orm::{
    DatabaseConnection,
    DbErr,
    EntityTrait,
    ActiveModelTrait,
    DeleteResult
};
use crate::entity::task;

/**
 * 查询任务列表
 */
pub async fn list_task(
    db:&DatabaseConnection
)->Result<Vec<task::Model>,DbErr>{

    task::Entity::find()
        .all(db)
        .await
}


/**
 * 根据id查询任务
 */
pub async fn find_by_id(
    db: &DatabaseConnection,
    task_id: i32,
)->Result<task::Model,DbErr>{

     let model =
        task::Entity::find_by_id(task_id)
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
 * 新建任务
 */
pub async fn insert(
    db: &DatabaseConnection,
    model:task::ActiveModel
)-> Result<task::Model, DbErr> {

    model.insert(db).await

}

/**
 * 更新任务
 */
pub async fn update(
    db: &DatabaseConnection,
    model:task::ActiveModel
)-> Result<task::Model, DbErr>{

    model.update(db).await
}

/**
 * 删除任务
 */
pub async fn delete(
    db: &DatabaseConnection,
    task_id: i32,
)-> Result<DeleteResult, DbErr>{

    task::Entity::delete_by_id(task_id)
        .exec(db)
        .await
}

use sea_orm::*;
use chrono::NaiveDate;
use crate::entity::{water_log};

pub struct TaskRepository;

impl TaskRepository {
    /// 查询所有 task
    pub async fn find_all_tasks(db: &DatabaseConnection) -> Result<Vec<task::Model>, DbErr> {
        task::Entity::find().all(db).await
    }

    /// 查询指定日期范围内的所有 water_log
    pub async fn find_water_logs_by_date_range(
        db: &DatabaseConnection,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<Vec<water_log::Model>, DbErr> {
        water_log::Entity::find()
            .filter(water_log::Column::CreatedDate.gte(start))
            .filter(water_log::Column::CreatedDate.lte(end))
            .all(db)
            .await
    }

    // 可选：按 task_id 过滤
    // pub async fn find_water_logs_by_task_and_date_range(
    //     db: &DatabaseConnection,
    //     task_id: i32,
    //     start: NaiveDate,
    //     end: NaiveDate,
    // ) -> Result<Vec<water_log::Model>, DbErr> {
    //     water_log::Entity::find()
    //         .filter(water_log::Column::TaskId.eq(task_id))
    //         .filter(water_log::Column::CreatedDate.gte(start))
    //         .filter(water_log::Column::CreatedDate.lte(end))
    //         .all(db)
    //         .await
    // }
}