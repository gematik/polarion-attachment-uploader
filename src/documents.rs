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

use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
};

/// Polarion document
#[derive(Deserialize, Debug)]
pub(crate) struct Document {
    pub(crate) project: String,
    pub(crate) space: String,
    pub(crate) name: String,
}

impl Document {
    /// Builds the document's prefix by slash-separating its project, space and document ID
    pub(crate) fn prefix(&self) -> String {
        format!("{}/{}/{}", self.project, self.space, self.name)
    }

    /// Builds an attachment's ID from its name
    pub(crate) fn get_attachment_id(&self, name: &str) -> String {
        format!("{}/{}", self.prefix(), name)
    }
}

/// Reads the document configuration
pub(crate) fn read_document_configuration(path: &PathBuf) -> Result<HashMap<String, Document>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader::<_, HashMap<String, Document>>(
        reader,
    )?)
}

/// Tries to determine the corresponding Polarion document and attachment name from a path
pub(crate) fn guess_document_and_name<'a>(
    path: &'a PathBuf,
    documents: &'a HashMap<String, Document>,
) -> Result<(&'a Document, String)> {
    let total = path.components().count();
    let mut remainder = total - 1;

    while remainder > 0 {
        let head = path.components().take(remainder).collect::<PathBuf>();

        if fs::metadata(head.join(".git")).is_ok() {
            break; // We've reached the root of a git repository, no need to traverse further
        }

        let folder = head.file_name().and_then(|n| n.to_str());

        if let Some(document) = folder.and_then(|n| documents.get(n)) {
            let name = path
                .components()
                .skip(remainder)
                .map(|c| c.as_os_str().to_string_lossy().to_string())
                .collect::<Vec<String>>()
                .join("___");
            return Ok((document, name));
        }

        remainder -= 1;
    }

    Err(anyhow!(
        "Could not determine Polarion document for {:?}",
        path
    ))
}
