use std::collections::BTreeSet;

use chrono::{Duration, Utc};
use diesel::{
    migration::{Migration, MigrationSource},
    pg::Pg,
    r2d2::{ConnectionManager, ManageConnection},
    sql_query, PgConnection, RunQueryDsl,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use rfd_model::{
    storage::{
        postgres::PostgresStore, ApiKeyFilter, ApiKeyStore, ApiUserFilter, ApiUserStore,
        ListPagination,
    },
    NewApiKey, NewApiUser,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn leakable_dbs() -> Vec<String> {
    let leaks = std::env::var("LEAK_TEST_DB").unwrap_or_else(|_| String::new());
    leaks.split(',').map(|s| s.to_string()).collect()
}

#[derive(
    Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, JsonSchema, PartialOrd, Ord,
)]
enum TestPermission {
    CreateApiUser,
    CreateApiKey(Uuid),
    GetApiKey(Uuid),
    DeleteApiKey(Uuid),
}

// A fresh test database that will be created and migrated for use in a test. At the end of the
// the test (or when the database is dropped) the database will be deleted
#[derive(Debug)]
struct TestDb {
    pub db_base: String,
    pub db_name: String,
    pub should_drop: bool,
}

impl TestDb {
    pub fn new(test_name: &str) -> Self {
        let db_base = std::env::var("TEST_DATABASE").unwrap();
        let db_name = format!(
            "rfd_api_{}_{}",
            test_name,
            Uuid::new_v4().to_string().replace("-", "_")
        );

        let should_drop = !leakable_dbs().iter().any(|s| s == test_name);

        let db = Self {
            db_base,
            db_name,
            should_drop,
        };

        sql_query(&format!("CREATE DATABASE {}", db.db_name))
            .execute(&mut db.conn())
            .unwrap();

        let mut conn = db.db_conn();
        let migrations: Vec<Box<dyn Migration<Pg>>> = MIGRATIONS.migrations().unwrap();

        for migration in migrations {
            migration.run(&mut conn).unwrap();
        }

        db
    }

    pub fn url(&self) -> String {
        format!("{}/{}", self.db_base, self.db_name)
    }

    fn conn(&self) -> PgConnection {
        let conn: ConnectionManager<PgConnection> = ConnectionManager::new(&self.db_base);
        conn.connect().unwrap()
    }

    fn db_conn(&self) -> PgConnection {
        let conn: ConnectionManager<PgConnection> = ConnectionManager::new(&self.url());
        conn.connect().unwrap()
    }
}

impl Drop for TestDb {
    fn drop(&mut self) {
        if self.should_drop {
            sql_query(&format!("DROP DATABASE {}", self.db_name))
                .execute(&mut self.conn())
                .unwrap();
        }
    }
}

