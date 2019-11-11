// Internal names, which are unfortunately named.
mod mvccpb {
    // Proto file: kv.proto
    tonic::include_proto!("mvccpb");
}

mod authpb {
    // Proto file: auth.proto
    tonic::include_proto!("authpb");
}

mod etcdserver {
    // Proto file: rpc.proto
    tonic::include_proto!("etcdserverpb");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
