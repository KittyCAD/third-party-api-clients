# `ramp-api`

A fully generated & opinionated API client for the ramp API.

[![docs.rs](https://docs.rs/ramp-api/badge.svg)](https://docs.rs/ramp-api)

## API Details








## Client Details



The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
ramp-api = "0.0.2"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```rust,no_run
use ramp_api::Client;

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

- `RAMP_CLIENT_ID`
- `RAMP_CLIENT_SECRET`
- `RAMP_REDIRECT_URI`

And then you can create a client from the environment.

```rust,no_run
use ramp_api::Client;

let client = Client::new_from_env(String::from("token"), String::from("refresh-token"));
```
