use sea_orm::{ConnectionTrait,Database, DatabaseConnection,ConnectOptions, DbErr,Schema};
use tauri::{AppHandle,Manager};
use crate::entity::{task,water_log};


pub async fn init_db(
    app: &AppHandle
) -> Result<DatabaseConnection, DbErr> {

    let app_dir = app
        .path()
        .app_data_dir()
        .expect("无法获取应用数据目录");


    // 确保目录存在
    std::fs::create_dir_all(&app_dir)
        .expect("创建数据库目录失败");


    let db_path = app_dir.join("point.db");


    let db_url = format!(
        "sqlite://{}?mode=rwc",
        db_path.to_string_lossy()
    );


    println!("database path = {}", db_url);

    // 配置连接池
    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(10)
       .min_connections(1)
       .sqlx_logging(true);


    // 1. 先正常连接数据库
    let db = Database::connect(opt).await?;

    // 2. 自动创建所有数据表（核心部分）
    create_all_tables(&db).await?;

    println!("✅ 数据库初始化完成，所有表已就绪");

    Ok(db)

}


/// 创建所有数据表
async fn create_all_tables(db: &DatabaseConnection) -> Result<(), DbErr> {
    let db_backend = db.get_database_backend();
    let schema = Schema::new(db_backend);

    // 逐个创建表，代码清晰且性能更好
    create_table(&schema, db, task::Entity).await?;
    create_table(&schema, db, water_log::Entity).await?;
    // 后续新增表只需在这里加一行即可
   

    Ok(())
}

/// 抽成小函数，避免重复代码
async fn create_table<E: sea_orm::EntityTrait>(
    schema: &Schema,
    db: &DatabaseConnection,
    entity: E,
) -> Result<(), DbErr> {
    let db_backend = db.get_database_backend();

    let create_stmt = db_backend.build(
        schema.create_table_from_entity(entity).if_not_exists()
    );

    db.execute(create_stmt).await.map_err(|e| {
        DbErr::Custom(format!(
            "创建表失败 [{}]: {}",
            std::any::type_name::<E>(),
            e
        ))
    })?;

    Ok(())
}