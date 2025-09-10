#![allow(dead_code)]

use super::*;
use std::{ops::Deref, time::Duration};

pub async fn db_1<E1>(e1: E1) -> Result<TmpDb, Box<dyn Error + Send + Sync>>
where
    E1: EntityX,
{
    let db = db().await?;
    create_table(&db, e1).await?;
    Ok(db)
}

pub async fn db_2<E1, E2>(e1: E1, e2: E2) -> Result<TmpDb, Box<dyn Error + Send + Sync>>
where
    E1: EntityX,
    E2: EntityX,
{
    let db = db().await?;
    create_table(&db, e1).await?;
    create_table(&db, e2).await?;
    Ok(db)
}

pub async fn db_3<E1, E2, E3>(e1: E1, e2: E2, e3: E3) -> Result<TmpDb, Box<dyn Error + Send + Sync>>
where
    E1: EntityX,
    E2: EntityX,
    E3: EntityX,
{
    let db = db().await?;
    create_table(&db, e1).await?;
    create_table(&db, e2).await?;
    create_table(&db, e3).await?;
    Ok(db)
}

async fn db() -> Result<TmpDb, Box<dyn Error + Send + Sync>> {
    #[cfg(feature = "postgres")]
    {
        let uri = "postgres://postgres:test_pwd@localhost:5432/test_db";
        let db = TmpDb::new(uri).await?;
        Ok(db)
    }
    #[cfg(feature = "mysql")]
    {
        let uri = "mysql://root:test_pwd@localhost:3306/test_db";
        let db = TmpDb::new(uri).await?;
        Ok(db)
    }
    #[cfg(feature = "sqlite")]
    {
        let uri = "sqlite::memory:";
        let db = TmpDb::new(uri).await?;
        Ok(db)
    }
    #[cfg(not(any(feature = "postgres", feature = "mysql", feature = "sqlite")))]
    {
        misuse!("must enable one of: postgres, mysql, sqlite")
    }
}

// ============================================================================
// create temporary db and automatically clean up on drop

#[derive(Clone)]
pub struct TmpDb {
    name: String,
    db: DatabaseConnection,
    ty: TmpDbType,
}

#[derive(Clone)]
enum TmpDbType {
    Postgres,
    MySql { admin: DatabaseConnection },
    Sqlite,
}

impl TmpDb {
    pub async fn new(uri: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        match get_uri_scheme(uri).as_str() {
            "postgres" => Self::new_postgres(uri).await,
            "mysql" => Self::new_mysql(uri).await,
            "sqlite" => Self::new_sqlite(uri).await,
            scheme => {
                let err = f!(
                    "TmpDb::new expect postgres or mysql or sqlite, found {}",
                    scheme
                );
                bug!(err)
            }
        }
    }

    async fn new_postgres(uri: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let name = new_db_name();
        let db = conn(uri).await?;

        let stmt = f!("CREATE SCHEMA {};", name);
        exec(&db, DbBackend::Postgres, &stmt).await?;

        let stmt = f!("SET search_path TO {};", name);
        exec(&db, DbBackend::Postgres, &stmt).await?;

        Ok(Self {
            name,
            db,
            ty: TmpDbType::Postgres,
        })
    }

    async fn new_mysql(uri: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let name = new_db_name();
        let admin = conn(uri).await?;

        let stmt = f!("CREATE DATABASE {};", name);
        exec(&admin, DbBackend::MySql, &stmt).await?;

        let uri = replace_db_name(uri, &name);
        let db = conn(&uri).await?;

        Ok(Self {
            name,
            db,
            ty: TmpDbType::MySql { admin },
        })
    }

    async fn new_sqlite(_: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let name = "memory".to_string();
        let db = conn("sqlite::memory:").await?;

        Ok(Self {
            name,
            db,
            ty: TmpDbType::Sqlite,
        })
    }
}

impl Deref for TmpDb {
    type Target = DatabaseConnection;
    fn deref(&self) -> &Self::Target {
        &self.db
    }
}

impl AsRef<DatabaseConnection> for TmpDb {
    fn as_ref(&self) -> &DatabaseConnection {
        &self.db
    }
}

impl Drop for TmpDb {
    fn drop(&mut self) {
        let name = self.name.clone();
        let db = self.db.clone();
        let ty = self.ty.clone();

        let future = async move {
            let _ = match ty.clone() {
                TmpDbType::Postgres => cleanup_postgres(&name, &db).await,
                TmpDbType::MySql { admin } => cleanup_mysql(&name, &db, admin).await,
                TmpDbType::Sqlite => cleanup_sqlite(&db).await,
            };
        };

        if let Ok(handle) = tokio::runtime::Handle::try_current() {
            handle.spawn(future);
        } else {
            std::thread::spawn(|| {
                if let Ok(runtime) = tokio::runtime::Runtime::new() {
                    runtime.block_on(future);
                }
            });
        }
    }
}

// ============================================================================
// helpers

fn new_db_name() -> String {
    f!("test_{}", ulid())
}

fn get_uri_scheme(uri: &str) -> String {
    uri.split(':').next().unwrap_or_default().to_string()
}

fn replace_db_name(uri: &str, name: &str) -> String {
    if let Some((head, _)) = uri.rsplit_once('/') {
        f!("{}/{}", head, name)
    } else {
        f!("{}/{}", uri.trim_end_matches('/'), name)
    }
}

async fn conn(uri: &str) -> Result<DatabaseConnection, Box<dyn Error + Send + Sync>> {
    let mut c = ConnectOptions::new(uri);
    let d = Duration::from_secs(1);
    c.connect_timeout(d);
    c.acquire_timeout(d);

    // prevent search_path override in migration
    #[cfg(feature = "postgres")]
    c.max_connections(1);

    let db = Database::connect(c).await?;

    Ok(db)
}

async fn exec(
    db: &DatabaseConnection,
    be: DbBackend,
    stmt: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let stmt = Statement::from_string(be, stmt);
    db.execute(stmt).await?;

    Ok(())
}

async fn create_table<E>(db: &DatabaseConnection, e: E) -> Result<(), Box<dyn Error + Send + Sync>>
where
    E: EntityX,
{
    let backend = db.get_database_backend();
    let schema = DbSchema::new(backend);
    let stmt = schema.create_table_from_entity(e);
    db.execute(backend.build(&stmt)).await?;

    Ok(())
}

async fn cleanup_postgres(
    name: &str,
    db: &DatabaseConnection,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let stmt = "SET search_path TO public;";
    exec(db, DbBackend::Postgres, &stmt).await?;

    let stmt = f!("DROP SCHEMA IF EXISTS {} CASCADE;", name);
    exec(db, DbBackend::Postgres, &stmt).await?;

    db.clone().close().await?;

    Ok(())
}

async fn cleanup_mysql(
    name: &str,
    db: &DatabaseConnection,
    admin: DatabaseConnection,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    db.clone().close().await?;

    let stmt = format!("DROP DATABASE IF EXISTS {};", name);
    exec(&admin, DbBackend::MySql, &stmt).await?;

    Ok(())
}

async fn cleanup_sqlite(db: &DatabaseConnection) -> Result<(), Box<dyn Error + Send + Sync>> {
    db.clone().close().await?;

    Ok(())
}
