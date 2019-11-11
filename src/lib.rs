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
    ) -> EtcdResult<()> {
        let request = etcdserver::PutRequest {
            key: key.into(),
            value: value.into(),
            prev_kv: true,
            ..Default::default()
        };

        let response = self.kv_client.put(request).await?;
        match response.metadata().get("grpc-status") {
            Some(v) => {
                if v == "0" {
                    Ok(())
                } else {
                    Err(format!("grpc status code {:?}", v).into())
                }
            }
            None => Ok(()),
        }
    }

    pub async fn get<K: Into<Vec<u8>>>(&mut self, key: K) -> EtcdResult<Vec<Vec<u8>>> {
        let request = etcdserver::RangeRequest {
            key: key.into(),
            ..Default::default()
        };
        let response = self.kv_client.range(request).await?;

        // Inline function to extract the values out of the range response
        fn extract_values(response: tonic::Response<etcdserver::RangeResponse>) -> Vec<Vec<u8>> {
            response.into_inner().kvs.iter().map(|kv| kv.value.clone()).collect()
        }

        match response.metadata().get("grpc-status") {
            Some(v) => {
                if v == "0" {
                    Ok(extract_values(response))
                } else {
                    Err(format!("grpc status code {:?}", v).into())
                }
            }
            None => Ok(extract_values(response)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn connecting_to_instance() {
        let client = EtcdClient::connect("http://127.0.0.1:2379").await.unwrap();
    }

    #[tokio::test]
    async fn test_putting_and_getting_a_value() {
        let mut client = EtcdClient::connect("http://127.0.0.1:2379").await.unwrap();

        client.put("foo", "bar").await.unwrap();
        let result = client.get("foo").await.unwrap();

        assert_eq!(result, vec!["bar".to_string().into_bytes()]);
    }
}
