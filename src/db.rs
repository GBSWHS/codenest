use sea_orm::{ConnectionTrait, Database, DbBackend, DbErr, Statement};

const DATABASE_URL: &str = "mysql://root:password@localhost:3306";
const DB_NAME: &str = "codenest";

pub async fn create_dbconnection() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS {}", DB_NAME),
            ))
            .await?;

            println!("Database {} created", DB_NAME);

            let url = format!("{}/{}", DATABASE_URL, DB_NAME);
            Database::connect(&url).await?
        }
        sea_orm::DatabaseBackend::Postgres => todo!(),
        sea_orm::DatabaseBackend::Sqlite => todo!(),
    };

    Ok(())
}