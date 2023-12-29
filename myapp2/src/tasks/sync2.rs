use std::collections::BTreeMap;

use loco_rs::prelude::*;

pub struct Sync2;
#[async_trait]
impl Task for Sync2 {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "sync2".to_string(),
            detail: "Task generator".to_string(),
        }
    }
    async fn run(&self, _app_context: &AppContext, _vars: &BTreeMap<String, String>) -> Result<()> {
        println!("Task Sync2 generated");
        Ok(())
    }
}
