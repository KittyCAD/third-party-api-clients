# `commonroom-api`

A fully generated & opinionated API client for the Common Room API.

[![docs.rs](https://docs.rs/commonroom-api/badge.svg)](https://docs.rs/commonroom-api)

## API Details

Common Room Community REST APIs for accessing Community scoped resources.
<br/><br/>
To use the Common Room API, or get started with the Common Room Zapier integration, you will need to create an API token.
To create an API token:
<ol>
  <li>Navigate to Setting | API tokens
  <li>Click on “Request Access to API”. Our API is currently in beta, so you need to first apply for access.
  <li>Once access is granted, return to the Settings | API tokens screen, and you will see a button to create a “New Token”.
</ol>

# Authentication

<!-- ReDoc-Inject: <security-definitions> -->






## Client Details



The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
commonroom-api = "0.1.0"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```rust,no_run
use commonroom_api::Client;

let client = Client::new(
    String::from("api-key"),
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `COMMONROOM_API_TOKEN`


And then you can create a client from the environment.

```rust,no_run
use commonroom_api::Client;

let client = Client::new_from_env();
```
