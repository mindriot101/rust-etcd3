//! Mirror of the low level ops example, but using the high level API provided by the etcd3 crate.
//!
use etcd3::EtcdClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = EtcdClient::connect("http://127.0.0.1:2379").await?;

    // KV
    client.put("foo", "bar").await?;
    let result = client.get("foo").await?;
    println!("{:#?}", result);

    // Status
    let server_status = client.status().await?;
    println!("{:#?}", server_status);

    // Alarms
    let alarms = client.server_alarms().await?;
    println!("{:#?}", alarms);

    // Cluster
    let members = client.cluster_members().await?;
    println!("{:#?}", members);

    // Watch
    let mut stream = client.watch("foo").await?;
    while let Some(msg) = stream.message().await? {
        println!("{:#?}", msg);
    }

    Ok(())
}
