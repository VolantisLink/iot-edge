fn main() {
    ::capnpc::CompilerCommand::new()
        .file("./schema/chunk.capnp")
        .run()
        .expect("compiling schema");
}