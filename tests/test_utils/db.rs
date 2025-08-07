#![allow(dead_code)]

use std::time::Duration;

use super::prelude::*;

pub async fn db_1<E1>(e1: E1) -> Result<DatabaseConnection, Box<dyn Error + Send + Sync>>
where
    E1: EntityTrait,
{
    let db = db().await?;
    create_table(&db, e1).await?;
    Ok(db)
}

pub async fn db_2<E1, E2>(
    e1: E1,
    e2: E2,
) -> Result<DatabaseConnection, Box<dyn Error + Send + Sync>>
where
    E1: EntityTrait,
    E2: EntityTrait,
{
    let db = db().await?;
    create_table(&db, e1).await?;
    create_table(&db, e2).await?;
    Ok(db)
}

pub async fn db_3<E1, E2, E3>(
    e1: E1,
    e2: E2,
    e3: E3,
) -> Result<DatabaseConnection, Box<dyn Error + Send + Sync>>
where
    E1: EntityTrait,
    E2: EntityTrait,
    E3: EntityTrait,
{
    let db = db().await?;
    create_table(&db, e1).await?;
    create_table(&db, e2).await?;
    create_table(&db, e3).await?;
    Ok(db)
}

async fn create_table<E>(db: &DatabaseConnection, e: E) -> Result<(), Box<dyn Error + Send + Sync>>
where
    E: EntityTrait,
{
    let backend = db.get_database_backend();
    let schema = DbSchema::new(backend);
    let stmt = schema.create_table_from_entity(e);
    db.execute(backend.build(&stmt)).await?;
    Ok(())
}

async fn db() -> Result<DatabaseConnection, Box<dyn Error + Send + Sync>> {
    #[cfg(feature = "postgres")]
    {
        let c = conn("postgres://postgres:test_pwd@localhost:5432/test_db");
        let db = Database::connect(c).await?;
        let stmts = vec![
            // drop recreate new db on each test
            "DROP SCHEMA public CASCADE;",
            "CREATE SCHEMA public;",
        ];
        for stmt in stmts.iter() {
            let stmt = Statement::from_string(DbBackend::Postgres, stmt.to_owned());
            db.execute(stmt).await?;
        }
        Ok(db)
    }
    #[cfg(feature = "mysql")]
    {
        let c = conn("mysql://root:test_pwd@localhost:3306/test_db");
        let db = Database::connect(c).await?;
        let stmts = vec![
            // drop recreate new db on each test
            "DROP DATABASE IF EXISTS test_db2;",
            "CREATE DATABASE test_db2;",
        ];
        for stmt in stmts.iter() {
            let stmt = Statement::from_string(DbBackend::Postgres, stmt.to_owned());
            db.execute(stmt).await?;
        }
        let c = conn("mysql://root:test_pwd@localhost:3306/test_db2");
        let db = Database::connect(c).await?;
        Ok(db)
    }
    #[cfg(feature = "sqlite")]
    {
        let c = conn("sqlite::memory:");
        let db = Database::connect(c).await?;
        Ok(db)
    }
    #[cfg(not(any(feature = "postgres", feature = "mysql", feature = "sqlite")))]
    {
        misuse!("must enable one of: postgres, mysql, sqlite")
    }
}

fn conn(c: &str) -> ConnectOptions {
    let mut c = ConnectOptions::new(c);
    let d = Duration::from_secs(1);
    c.connect_timeout(d);
    c.acquire_timeout(d);
    c
}
