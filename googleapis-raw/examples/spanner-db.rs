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

use google_cloud_rust_raw::googleapis::longrunning::operations_client::OperationsClient;
use google_cloud_rust_raw::googleapis::longrunning::GetOperationRequest;
use google_cloud_rust_raw::googleapis::spanner::admin::database::v1::database_admin_client::DatabaseAdminClient;
use google_cloud_rust_raw::googleapis::spanner::admin::database::v1::{
    CreateDatabaseRequest, DropDatabaseRequest, GetDatabaseRequest,
};
use google_cloud_rust_raw::googleapis::spanner::v1::spanner_client::SpannerClient;
use google_cloud_rust_raw::googleapis::spanner::v1::transaction_options::{self, ReadWrite};
use google_cloud_rust_raw::googleapis::spanner::v1::{
    commit_request, mutation, BeginTransactionRequest, CommitRequest, CreateSessionRequest,
    Mutation, Session, TransactionOptions,
};
use google_cloud_rust_raw::utils::{connect, AuthInterceptor, GcpService};
use prost_types::{value::Kind::*, ListValue, Value};
use std::env;
use std::time::Duration;
use tonic::{Request, Response, Status};

const CREATE_DATABASE: &str = "CREATE DATABASE music";

const CREATE_SINGER_TABLE: &str = "CREATE TABLE Singers (
      SingerId   INT64 NOT NULL,
      FirstName  STRING(1024),
      LastName   STRING(1024),
      SingerInfo BYTES(MAX),
    ) PRIMARY KEY (SingerId)";

const CREATE_ALBUMS_TABLE: &str = "CREATE TABLE Albums (
      SingerId     INT64 NOT NULL,
      AlbumId      INT64 NOT NULL,
      AlbumTitle   STRING(MAX),
    ) PRIMARY KEY (SingerId, AlbumId),
      INTERLEAVE IN PARENT Singers ON DELETE CASCADE";

/// A basic struct for a singer
///
struct Singer {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
}

/// Waits until the operation is finished
///
async fn wait_operation_finished(service: &GcpService, operation: &str) {
    let mut operations_client = OperationsClient::new(service.clone());

    let request = GetOperationRequest {
        name: operation.to_string(),
    };

    loop {
        println!("Checking operation: {}", operation);
        match operations_client
            .get_operation(Request::new(request.clone()))
            .await
        {
            Ok(response) => {
                if response.into_inner().done {
                    println!("Operation {} finished", operation);
                    break;
                }

                // wait instead
                let wait_time = Duration::from_millis(250);
                std::thread::sleep(wait_time);
            }
            Err(error) => {
                println!("Failed to get operation");
                dbg!(error);
            }
        }
    }
}

/// Creates a new database if it does not exist yet.
///
async fn create_database_if_not_exists(
    service: &GcpService,
    database_name: &str,
    instance_id: &str,
) {
    let mut client = DatabaseAdminClient::new(service.clone());

    // find database
    println!("Finding database {}", database_name);
    let request = GetDatabaseRequest {
        name: database_name.to_string(),
    };
    if let Ok(database) = client.get_database(Request::new(request)).await {
        println!("Found database: {}", database.into_inner().name);
        return;
    } else {
        println!("Database not found");
    }

    // create a new database
    println!("Create database {}", database_name);
    let statements = vec![CREATE_SINGER_TABLE, CREATE_ALBUMS_TABLE]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let request = CreateDatabaseRequest {
        parent: instance_id.to_string(),
        create_statement: CREATE_DATABASE.to_string(),
        extra_statements: statements,
        ..CreateDatabaseRequest::default()
    };
    let operation = client
        .create_database(Request::new(request))
        .await
        .expect("Failed to create database");
    dbg!(operation.get_ref());

    // check that operation is finished
    wait_operation_finished(&service, &operation.into_inner().name).await;
}

/// Deletes a given database
///
#[allow(dead_code)]
async fn drop_database(service: &mut GcpService, database_name: &str) {
    println!("Drop database {}", database_name);
    let mut client = DatabaseAdminClient::new(service);

    let request = DropDatabaseRequest {
        database: database_name.to_string(),
    };

    client.drop_database(Request::new(request)).await.unwrap();
}

