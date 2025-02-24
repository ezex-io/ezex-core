fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("compiling vault.proto file");
    tonic_build::compile_protos("./src/api/grpc/proto/vault.proto")?;
    Ok(())
}
