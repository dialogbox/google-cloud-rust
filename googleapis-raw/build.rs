fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/google") // you can change the generated code's location
        .compile(
            &[
                "proto/googleapis/google/storage/v1/storage.proto",
                "proto/googleapis/google/storage/v2/storage.proto",
            ],
            &["proto/googleapis"], // specify the root location to search proto dependencies
        )?;
    Ok(())
}
