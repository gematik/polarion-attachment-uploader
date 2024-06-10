[![Lint](https://github.com/gematik/polarion-attachment-uploader/actions/workflows/lint.yml/badge.svg)](https://github.com/gematik/polarion-attachment-uploader/actions/workflows/lint.yml)

# Polarion Attachment Uploader

A command line tool for uploading images into Polarion as attachments.

## Usage

The uploader only works while on the VPN.

First, make `POLARION_API` and `POLARION_TOKEN` available in your shell environment, e.g. by exporting
them in your `~/.zshenv`.

```zsh
export POLARION_API=.../polarion/rest/v1
export POLARION_TOKEN=...
```

Next, create a mapping of folder names to Polarion documents in a `polarion-documents.json`
file in your [user-level configuration directory].

As an example, the following configuration file will map images beneath a folder named `folder-1` to
the Polarion document `project-1/space-1/name-1`.

```json
{
  "folder-1": {
    "project": "project-1",
    "space": "space-1",
    "name": "name-1"
  },
  "folder-2": ...
}
```

To run the uploader from a git checkout, invoke it through `cargo` and specify the images to upload
as arguments.

For each supplied image, the uploader will walk up the directory tree and look for a matching Polarion
document at each level. The traversal ends when either a matching document or the root of a git
repository is found. The attachment file name is composed from the relative path below the document's
folder by replacing path separators with `___`.

For example, using the configuration above, the following command will attempt to upload the attachments
`image.png` and `bar___image.png` into the document `project-1/space-1/name-1`.

```
cargo run foo/folder-1/image.png foo/folder-1/bar/image.png
```

## Development

To format and lint locally, run

```
cargo fmt
cargo clippy --all-targets --all-features
```

[user-level configuration directory]: https://docs.rs/dirs/latest/dirs/fn.config_dir.html
