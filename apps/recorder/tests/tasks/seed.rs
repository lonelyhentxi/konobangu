//! This task implements data seeding functionality for initializing new
//! development/demo environments.
//!
//! # Example
//!
//! Run the task with the following command:
//! ```sh
//! cargo run task
//! ```
//!
//! To override existing data and reset the data structure, use the following
//! command with the `refresh:true` argument:
//! ```sh
//! cargo run task seed_data refresh:true
//! ```
#![allow(unused_imports)]
use loco_rs::{db, prelude::*};
use recorder::{app::App, migrations::Migrator};

#[allow(clippy::module_name_repetitions)]
pub struct SeedData;
#[async_trait]
impl Task for SeedData {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "seed_data".to_string(),
            detail: "Task for seeding data".to_string(),
        }
    }

    async fn run(&self, _app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        // let refresh = vars.cli.get("refresh").is_some_and(|refresh| refresh ==
        // "true");
        //
        // if refresh {
        //     db::reset::<Migrator>(&app_context.db).await?;
        // }
        // let path = std::path::Path::new("src/fixtures");
        // db::run_app_seed::<App>(&app_context.db, path).await?;
        Ok(())
    }
}