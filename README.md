<img align="right" width="250" height="47" src="gematik.png"/>

# Polarion Attachment Uploader

A command line tool for uploading images into Polarion as attachments.

[![Lint](https://github.com/gematik/polarion-attachment-uploader/actions/workflows/lint.yml/badge.svg)](https://github.com/gematik/polarion-attachment-uploader/actions/workflows/lint.yml)

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

## License

Copyright 2024 gematik GmbH

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in
compliance with the License.

See the [LICENSE](./LICENSE) for the specific language governing permissions and limitations under
the License.

Unless required by applicable law the software is provided "as is" without warranty of any kind,
either express or implied, including, but not limited to, the warranties of fitness for a particular
purpose, merchantability, and/or non-infringement. The authors or copyright holders shall not be
liable in any manner whatsoever for any damages or other claims arising from, out of or in connection
with the software or the use or other dealings with the software, whether in an action of contract,
tort, or otherwise.

The software is the result of research and development activities, therefore not necessarily quality
assured and without the character of a liable product. For this reason, gematik does not provide any
support or other user assistance (unless otherwise stated in individual cases and without justification
of a legal obligation). Furthermore, there is no claim to further development and adaptation of the
results to a more current state of the art.

Gematik may remove published results temporarily or permanently from the place of publication at any
time without prior notice or justification.
