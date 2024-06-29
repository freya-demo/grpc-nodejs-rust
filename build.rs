fn main() {
    // tonic_build::compile_protos("protos/hello_world.proto").unwrap();
    tonic_build::configure()
        .out_dir("server/compiled_protos")
        .compile(
            &["protos/hello_world.proto", "protos/counter.proto"],
            &["protos"],
        )
        .unwrap();
}
