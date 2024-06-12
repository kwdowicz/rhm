fn main() {
    tonic_build::compile_protos("proto/rhm_service.proto").unwrap();
    tonic_build::compile_protos("proto/cluster_service.proto").unwrap();
}
