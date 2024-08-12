# `vercel-api`

A fully generated & opinionated API client for the Vercel API.

[![docs.rs](https://docs.rs/vercel-api/badge.svg)](https://docs.rs/vercel-api)

## API Details

Vercel combines the best developer experience with an obsessive focus on end-user performance. Our platform enables frontend teams to do their best work.



### Contact


| name | url | email |
|----|----|----|
| Vercel Support | <https://vercel.com/support> | support@vercel.com |



## Client Details



The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
vercel-api = "0.1.0"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```rust,no_run
use vercel_api::Client;

let client = Client::new(
    String::from("api-key"),
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `VERCEL_API_TOKEN`

And then you can create a client from the environment.

```rust,no_run
use vercel_api::Client;

let client = Client::new_from_env();
```
