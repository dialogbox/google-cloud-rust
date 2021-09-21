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

pub mod google {
    #[path = "google.api.rs"]
    pub mod api;

    #[path = "google.protobuf.rs"]
    pub mod protobuf;

    #[path = "google.r#type.rs"]
    pub mod r#type;

    #[path = ""]
    pub mod iam {
        #[path = "google.iam.v1.rs"]
        pub mod v1;
    }
    #[path = ""]
    pub mod storage {
        #[path = "google.storage.v1.rs"]
        pub mod v1;
        #[path = "google.storage.v2.rs"]
        pub mod v2;
    }
}
