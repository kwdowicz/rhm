fn main() {
    tonic_build::compile_protos("proto/rhm_service.proto").unwrap();
}
