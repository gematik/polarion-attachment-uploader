// Copyright 2024 gematik GmbH
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde::Deserialize;

/// HTTP 200 response object for
///
///     GET /projects/{projectId}/spaces/{spaceId}/documents/{documentName}/attachments
///
/// See https://pet-gematikde.msappproxy.net/polarion/rest/v1#operations-Document_Attachments-getDocumentAttachments
/// for further details.
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub(crate) struct Response {
    pub(crate) data: Vec<Attachment>,
    pub(crate) links: Option<PaginationLinks>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct Attachment {
    #[serde(rename = "type")]
    pub(crate) type_field: String,
    pub(crate) id: String,
    pub(crate) links: ContentLinks,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct ContentLinks {
    #[serde(rename = "self")]
    pub(crate) self_field: String,
    pub(crate) content: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct PaginationLinks {
    #[serde(rename = "self")]
    pub(crate) self_field: String,
    pub(crate) first: String,
    pub(crate) prev: Option<String>,
    pub(crate) next: Option<String>,
    pub(crate) last: String,
}
