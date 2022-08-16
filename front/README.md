# `front-api`

A fully generated & opinionated API client for the Front API.

[![docs.rs](https://docs.rs/front-api/badge.svg)](https://docs.rs/front-api)

## API Details





### Contact


| name | email |
|----|----|
| Front Platform | api@frontapp.com |



## Client Details



The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
front-api = "0.0.2"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```rust,no_run
use front_api::Client;

let client = Client::new(
    String::from("api-key"),
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `FRONT_API_TOKEN`

And then you can create a client from the environment.

```rust,no_run
use front_api::Client;

let client = Client::new_from_env();
```
