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

pub mod utils;

pub mod googleapis {
    #[path = "google.api.rs"]
    pub mod api;

    #[path = ""]
    pub mod bigtable {
        #[path = "google.bigtable.v2.rs"]
        pub mod v2;

        #[path = ""]
        pub mod admin {
            #[path = "google.bigtable.admin.v2.rs"]
            pub mod v2;
        }
    }

    #[path = ""]
    pub mod iam {
        #[path = "google.iam.v1.rs"]
        pub mod v1;
    }

    #[path = "google.longrunning.rs"]
    pub mod longrunning;

    #[path = "google.protobuf.rs"]
    pub mod protobuf;

    #[path = ""]
    pub mod pubsub {
        #[path = "google.pubsub.v1.rs"]
        pub mod v1;
    }

    #[path = "google.r#type.rs"]
    pub mod r#type;

    #[path = "google.rpc.rs"]
    pub mod rpc;

    #[path = ""]
    pub mod spanner {
        #[path = "google.spanner.v1.rs"]
        pub mod v1;

        #[path = ""]
        pub mod admin {
            #[path = ""]
            pub mod database {
                #[path = "google.spanner.admin.database.v1.rs"]
                pub mod v1;
            }

            #[path = ""]
            pub mod instance {
                #[path = "google.spanner.admin.instance.v1.rs"]
                pub mod v1;
            }
        }
    }

    #[path = ""]
    pub mod storage {
        #[path = "google.storage.v1.rs"]
        pub mod v1;
        #[path = "google.storage.v2.rs"]
        pub mod v2;
    }
}
