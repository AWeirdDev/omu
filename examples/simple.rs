use anyhow::Result;
use dotenv;

use omu::{slash, Client, GatewayEventData};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    println!("{:?}", ping().await);

    let start = std::time::Instant::now();

    let mut client = Client::new(dotenv::var("TOKEN")?, None);
    client.connect().await?;

    while let Ok(e) = client.next().await {
        match e {
            GatewayEventData::Ready(ready) => {
                println!("Ready: {:#?}", ready);
                println!("Ready in {:?}", start.elapsed());
            },
            GatewayEventData::Hello(hello) => println!("Hello! {:#?}", hello),
        }
    }

    Ok(())
}

#[slash]
async fn ping() -> String {
    "Pong!".to_string()
}
