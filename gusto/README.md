# `gusto-api`

A fully generated & opinionated API client for the Gusto API.

[![docs.rs](https://docs.rs/gusto-api/badge.svg)](https://docs.rs/gusto-api)

## API Details

Welcome to Gusto's API documentation.

[API Terms of Service](https://gusto.com/about/terms/developer-terms-of-service)

### Contact


| name | email |
|----|----|
| Developer Relations | developer@gusto.com |



## Client Details



The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
gusto-api = "2.1.16"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```rust,no_run
use gusto_api::Client;

let client = Client::new(
    String::from("client-id"),
    String::from("client-secret"),
    String::from("redirect-uri"),
    String::from("token"),
    String::from("refresh-token"),
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `GUSTO_CLIENT_ID`
- `GUSTO_CLIENT_SECRET`
- `GUSTO_REDIRECT_URI`

And then you can create a client from the environment.

```rust,no_run
use gusto_api::Client;

let client = Client::new_from_env(String::from("token"), String::from("refresh-token"));
```