// Steps through the process of:
//   1. Creating an API user
//   2. Retrieve the user
//   3. Update the user's email
//   4. Update the user's permissions
//   5. Create an API token for the user
//   6. Create an API token with excess permissions for the user
//   7. Create an expired API token for the user
//   8. List the active API tokens for the user
//   9. List all API tokens for the user
//   10. Delete the API tokens for the user
//   11. List the deleted API tokens for the user
//   12. Delete the user
//   13. List the deleted user
#[tokio::test]
async fn test_api_user() {
    let db = TestDb::new("test_api_user");
    let store = PostgresStore::new(&db.url()).await.unwrap();

    let api_user_id = Uuid::new_v4();

    // 1. Create an API user
    let api_user = ApiUserStore::<TestPermission>::upsert(
        &store,
        NewApiUser {
            id: api_user_id,
            permissions: vec![TestPermission::CreateApiKey(api_user_id).into()].into(),
            groups: BTreeSet::new(),
        },
    )
    .await
    .unwrap();

    // 2. Retrieve the user
    let api_user_lookup = ApiUserStore::get(&store, &api_user.id, false)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(api_user, api_user_lookup);

    // 3. Update the user's email
    let api_user = ApiUserStore::<TestPermission>::upsert(
        &store,
        NewApiUser {
            id: api_user_id,
            permissions: vec![TestPermission::CreateApiKey(api_user_id).into()].into(),
            groups: BTreeSet::new(),
        },
    )
    .await
    .unwrap();

    assert_eq!(api_user_id, api_user.id);

    // 4. Update the user's permissions
    let api_user = ApiUserStore::<TestPermission>::upsert(
        &store,
        NewApiUser {
            id: api_user_id,
            permissions: vec![
                TestPermission::CreateApiKey(api_user_id).into(),
                TestPermission::GetApiKey(api_user_id).into(),
                TestPermission::DeleteApiKey(api_user_id).into(),
            ]
            .into(),
            groups: BTreeSet::new(),
        },
    )
    .await
    .unwrap();

    assert!(api_user
        .permissions
        .can(&TestPermission::GetApiKey(api_user_id).into()));
    assert!(api_user
        .permissions
        .can(&TestPermission::DeleteApiKey(api_user_id).into()));

    // 5. Create an API token for the user
    let token = ApiKeyStore::upsert(
        &store,
        NewApiKey {
            id: Uuid::new_v4(),
            api_user_id: api_user.id,
            key_signature: format!("key-{}", Uuid::new_v4()),
            permissions: vec![TestPermission::GetApiKey(api_user_id).into()].into(),
            expires_at: Utc::now() + Duration::seconds(5 * 60),
        },
        &api_user,
    )
    .await
    .unwrap();

    // 6. Create an API token with excess permissions for the user
    let excess_token = ApiKeyStore::upsert(
        &store,
        NewApiKey {
            id: Uuid::new_v4(),
            api_user_id: api_user.id,
            key_signature: format!("key-{}", Uuid::new_v4()),
            permissions: vec![
                TestPermission::CreateApiUser.into(),
                TestPermission::GetApiKey(api_user_id).into(),
            ]
            .into(),
            expires_at: Utc::now() + Duration::seconds(5 * 60),
        },
        &api_user,
    )
    .await
    .unwrap();

    assert!(!excess_token
        .permissions
        .can(&TestPermission::CreateApiUser.into()));

    // 7. Create an API token with excess permissions for the user
    let expired_token = ApiKeyStore::upsert(
        &store,
        NewApiKey {
            id: Uuid::new_v4(),
            api_user_id: api_user.id,
            key_signature: format!("key-{}", Uuid::new_v4()),
            permissions: vec![
                TestPermission::CreateApiUser.into(),
                TestPermission::GetApiKey(api_user_id).into(),
            ]
            .into(),
            expires_at: Utc::now() - Duration::seconds(5 * 60),
        },
        &api_user,
    )
    .await
    .unwrap();

    assert!(expired_token.expires_at < Utc::now());

    // 8. List the active API tokens for the user
    let tokens = ApiKeyStore::list(
        &store,
        ApiKeyFilter {
            id: None,
            api_user_id: Some(vec![api_user.id]),
            key_signature: None,
            expired: false,
            deleted: false,
        },
        &ListPagination::default(),
    )
    .await
    .unwrap();

    assert_eq!(tokens.len(), 2);
    assert!(tokens.contains(&token));
    assert!(tokens.contains(&excess_token));

    // 9. List all API tokens for the user
    let all_tokens = ApiKeyStore::list(
        &store,
        ApiKeyFilter {
            id: None,
            api_user_id: Some(vec![api_user.id]),
            key_signature: None,
            expired: true,
            deleted: false,
        },
        &ListPagination::default(),
    )
    .await
    .unwrap();

    assert!(all_tokens.len() == 3);
    assert!(all_tokens.contains(&expired_token));

    // 10. Lookup an API token for the user
    let token_lookup = ApiKeyStore::<TestPermission>::get(&store, &token.id, false)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(token.id, token_lookup.id);

    // 11. Delete the API tokens for the user
    for token in all_tokens {
        let _ = ApiKeyStore::<TestPermission>::delete(&store, &token.id)
            .await
            .unwrap();
    }

    // 12. List the deleted API tokens for the user
    let deleted_tokens = ApiKeyStore::<TestPermission>::list(
        &store,
        ApiKeyFilter {
            id: None,
            api_user_id: Some(vec![api_user.id]),
            key_signature: None,
            expired: true,
            deleted: true,
        },
        &ListPagination::default(),
    )
    .await
    .unwrap();

    assert!(deleted_tokens.len() == 3);

    for token in deleted_tokens {
        assert!(token.deleted_at.is_some());
        assert!(token.deleted_at.unwrap() < Utc::now());
    }

    // 13. Delete the user
    let api_user = ApiUserStore::<TestPermission>::delete(&store, &api_user.id)
        .await
        .unwrap()
        .unwrap();

    assert!(api_user.deleted_at.is_some());
    assert!(api_user.deleted_at.unwrap() < Utc::now());
    println!("Created api user {:#?}", api_user);

    // 14. List the deleted user
    let all_api_users = ApiUserStore::<TestPermission>::list(
        &store,
        ApiUserFilter {
            id: None,
            email: None,
            groups: None,
            deleted: true,
        },
        &ListPagination::default(),
    )
    .await
    .unwrap();

    assert!(all_api_users.len() == 1);

    for user in all_api_users {
        assert!(user.deleted_at.is_some());
        assert!(user.deleted_at.unwrap() < Utc::now());
    }
}

// ...
#[tokio::test]
async fn test_device_token() {}
