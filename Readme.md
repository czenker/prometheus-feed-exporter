# Prometheus Feed Exporter

Parses and filters feeds and converts them into metrics.

This tool was created to watch certain feeds and release feeds
for security relevant fixes and alert a maintenance team of them.
But it can be used for many other use cases too.

Feed types supported by the library [feed-rs](https://github.com/feed-rs/feed-rs)
include RSS, Atom and JSON. All entries can be filtered with the
[Common Expression Language](https://github.com/google/cel-spec/blob/master/doc/intro.md).

## Build

It should work with Docker too. If `relabel=shared` causes issues, remove it for Docker.

````bash
podman build -f Containerfile -t prometheus-feed-exporter:latest .
````


## Todos

* Request Headers
* read config (especially request headers) from env
* Cli
* Debug why entry was taken or not (debug command)
* error handling
  * no valid XML
  * 404
  * etc
* Github Actions
* async
