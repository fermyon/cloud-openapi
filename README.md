# cloud-openapi

This repository contains the Cloud OpenAPI specification and related artifacts.

The OpenAPI specification defines a machine-readable schema for describing HTTP
APIs. From an OpenAPI specification, clients and servers for your project can
be generated in a number of programming languages.

The latest version of the OpenAPI specification for the Cloud can be found in this
repository at swagger.json. This file is itself a generated file and should not
be edited directly. You can use this file to generate a client for the Cloud
HTTP API in the language of your choice.

## Using the Rust client

To use the Rust client, add a reference to it in your Cargo.toml:

```
cloud-openapi = { git = "https://github.com/fermyon/cloud-openapi" }
```

## Code Generation

This repository uses the OpenAPI Generator project to generate clients that is
then used to validate the generated specification.

A `swagger.json`` can be fetched from Fermyon Cloud by running the following commands:

```sh
rm swagger.json
make swagger.json
```

To fetch the swagger of a locally running instance of Fermyon Cloud, configure the `SWAGGER_ENDPOINT`:

```sh
rm swagger.json
SWAGGER_ENDPOINT=http://localhost:5309/swagger/v1/swagger.json make swagger.json
```