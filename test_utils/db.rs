use crate::prelude::*;
use std::time::Duration;

pub async fn tmp_db() -> Res<TmpDb> {
    #[cfg(feature = "postgres")]
    {
        let uri = "postgres://postgres:test_pwd@localhost:5432/test_db";
        let tmp = TmpDb::new(uri).await?;
        Ok(tmp)
    }
    #[cfg(feature = "mysql")]
    {
        let uri = "mysql://root:test_pwd@localhost:3306/test_db";
        let tmp = TmpDb::new(uri).await?;
        Ok(tmp)
    }
    #[cfg(feature = "sqlite")]
    {
        let uri = "sqlite::memory:";
        let tmp = TmpDb::new(uri).await?;
        Ok(tmp)
    }
    #[cfg(not(any(feature = "postgres", feature = "mysql", feature = "sqlite")))]
    {
        misuse!("must enable one of: postgres, mysql, sqlite")
    }
}

// ============================================================================
// create temporary db and automatically clean up on drop

pub struct TmpDb {
    ty: TmpDbType,
    name: String,
    pub db: DatabaseConnection,
}

enum TmpDbType {
    Postgres,
    MySql { admin: DatabaseConnection },
    Sqlite,
}

impl TmpDb {
    pub async fn new(uri: &str) -> Res<Self> {
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

    async fn new_postgres(uri: &str) -> Res<Self> {
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

    async fn new_mysql(uri: &str) -> Res<Self> {
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

    async fn new_sqlite(_: &str) -> Res<Self> {
        let name = "memory".to_string();
        let db = conn("sqlite::memory:").await?;

        Ok(Self {
            name,
            db,
            ty: TmpDbType::Sqlite,
        })
    }

    pub async fn drop(&self) -> Res<()> {
        Ok(match &self.ty {
            TmpDbType::Postgres => {
                let stmt = "SET search_path TO public;";
                exec(&self.db, DbBackend::Postgres, &stmt).await?;
                let stmt = f!("DROP SCHEMA IF EXISTS {} CASCADE;", self.name);
                exec(&self.db, DbBackend::Postgres, &stmt).await?;
                self.db.clone().close().await?;
            }
            TmpDbType::MySql { admin } => {
                self.db.clone().close().await?;
                let stmt = format!("DROP DATABASE IF EXISTS {};", self.name);
                exec(admin, DbBackend::MySql, &stmt).await?;
            }
            TmpDbType::Sqlite => {
                self.db.clone().close().await?;
            }
        })
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

async fn conn(uri: &str) -> Res<DatabaseConnection> {
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

async fn exec(db: &DatabaseConnection, be: DbBackend, stmt: &str) -> Res<()> {
    let stmt = Statement::from_string(be, stmt);
    db.execute(stmt).await?;

    Ok(())
}

pub async fn create_table<E>(db: &DatabaseConnection, e: E) -> Res<()>
where
    E: EntityX,
{
    let backend = db.get_database_backend();
    let schema = DbSchema::new(backend);
    let stmt = schema.create_table_from_entity(e);
    db.execute(backend.build(&stmt)).await?;

    Ok(())
}
