fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/googleapis") // you can change the generated code's location
        .compile(
            &[
                "proto/googleapis/google/iam/v1/policy.proto",
                "proto/googleapis/google/iam/v1/options.proto",
                "proto/googleapis/google/iam/v1/iam_policy.proto",
                "proto/googleapis/google/storage/v1/storage.proto",
                "proto/googleapis/google/storage/v2/storage.proto",
                "proto/googleapis/google/pubsub/v1/pubsub.proto",
                "proto/googleapis/google/bigtable/v2/bigtable.proto",
                "proto/googleapis/google/spanner/v1/spanner.proto",
                "proto/googleapis/google/spanner/admin/database/v1/spanner_database_admin.proto",
                "proto/googleapis/google/spanner/admin/instance/v1/spanner_instance_admin.proto",
            ],
            &["proto/googleapis"], // specify the root location to search proto dependencies
        )?;
    Ok(())
}
