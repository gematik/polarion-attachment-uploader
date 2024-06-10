use crate::api_types::{get_attachments, patch_attachment, post_attachments};
use crate::documents::Document;
use anyhow::Result;
use colored::Colorize;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use reqwest::blocking::multipart::Form;
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::IntoUrl;
use serde_json::to_string;
use std::path::PathBuf;

/// Control characters for path components that require escaping (https://url.spec.whatwg.org/#path-percent-encode-set)
const PATH_COMPONENT: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'?')
    .add(b'`')
    .add(b'{')
    .add(b'}')
    .add(b'/');

pub(crate) struct NetworkClient {
    client: Client,
    base_url: String,
}

impl NetworkClient {
    /// Creates a full URL from a given path
    fn create_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    /// Sends a GET request for a given URL
    fn get_url<U: IntoUrl>(&self, url: U) -> Result<Response> {
        NetworkClient::log_request("GET", url.as_str());
        let response = self.client.get(url).send()?;
        NetworkClient::log_response(&response);
        Ok(response)
    }

    /// Sends a POST request for a given path
    fn post(&self, path: &str, form: Option<Form>) -> Result<Response> {
        self.post_url(self.create_url(path), form)
    }

    /// Sends a POST request for a given URL
    fn post_url<U: IntoUrl>(&self, url: U, form: Option<Form>) -> Result<Response> {
        let mut request = self.client.post(url.as_str());
        if let Some(form) = form {
            request = request.multipart(form);
        }

        NetworkClient::log_request("POST", url.as_str());
        let response = request.send()?;
        NetworkClient::log_response(&response);

        Ok(response)
    }

    /// Sends a PATCH request for a given path
    fn patch(&self, path: &str, form: Option<Form>) -> Result<Response> {
        self.patch_url(self.create_url(path), form)
    }

    /// Sends a PATCH request for a given URL
    fn patch_url<U: IntoUrl>(&self, url: U, form: Option<Form>) -> Result<Response> {
        let mut request = self.client.patch(url.as_str());
        if let Some(form) = form {
            request = request.multipart(form);
        }

        NetworkClient::log_request("PATCH", url.as_str());
        let response = request.send()?;
        NetworkClient::log_response(&response);

        Ok(response)
    }

    fn log_request(operation: &str, url: &str) {
        println!(
            "{} {}",
            operation.truecolor(128, 128, 128),
            url.truecolor(128, 128, 128)
        );
    }

    fn log_response(response: &Response) {
        if response.status().is_success() {
            println!("{}", response.status().to_string().green());
        } else {
            eprintln!("{}", response.status().to_string().red());
        }
    }
}

/// Creates a new reqwest client with appropriate headers and configuration
pub(crate) fn create_client(base_url: String, token: String) -> Result<NetworkClient> {
    let mut headers = HeaderMap::new();
    let mut authorization = HeaderValue::from_str(&format!("Bearer {}", token))?;
    authorization.set_sensitive(true);
    headers.insert(AUTHORIZATION, authorization);

    let client = reqwest::blocking::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .default_headers(headers)
        .build()?;

    Ok(NetworkClient { client, base_url })
}

/// Checks if an attachment already exists in Polarion
pub(crate) fn is_attachment_existing(
    document: &Document,
    name: &str,
    client: &NetworkClient,
) -> Result<bool> {
    let id = document.get_attachment_id(name);
    let attachments = get_attachments(document, client)?;
    Ok(attachments.iter().any(|a| a.id == id))
}

/// Fetches all existing attachments for a given document
pub(crate) fn get_attachments(
    document: &Document,
    client: &NetworkClient,
) -> Result<Vec<get_attachments::Attachment>> {
    let url_path = format!(
        "/projects/{}/spaces/{}/documents/{}/attachments",
        document.project, document.space, document.name
    );

    let mut url = Some(client.create_url(&url_path));
    let mut attachments: Vec<get_attachments::Attachment> = vec![];

    while url.is_some() {
        let response = client.get_url(url.unwrap())?;
        let mut json = response.json::<get_attachments::Response>()?;
        attachments.append(&mut json.data);
        url = json.links.and_then(|l| l.next);
    }

    Ok(attachments)
}

/// Creates a new attachments by uploading a file
pub(crate) fn post_attachment(
    path: &PathBuf,
    document: &Document,
    name: &str,
    client: &NetworkClient,
) -> Result<()> {
    let url_path = format!(
        "/projects/{}/spaces/{}/documents/{}/attachments",
        document.project, document.space, document.name
    );

    let mut form = reqwest::blocking::multipart::Form::new();

    let resource = to_string(&post_attachments::Resource::create(name))?;
    form = form.text("resource", resource);

    let file = reqwest::blocking::multipart::Part::file(path)?.file_name(name.to_string());
    form = form.part("files", file);

    client.post(&url_path, Some(form))?.error_for_status()?;

    Ok(())
}

/// Replaces an existing attachment by uploading a file
pub(crate) fn patch_attachment(
    path: &PathBuf,
    document: &Document,
    name: &str,
    client: &NetworkClient,
) -> Result<()> {
    let url_path = format!(
        "/projects/{}/spaces/{}/documents/{}/attachments/{}",
        document.project,
        document.space,
        document.name,
        utf8_percent_encode(name, PATH_COMPONENT).collect::<String>()
    );

    let mut form = reqwest::blocking::multipart::Form::new();

    let resource = to_string(&patch_attachment::Resource::create(
        &document.get_attachment_id(name),
        name,
    ))?;
    form = form.text("resource", resource);

    let file = reqwest::blocking::multipart::Part::file(path)?.file_name(name.to_string());
    form = form.part("files", file);

    client.patch(&url_path, Some(form))?.error_for_status()?;

    Ok(())
}
