// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use diesel::{
    r2d2::{ConnectionManager, ManageConnection},
    PgConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../rfd-model/migrations");

pub fn run_migrations(url: &str, v_only: bool) {
    // These are safe to run multiple times.
    v_api_installer::run_migrations(url);

    if !v_only {
        let mut conn = db_conn(url);
        conn.run_pending_migrations(MIGRATIONS).unwrap();
    }
}

fn db_conn(url: &str) -> PgConnection {
    let conn: ConnectionManager<PgConnection> = ConnectionManager::new(url);
    conn.connect().unwrap()
}
