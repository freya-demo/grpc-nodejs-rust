fn main() {
    tonic_build::compile_protos("protos/hello_world.proto").unwrap();
}
