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
#[macro_use]
extern crate maplit;

use prost_types::Duration;
use tokio_stream::StreamExt;
use tonic::{Request, Response, Status};

use google_cloud_rust_raw::googleapis::bigtable::admin::v2::bigtable_instance_admin_client::BigtableInstanceAdminClient;
use google_cloud_rust_raw::googleapis::bigtable::admin::v2::bigtable_table_admin_client::BigtableTableAdminClient;
use google_cloud_rust_raw::googleapis::bigtable::admin::v2::gc_rule::{self, Rule};
use google_cloud_rust_raw::googleapis::bigtable::admin::v2::{
    Cluster, ColumnFamily, CreateTableRequest, DeleteTableRequest, GcRule, GetClusterRequest,
    ListTablesRequest, Table,
};
use google_cloud_rust_raw::googleapis::bigtable::v2::bigtable_client::BigtableClient;
use google_cloud_rust_raw::googleapis::bigtable::v2::{
    mutate_rows_request, mutation, MutateRowsRequest, Mutation,
};
use google_cloud_rust_raw::utils::*;

/// Returns the cluster information
///
async fn get_cluster(
    client: &mut BigtableInstanceAdminClient<GcpService>,
    cluster_id: &str,
) -> Result<Response<Cluster>, Status> {
    println!("Get cluster information");
    client
        .get_cluster(Request::new(GetClusterRequest {
            name: cluster_id.into(),
        }))
        .await
}

/// Lists all tables for a given cluster
///
async fn list_tables(client: &mut BigtableTableAdminClient<GcpService>, instance_id: &str) {
    println!("List all existing tables");
    let response = client
        .list_tables(Request::new(ListTablesRequest {
            parent: instance_id.into(),
            ..ListTablesRequest::default()
        }))
        .await;
    match response {
        Ok(response) => {
            response
                .into_inner()
                .tables
                .iter()
                .for_each(|table| println!("  table: {:?}", table));
        }
        Err(error) => println!("Failed to list tables: {}", error),
    }
}

/// Create a new table in the BigTable cluster
///
async fn create_table(
    client: &mut BigtableTableAdminClient<GcpService>,
    instance_id: &str,
    table_name: &str,
    table: Table,
) -> Result<Response<Table>, Status> {
    println!("Creating table {}", table_name);
    client
        .create_table(Request::new(CreateTableRequest {
            parent: instance_id.into(),
            table_id: "hello-world".into(),
            table: Some(table),
            ..CreateTableRequest::default()
        }))
        .await
}

/// Deletes a table asynchronously, returns a future
async fn delete_table_async(
    client: &mut BigtableTableAdminClient<GcpService>,
    table_name: &str,
) -> Result<Response<()>, Status> {
    println!("Deleting the {} table", table_name);
    client
        .delete_table(Request::new(DeleteTableRequest {
            name: table_name.into(),
        }))
        .await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // BigTable project id
    let _project_id = String::from("mozilla-rust-sdk-dev");
    // The BigTable instance id
    let instance_id = String::from("projects/mozilla-rust-sdk-dev/instances/mozilla-rust-sdk");
    // The cluster id
    let cluster_id = String::from(
        "projects/mozilla-rust-sdk-dev/instances/mozilla-rust-sdk/clusters/mozilla-rust-sdk-c1",
    );
    // common table endpoint
    let endpoint = "https://bigtable.googleapis.com";
    let domain_name = "bigtable.googleapis.com";
    // Google Cloud configuration.
    let admin_endpoint = "https://bigtableadmin.googleapis.com";
    let admin_domain_name = "bigtableadmin.googleapis.com";
    // The table name
    let table_name =
        String::from("projects/mozilla-rust-sdk-dev/instances/mozilla-rust-sdk/tables/hello-world");

    let column_family_id = "cf1";

    // Create a Bigtable client.
    let auth_interceptor = AuthInterceptor::with_adc().await?;
    dbg!(&auth_interceptor);

    let admin_channel = connect(admin_domain_name, admin_endpoint).await?;

    let mut client = BigtableInstanceAdminClient::with_interceptor(
        admin_channel.clone(),
        auth_interceptor.clone(),
    );

    // display cluster information
    let cluster = get_cluster(&mut client, &cluster_id).await.unwrap();
    dbg!(&cluster);

    // create admin client for tables
    let mut admin_client =
        BigtableTableAdminClient::with_interceptor(admin_channel, auth_interceptor.clone());

    // display current tables
    list_tables(&mut admin_client, &instance_id).await;

    // create a new table with a custom column family / gc rule
    let duration = Duration {
        seconds: 60 * 60 * 24 * 5,
        nanos: 0,
    };
    let gc_rule = GcRule {
        rule: Some(Rule::Union(gc_rule::Union {
            rules: vec![
                GcRule {
                    rule: Some(Rule::MaxNumVersions(2)),
                },
                GcRule {
                    rule: Some(Rule::MaxAge(duration)),
                },
            ],
        })),
    };
    let column_family = ColumnFamily {
        gc_rule: Some(gc_rule),
    };

    let hash_map = hashmap! { column_family_id.to_string() => column_family };
    let table = Table {
        column_families: hash_map,
        ..Table::default()
    };
    match create_table(&mut admin_client, &instance_id, &table_name, table).await {
        Ok(table) => println!("  table {:?} created", table),
        Err(error) => println!("  failed to created table: {}", error),
    }

    // insert entries into new table
    println!("Insert entries into table");

    let greetings = vec!["Hello World!", "Hello Cloud!", "Hello Rust!"];
    let mut mutation_requests = Vec::new();
    let column = "greeting";
    for (i, greeting) in greetings.iter().enumerate() {
        let row_key = format!("greeting{}", i);

        let set_cell = mutation::SetCell {
            column_qualifier: column.to_string().into_bytes(),
            timestamp_micros: -1,
            value: greeting.to_string().into_bytes(),
            family_name: column_family_id.to_string(),
        };

        let mutation = Mutation {
            mutation: Some(mutation::Mutation::SetCell(set_cell)),
        };

        let entry = mutate_rows_request::Entry {
            row_key: row_key.into_bytes(),
            mutations: vec![mutation],
        };

        mutation_requests.push(entry);
    }

    let channel = connect(domain_name, endpoint).await?;
    let mut client = BigtableClient::with_interceptor(channel, auth_interceptor.clone());

    let request = MutateRowsRequest {
        table_name: table_name.to_string(),
        entries: mutation_requests,
        ..MutateRowsRequest::default()
    };

    let response = client
        .mutate_rows(request)
        .await
        .unwrap()
        .into_inner()
        .collect::<Vec<_>>()
        .await;

    for response in response.iter() {
        for entry in &response.as_ref().unwrap().entries {
            let status = entry.status.as_ref().unwrap();
            println!(
                "  entry index: {}, status: {} - {}",
                entry.index, status.code, status.message
            );
        }
    }

    // display all tables, should include new table
    list_tables(&mut admin_client, &instance_id).await;

    // delete the table
    delete_table_async(&mut admin_client, &table_name)
        .await
        .unwrap();
    // .map_err(|e| dbg!(e))
    // .expect("Failure");

    // list of tables should not have deleted table
    list_tables(&mut admin_client, &instance_id).await;

    Ok(())
}
