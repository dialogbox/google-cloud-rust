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

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use gcp_auth::Token;
use google_cloud_rust_raw::googleapis::bigtable::admin::v2::{ColumnFamily, GcRule, Table};
use prost_types::Duration;

use tonic::{
    body::BoxBody,
    client::GrpcService,
    metadata::{errors::InvalidMetadataValue, AsciiMetadataValue, MetadataValue},
    service::interceptor::InterceptedService,
    service::Interceptor,
    transport::{Channel, ClientTlsConfig},
    Request, Response, Status,
};

use google_cloud_rust_raw::googleapis::bigtable::admin::v2::{
    bigtable_instance_admin_client::BigtableInstanceAdminClient,
    bigtable_table_admin_client::BigtableTableAdminClient, Cluster, CreateTableRequest,
    DeleteTableRequest, GetClusterRequest, ListTablesRequest,
};

#[allow(dead_code)]
fn timestamp() -> u128 {
    let start = SystemTime::now();
    let time = start
        .duration_since(UNIX_EPOCH)
        .expect("Failed to fetch timestamp");
    time.as_micros()
}

/// Create a new channel used for the different types of clients
async fn connect(
    domain_name: &'static str,
    endpoint: &'static str,
) -> Result<Channel, tonic::transport::Error> {
    let tls_config = ClientTlsConfig::new().domain_name(domain_name);
    let channel = Channel::from_static(endpoint)
        .tls_config(tls_config)?
        .connect()
        .await?;

    Ok(channel)
}

async fn auth_header() -> Result<AsciiMetadataValue, gcp_auth::Error> {
    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
    let authentication_manager = gcp_auth::init().await?;
    let token = authentication_manager.get_token(scopes).await?;

    let bearer_token = format!("Bearer {}", token.as_str());
    Ok(MetadataValue::from_str(&bearer_token).unwrap())
}

pub type BigtableInstanceAdminClientType =
    BigtableInstanceAdminClient<InterceptedService<Channel, AuthInterceptor>>;
pub type BigtableTableAdminClientType =
    BigtableTableAdminClient<InterceptedService<Channel, AuthInterceptor>>;

/// Returns the cluster information
///
async fn get_cluster(
    client: &mut BigtableInstanceAdminClientType,
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
async fn list_tables(client: &mut BigtableTableAdminClientType, instance_id: &str) {
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
    client: &mut BigtableTableAdminClientType,
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
    client: &mut BigtableTableAdminClientType,
    table_name: &str,
) -> Result<Response<()>, Status> {
    println!("Deleting the {} table", table_name);
    client
        .delete_table(Request::new(DeleteTableRequest {
            name: table_name.into(),
        }))
        .await
}

pub struct AuthInterceptor {
    header_value: AsciiMetadataValue,
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        request
            .metadata_mut()
            .insert("authorization", self.header_value.clone());
        Ok(request)
    }
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
    let channel = connect(admin_domain_name, admin_endpoint).await?;
    let header_value = auth_header().await?;
    dbg!(&header_value);

    let mut client =
        BigtableInstanceAdminClient::with_interceptor(channel, AuthInterceptor { header_value });
    dbg!(&client);

    // display cluster information
    let cluster = get_cluster(&mut client, &cluster_id).await.unwrap();
    dbg!(&cluster);

    //     // create admin client for tables
    //     let mut admin_client =
    //         BigtableTableAdminClient::with_interceptor(channel, move |mut req: Request<()>| {
    //             req.metadata_mut()
    //                 .insert("authorization", header_value.clone());
    //             Ok(req)
    //         });

    //     // display current tables
    //     list_tables(&admin_client, &instance_id);

    //     // create a new table with a custom column family / gc rule
    //     let mut duration = Duration::new();
    //     duration.set_seconds(60 * 60 * 24 * 5);
    //     let mut gc_rule = GcRule::new();
    //     gc_rule.set_max_num_versions(2);
    //     gc_rule.set_max_age(duration);
    //     let mut column_family = ColumnFamily::new();
    //     column_family.set_gc_rule(gc_rule);
    //     let mut hash_map = HashMap::new();
    //     hash_map.insert(column_family_id.to_string(), column_family);
    //     let mut table = Table::new();
    //     table.set_column_families(hash_map);
    //     match create_table(&admin_client, &instance_id, &table_name, table) {
    //         Ok(table) => println!("  table {:?} created", table),
    //         Err(error) => println!("  failed to created table: {}", error),
    //     }

    //     // insert entries into new table
    //     println!("Insert entries into table");

    //     let greetings = vec!["Hello World!", "Hello Cloud!", "Hello Rust!"];
    //     let mut mutation_requests = Vec::new();
    //     let column = "greeting";
    //     for (i, greeting) in greetings.iter().enumerate() {
    //         let row_key = format!("greeting{}", i);

    //         let mut set_cell = Mutation_SetCell::new();
    //         set_cell.set_column_qualifier(column.to_string().into_bytes());
    //         set_cell.set_timestamp_micros(-1);
    //         set_cell.set_value(greeting.to_string().into_bytes());
    //         set_cell.set_family_name(column_family_id.to_string());

    //         let mut mutation = Mutation::new();
    //         mutation.set_set_cell(set_cell);

    //         let mut request = MutateRowsRequest_Entry::new();
    //         request.set_row_key(row_key.into_bytes());
    //         request.set_mutations(RepeatedField::from_vec(vec![mutation]));

    //         mutation_requests.push(request);
    //     }

    //     let channel = connect(endpoint);
    //     let _client = BigtableClient::new(channel.clone());
    //     let mut request = MutateRowsRequest::new();
    //     request.set_table_name(table_name.to_string());
    //     request.set_entries(RepeatedField::from_vec(mutation_requests));

    //     /*

    //     TODO:: fix this.admin_client
    //     `.collect()` needs a type.
    //     // apply changes and check responses
    //     let response = client
    //         .mutate_rows(&request).unwrap()
    //         .collect()
    //         .into_future()
    //         .wait().unwrap();
    //     for response in response.iter() {
    //         for entry in response.get_entries().iter() {
    //             let status = entry.get_status();
    //             println!(
    //                 "  entry index: {}, status: {} - {}",
    //                 entry.get_index(),
    //                 status.code,
    //                 status.message
    //             );
    //         }
    //     }
    //     */
    //     // display all tables, should include new table
    //     list_tables(&admin_client, &instance_id);

    //     // delete the table
    //     delete_table_async(&admin_client, &table_name)
    //         .unwrap()
    //         .await
    //         .map_err(|e| dbg!(e))
    //         .expect("Failure");

    //     // list of tables should not have deleted table
    //     list_tables(&admin_client, &instance_id);

    Ok(())
}
