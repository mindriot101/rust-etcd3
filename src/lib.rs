use std::convert::TryInto;
use tonic::codegen::StdError;
use tonic::transport::Endpoint;

type EtcdResult<T> = Result<T, Box<dyn std::error::Error>>;

// Internal names, which are unfortunately named.
pub mod mvccpb {
    // Proto file: kv.proto
    tonic::include_proto!("mvccpb");
}

pub mod authpb {
    // Proto file: auth.proto
    tonic::include_proto!("authpb");
}

pub mod etcdserver {
    // Proto file: rpc.proto
    tonic::include_proto!("etcdserverpb");
}

use etcdserver::client;

/// Etcd client
pub struct EtcdClient<T> {
    kv_client: client::KvClient<T>,
    cluster_client: client::ClusterClient<T>,
    status_client: client::MaintenanceClient<T>,
    watch_client: client::WatchClient<T>,
}

impl EtcdClient<tonic::transport::channel::Channel> {
    pub async fn connect<D>(dst: D) -> EtcdResult<Self>
    where
        D: TryInto<Endpoint> + Clone,
        D::Error: Into<StdError>,
    {
        let kv_client = client::KvClient::connect(dst.clone()).await?;
        let cluster_client = client::ClusterClient::connect(dst.clone()).await?;
        let status_client = client::MaintenanceClient::connect(dst.clone()).await?;
        let watch_client = client::WatchClient::connect(dst).await?;

        Ok(Self {
            kv_client,
            cluster_client,
            status_client,
            watch_client,
        })
    }

    pub async fn put<K: Into<Vec<u8>>, V: Into<Vec<u8>>>(
        &mut self,
        key: K,
        value: V,
    ) -> EtcdResult<etcdserver::PutResponse> {
        let request = etcdserver::PutRequest {
            key: key.into(),
            value: value.into(),
            prev_kv: true,
            ..Default::default()
        };

        let response = self.kv_client.put(request).await?;
        Ok(response.into_inner())
    }

    pub async fn get<K: Into<Vec<u8>>>(&mut self, key: K) -> EtcdResult<etcdserver::RangeResponse> {
        let request = etcdserver::RangeRequest {
            key: key.into(),
            ..Default::default()
        };
        let response = self.kv_client.range(request).await?;
        Ok(response.into_inner())
    }

    pub async fn watch<K>(
        &mut self,
        key: K,
    ) -> EtcdResult<tonic::Streaming<etcdserver::WatchResponse>>
    where
        K: Into<Vec<u8>> + Sync + Send + 'static,
    {
        let request = async_stream::stream! {
            let watch_create_req = etcdserver::WatchCreateRequest {
                key: key.into(),
                ..Default::default()
            };
            let request_union = etcdserver::watch_request::RequestUnion::CreateRequest(watch_create_req);
            let request = etcdserver::WatchRequest {
                request_union: Some(request_union),
            };

            yield request;
        };

        let response = self.watch_client.watch(request).await?;
        let inbound = response.into_inner();

        Ok(inbound)
    }

    pub async fn status(&mut self) -> EtcdResult<etcdserver::StatusResponse> {
        let request = etcdserver::StatusRequest {};
        let response = self.status_client.status(request).await?;
        Ok(response.into_inner())
    }

    pub async fn server_alarms(&mut self) -> EtcdResult<etcdserver::AlarmResponse> {
        let mut request = etcdserver::AlarmRequest::default();
        request.set_action(etcdserver::alarm_request::AlarmAction::Get);
        let response = self.status_client.alarm(request).await?;
        Ok(response.into_inner())
    }

    pub async fn cluster_members(&mut self) -> EtcdResult<etcdserver::MemberListResponse> {
        let request = etcdserver::MemberListRequest {};
        let response = self.cluster_client.member_list(request).await?;
        Ok(response.into_inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn connecting_to_instance() {
        let _client = EtcdClient::connect("http://127.0.0.1:2379").await.unwrap();
    }

    #[tokio::test]
    async fn test_putting_and_getting_a_value() {
        let mut client = EtcdClient::connect("http://127.0.0.1:2379").await.unwrap();

        client.put("foo", "bar").await.unwrap();
        let result = client.get("foo").await.unwrap();

        assert_eq!(result.count, 1);

        let kv = &result.kvs[0];
        assert_eq!(kv.key, "foo".to_string().into_bytes());
        assert_eq!(kv.value, "bar".to_string().into_bytes());
    }
}
