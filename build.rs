fn main() {
    tonic_build::compile_protos("proto/kvdb.proto").unwrap();
}
