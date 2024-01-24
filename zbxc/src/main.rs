use reqwest::blocking::Client;
use zabbix_api::client::v6::ZabbixApiV6Client;
use zabbix_api::client::ZabbixApiClient;
use log::error;

fn main() {
  let http_client = Client::new();

  let client = ZabbixApiV6Client::new(http_client, "https://ilclnagios03.mot.com/zabbix/api_jsonrpc.php");
    
  match client.get_auth_session("Admin", "zabbix") {
    Ok(session) => println!("session: {session}"),
    Err(e) => {
        error!("error: {}", e);
        panic!("unexpected error")
    }
  }
}