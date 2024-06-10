use anyhow::Result;
use colored::Colorize;
use dirs::config_dir;
use documents::{guess_document_and_name, read_document_configuration, Document};
use networking::{
    create_client, is_attachment_existing, patch_attachment, post_attachment, NetworkClient,
};
use std::collections::HashMap;
use std::env;
use std::io::stdin;
use std::path::PathBuf;
use std::str::{self, FromStr};

mod api_types;
mod documents;
mod networking;

/// The ... well ... main function
fn main() {
    let base_url =
        env::var("POLARION_API").expect("Should have a POLARION_API environment variable");
    let token = env::var("POLARION_TOKEN")
        .expect("Should have a POLARION_ACCESS_TOKEN environment variable");
    let client = create_client(base_url, token).expect("Should be able to create network client");

    let configuration_path = config_dir()
        .expect("Should have a user-level config directory")
        .join("polarion-documents.json");
    let documents = read_document_configuration(&configuration_path).unwrap_or_else(|_| {
        panic!(
            "Should have a valid config file in {:?}",
            configuration_path
        )
    });

    for arg in std::env::args().skip(1) {
        println!("Processing {}", arg);
        if let Err(error) = process_arg(arg.as_str(), &documents, &client) {
            let msg = format!("[Error]: {}", error);
            eprintln!("{}", msg.red());
        }
    }
}

/// Processes a single command line argument
fn process_arg(
    arg: &str,
    documents: &HashMap<String, Document>,
    client: &NetworkClient,
) -> Result<()> {
    let path = PathBuf::from_str(arg)?.canonicalize()?;
    let (document, name) = guess_document_and_name(&path, documents)?;
    let is_existing = is_attachment_existing(document, &name, client)?;

    confirm_operation(document, &name, is_existing)?;

    if is_existing {
        patch_attachment(&path, document, &name, client)?;
    } else {
        post_attachment(&path, document, &name, client)?;
    }

    Ok(())
}

/// Asks the user to confirm uploading or replacing the attachment
fn confirm_operation(document: &Document, name: &str, is_existing: bool) -> Result<()> {
    let message = if is_existing {
        format!(
            "Attachment {} already exists in {}. Hit <Enter> to {} it.",
            name.bold(),
            document.prefix().bold(),
            "overwrite".bold()
        )
    } else {
        format!(
            "Attachment {} doesn't exist in {}. Hit <Enter> to {} it.",
            name.bold(),
            document.prefix().bold(),
            "upload".bold()
        )
    };
    println!("{}", message.cyan());

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    Ok(())
}
