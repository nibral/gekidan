use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.
            create_table(
                Table::create()
                    .table(Follower::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Follower::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment()
                    )
                    .col(ColumnDef::new(Follower::UserId).string().not_null())
                    .col(ColumnDef::new(Follower::Actor).string().not_null())
                    .col(ColumnDef::new(Follower::Object).string().not_null())
                    .col(ColumnDef::new(Follower::Inbox).string().not_null())
                    .col(
                        ColumnDef::new(Follower::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null()
                    )
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Follower::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Follower {
    Table,
    Id,
    UserId,
    Actor,
    Object,
    Inbox,
    CreatedAt,
}
