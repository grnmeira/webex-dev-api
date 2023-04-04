use clap::Parser;
use serde::Deserialize;
use webex::{self};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 't', help = "Bearer token")]
    bearer_token: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Device {
    #[serde(rename="webSocketUrl")]
    websocket_url: String,
}

#[derive(Deserialize, Debug)]
struct Devices {
    devices: Vec<Device>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let webex_client = webex::api::Client::new(args.bearer_token);
    let devices = webex_client.get_devices().await.expect("Error requesting devices");

    println!("{:#?}", devices);
    // let client = reqwest::Client::new();
    // let response = client.get("https://wdm-a.wbx2.com/wdm/api/v1/devices")
    //     .bearer_auth(args.bearer_token)
    //     .send()
    //     .await.expect("Error while getting devices");

    // println!("{:#?}", response);

    // let devices_response = response.json::<Devices>().await.expect("Error parsing devices");

    // println!("{:#?}", devices_response);
}