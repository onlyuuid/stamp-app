use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

// 创建task表
#[derive(Iden)]
enum Task {
    Table,
    TaskId,
    TaskName,
    Description,
    Status,
    AddUpNum,
    DayUpNum,
    AddDayNum,
    CreateDate,
    LastRecordDate,
}


#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Task::Table)
                    .if_not_exists()
                    .col(
                         ColumnDef::new(Task::TaskId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                         ColumnDef::new(Task::TaskName)
                            .string()
                            .not_null(),
                    )
                    .col(
                         ColumnDef::new(Task::Description)
                            .string(),
                    )
                    .col(
                        ColumnDef::new(Task::Status)
                            .integer()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Task::AddUpNum)
                            .integer()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Task::DayUpNum)
                            .integer()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Task::AddDayNum)
                            .integer()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Task::CreateDate)
                            .string(),
                    )
                    .col(
                        ColumnDef::new(Task::LastRecordDate)
                            .string(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(
                Table::drop()
                    .table(Task::Table)
                    .to_owned(),
            )
            .await
    }
}