/// Create a new session to communicate with Spanner
///
async fn create_session(
    client: &mut SpannerClient<GcpService>,
    database_name: &str,
) -> Result<Response<Session>, Status> {
    let mut request = Request::new(CreateSessionRequest {
        database: database_name.to_string(),
        ..CreateSessionRequest::default()
    });
    request.metadata_mut().append(
        "google-cloud-resource-prefix",
        database_name.parse().expect("Failed to set meta data"),
    );
    request.metadata_mut().append(
        "x-goog-api-client",
        "googleapis-rs".parse().expect("Failed to set meta data"),
    );
    client.create_session(request).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = "https://spanner.googleapis.com";

    // global project_id
    let project_id = env::var("PROJECT_ID").unwrap_or("mozilla-rust-sdk-dev".to_string());
    // spanner instance id
    let instance_id = format!("projects/{}/instances/mozilla-spanner-dev", project_id);
    // database name
    let database_name = format!(
        "projects/{}/instances/mozilla-spanner-dev/databases/music",
        project_id
    );

    // Set up the gRPC environment.
    let channel = connect(endpoint).await?;

    // Create an auth interceptor
    let auth_interceptor = AuthInterceptor::with_adc().await?;

    // Create an GCP Service
    let gcp_service = GcpService::new(channel, auth_interceptor);

    let mut client = SpannerClient::new(gcp_service.clone());

    // create database if it not already exists
    create_database_if_not_exists(&gcp_service, &database_name, &instance_id).await;

    // create session to communicate
    let session = create_session(&mut client, &database_name)
        .await?
        .into_inner();

    // insert data into database by using a transaction
    let request = BeginTransactionRequest {
        session: session.name.to_string(),
        options: Some(TransactionOptions {
            mode: Some(transaction_options::Mode::ReadWrite(ReadWrite {})),
        }),
        ..BeginTransactionRequest::default()
    };

    let transaction = client
        .begin_transaction(Request::new(request))
        .await?
        .into_inner();

    // the list of singers to add
    let columns = vec![
        "SingerId".to_string(),
        "FirstName".to_string(),
        "LastName".to_string(),
    ];
    let singers = vec![
        Singer {
            id: 1,
            first_name: "Marc".to_string(),
            last_name: "Richards".to_string(),
        },
        Singer {
            id: 2,
            first_name: "Catalina".to_string(),
            last_name: "Smith".to_string(),
        },
        Singer {
            id: 3,
            first_name: "Alice".to_string(),
            last_name: "Trentor".to_string(),
        },
        Singer {
            id: 4,
            first_name: "Lea".to_string(),
            last_name: "Martin".to_string(),
        },
        Singer {
            id: 5,
            first_name: "David".to_string(),
            last_name: "Lomond".to_string(),
        },
    ];

    // collect all values
    let mut list_values = Vec::new();
    for singer in singers {
        let id = Value {
            kind: Some(StringValue(singer.id.to_string())),
        };
        let first_name = Value {
            kind: Some(StringValue(singer.first_name.to_string())),
        };
        let last_name = Value {
            kind: Some(StringValue(singer.last_name.to_string())),
        };

        let list = ListValue {
            values: vec![id, first_name, last_name],
        };
        list_values.push(list);
    }

    // create a suitable mutation with all values
    println!("Preparing write mutation to add singers");
    let mutation_write = mutation::Write {
        table: "Singers".to_string(),
        columns: columns,
        values: list_values,
    };
    println!("Mutation write object");
    dbg!(mutation_write.clone());

    let mutation = Mutation {
        operation: Some(mutation::Operation::InsertOrUpdate(mutation_write)),
    };

    // finally commit to database
    println!("Commit data to database {}", database_name);
    let commit = CommitRequest {
        transaction: Some(commit_request::Transaction::TransactionId(transaction.id)),
        session: session.name.to_string(),
        mutations: vec![mutation],
        ..CommitRequest::default()
    };
    let response = client.commit(Request::new(commit)).await?.into_inner();
    dbg!(response);

    // delete database
    // drop_database(&mut gcp_service, &database_name).await;

    Ok(())
}
