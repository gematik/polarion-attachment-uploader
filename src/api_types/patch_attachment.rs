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

use serde::Serialize;

/// Request object for `resource` part of
///
///     PATCH /projects/{projectId}/spaces/{spaceId}/documents/{documentName}/attachments/{attachmentId}
///
/// See https://pet-gematikde.msappproxy.net/polarion/rest/v1#operations-Document_Attachments-patchDocumentAttachment
/// for further details.
#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub(crate) struct Resource {
    pub(crate) data: Item,
}

impl Resource {
    /// Creates a resource object for replacing a single file
    pub(crate) fn create(id: &str, name: &str) -> Self {
        Self {
            data: Item {
                type_field: "document_attachments".to_string(),
                id: id.to_string(),
                attributes: Attributes {
                    title: name.to_string(),
                },
            },
        }
    }
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub(crate) struct Item {
    #[serde(rename = "type")]
    pub(crate) type_field: String,
    pub(crate) id: String,
    pub(crate) attributes: Attributes,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub(crate) struct Attributes {
    pub(crate) title: String,
}
