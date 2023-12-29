use std::collections::BTreeMap;

use loco_rs::prelude::*;

pub struct Sync2Data;
#[async_trait]
impl Task for Sync2Data {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "sync2_data".to_string(),
            detail: "Task generator sync3_data".to_string(),
        }
    }
    async fn run(&self, _app_context: &AppContext, _vars: &BTreeMap<String, String>) -> Result<()> {
        println!("Task Sync2Data generated");
        Ok(())
    }
}
