//! This example uses the gRPC modules directly to build a client, connect to a running etcd
//! instance, and perform some actions.
//!
use std::error::Error;

static HOST_STR: &str = "http://127.0.0.1:2379";

#[allow(dead_code)]
async fn run_kv() -> Result<(), Box<dyn Error>> {
    let mut client = etcd3::etcdserver::client::KvClient::connect(HOST_STR).await?;

    // Put some data into etcd
    let request = etcd3::etcdserver::PutRequest {
        key: "foo".to_string().into_bytes(),
        value: "bar".to_string().into_bytes(),
        prev_kv: true,
        ..Default::default()
    };
    let response = client.put(request).await?;

    // Now we have the response, let's take a look at it.
    println!("{:#?}", response);

    // Get the value back out
    let request = etcd3::etcdserver::RangeRequest {
        key: "foo".to_string().into_bytes(),
        ..Default::default()
    };
    let response = client.range(request).await?;

    println!("{:#?}", response);

    Ok(())
}

#[allow(dead_code)]
async fn run_status() -> Result<(), Box<dyn Error>> {
    let mut client = etcd3::etcdserver::client::MaintenanceClient::connect(HOST_STR).await?;

    // Check on the status of the server
    let request = etcd3::etcdserver::StatusRequest {};
    let response = client.status(request).await?;

    // Now we have the response, let's take a look at it.
    println!("{:#?}", response);

    // Check on the alarms
    use etcd3::etcdserver::alarm_request::AlarmAction;

    let mut request = etcd3::etcdserver::AlarmRequest::default();
    request.set_action(AlarmAction::Get);

    let response = client.alarm(request).await?;
    println!("{:#?}", response);
    Ok(())
}

#[allow(dead_code)]
async fn run_cluster() -> Result<(), Box<dyn Error>> {
    let mut client = etcd3::etcdserver::client::ClusterClient::connect(HOST_STR).await?;

    let request = etcd3::etcdserver::MemberListRequest {};
    let response = client.member_list(request).await?;

    println!("{:#?}", response);
    Ok(())
}

#[allow(dead_code)]
async fn run_watch() -> Result<(), Box<dyn Error>> {
    let mut client = etcd3::etcdserver::client::WatchClient::connect(HOST_STR).await?;

    let request = async_stream::stream! {
        // Create a "streaming" request, that creates a watch on the "foo" key.
        let watch_create_req = etcd3::etcdserver::WatchCreateRequest {
            key: "foo".to_string().into_bytes(),
            ..Default::default()
        };
        let request_union = etcd3::etcdserver::watch_request::RequestUnion::CreateRequest(watch_create_req);
        let request = etcd3::etcdserver::WatchRequest {
            request_union: Some(request_union),
        };

        yield request
    };

    let response = client.watch(request).await?;
    let mut inbound = response.into_inner();

    while let Some(msg) = inbound.message().await? {
        println!("{:#?}", msg);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    run_kv().await?;
    run_status().await?;
    run_cluster().await?;
    run_watch().await?;

    Ok(())
}
