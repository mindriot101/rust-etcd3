use std::path::PathBuf;

fn main() {
    let protos = &["proto/auth.proto", "proto/kv.proto", "proto/rpc.proto"]
        .iter()
        .map(PathBuf::from)
        .collect::<Vec<_>>();

    match tonic_build::configure().compile(&protos, &[PathBuf::from("proto")]) {
        Ok(_) => {}
        Err(e) => {
            panic!("{}", e);
        }
    }
}
