use std::collections::BTreeMap;

use loco_rs::prelude::*;

pub struct Sync1;
#[async_trait]
impl Task for Sync1 {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "sync1".to_string(),
            detail: "Task generator".to_string(),
        }
    }
    async fn run(&self, _app_context: &AppContext, _vars: &BTreeMap<String, String>) -> Result<()> {
        println!("Task Sync1 generated");
        Ok(())
    }
}
