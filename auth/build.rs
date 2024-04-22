fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .protoc_arg("-I=../proto")
        .compile(&["auth.proto"], &["proto"])?;

    Ok(())
}
