// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use google_cloud_rust_raw::googleapis::bigtable::v2::bigtable_client::BigtableClient;
use google_cloud_rust_raw::googleapis::bigtable::v2::ReadRowsRequest;
use google_cloud_rust_raw::utils::*;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // BigTable project id
    let project_id = env::var("PROJECT_ID").unwrap_or("mozilla-rust-sdk-dev".to_string());
    // An example database inside Mozilla's Bigtable instance.
    let table = format!(
        "projects/{}/instances/mozilla-rust-sdk/tables/prezzy",
        project_id
    );

    // Google Cloud configuration.
    let endpoint = "https://bigtable.googleapis.com";

    // Create an auth interceptor
    let auth_interceptor = AuthInterceptor::with_adc().await?;

    // Set up the gRPC environment.
    let channel = connect(endpoint).await?;
    let mut client = BigtableClient::with_interceptor(channel, auth_interceptor.clone());

    // Create a request to read all rows.
    let req = ReadRowsRequest {
        table_name: table.to_string(),
        ..ReadRowsRequest::default()
    };

    // Iterate over the rows and print them.
    let mut stream = match client.read_rows(req).await {
        Ok(s) => s.into_inner(),
        Err(e) => {
            println!("Error: {:?}", e);
            return Ok(());
        }
    };
    while let Some(row) = stream.message().await? {
        dbg!(&row);
    }

    Ok(())
}
