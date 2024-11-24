use anyhow::Result;
use dotenv;

use omu::{Client, GatewayEventData};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let mut client = Client::new(dotenv::var("TOKEN")?, None);
    client.connect().await?;

    while let Ok(e) = client.next().await {
        match e {
            GatewayEventData::Ready(ready) => println!("Ready: {:#?}", ready),
            GatewayEventData::Hello(hello) => println!("Hello! {:#?}", hello),
        }
    }

    Ok(())
}
