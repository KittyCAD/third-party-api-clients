# `hubspot-tickets`

A fully generated & opinionated API client for the Hubspot Tickets API.

[![docs.rs](https://docs.rs/hubspot-tickets/badge.svg)](https://docs.rs/hubspot-tickets)

## API Details








## Client Details



The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
hubspot-tickets = "0.1.0"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```rust,no_run
use hubspot_tickets::Client;

let client = Client::new(
    String::from("api-key"),
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `HUBSPOT_TICKETS_API_TOKEN`


And then you can create a client from the environment.

```rust,no_run
use hubspot_tickets::Client;

let client = Client::new_from_env();
```
