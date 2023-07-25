use sea_orm_migration::prelude::*;
use crate::m20220713_000001_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserRsaKey::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserRsaKey::UserId)
                            .string()
                            .not_null()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(UserRsaKey::PrivateKey)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(UserRsaKey::PublicKey)
                            .string()
                            .not_null()
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-rsa-key-user_id")
                            .from(UserRsaKey::Table, UserRsaKey::UserId)
                            .to(User::Table, User::Id)
                    )
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserRsaKey::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum UserRsaKey {
    Table,
    UserId,
    PrivateKey,
    PublicKey,
}
