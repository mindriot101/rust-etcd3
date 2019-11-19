use etcd3::{EtcdClient, EtcdResult};
use std::time;
use tokio::timer::delay;

async fn sleep() {
    let when = tokio::clock::now() + time::Duration::from_secs(2);
    delay(when).await;
}

#[tokio::main]
async fn main() -> EtcdResult<()> {
    // Connect to the server
    let mut client = EtcdClient::connect("http://127.0.0.1:2379").await?;

    // Set up the watch
    let mut stream = client.watch("foo").await?;

    // Spawn the watcher task in the background
    tokio::spawn(async move {
        while let Some(msg) = stream.message().await.unwrap() {
            println!("Value changed: {:#?}", msg);
        }
    });

    sleep().await;

    // Now we write the key a few times
    let mut range = client.range("foo", None);
    range.put("bar").await?;

    sleep().await;

    range.put("baz").await?;

    Ok(())
}
