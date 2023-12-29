use std::collections::BTreeMap;

use loco_rs::prelude::*;

pub struct Sync;
#[async_trait]
impl Task for Sync {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "sync".to_string(),
            detail: "Task generator".to_string(),
        }
    }
    async fn run(&self, _app_context: &AppContext, _vars: &BTreeMap<String, String>) -> Result<()> {
        println!("Task Sync generated");
        Ok(())
    }
}
