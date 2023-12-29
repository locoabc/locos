use myapp2::app::App;
use loco_rs::testing;

use loco_rs::boot::run_task;
use serial_test::serial;
use std::collections::BTreeMap;

#[tokio::test]
#[serial]
async fn test_can_run_sync2() {
    let boot = testing::boot_test::<App>().await.unwrap();

    let vars = BTreeMap::new();

    assert!(
        run_task::<App>(&boot.app_context, Some(&"sync2".to_string()), &vars)
            .await
            .is_ok()
    );
}
