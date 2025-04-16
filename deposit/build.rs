fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("compiling deposit.proto file");
    tonic_build::compile_protos("./src/grpc/proto/deposit.proto")?;
    Ok(())
}
