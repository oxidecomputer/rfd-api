// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use rfd_installer::run_migrations;

fn main() {
    if let Ok(url) = std::env::var("DATABASE_URL") {
        run_migrations(&url, std::env::var("V_ONLY").is_ok());
    } else {
        println!("DATABASE_URL environment variable must be specified to run migrations and must be in the form of a connection string")
    }
}
