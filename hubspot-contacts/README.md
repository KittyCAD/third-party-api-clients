# `hubspot-contacts`

A fully generated & opinionated API client for the Hubspot Contacts API.

[![docs.rs](https://docs.rs/hubspot-contacts/badge.svg)](https://docs.rs/hubspot-contacts)

## API Details

API description.






## Client Details



The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
hubspot-contacts = "0.1.2"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```rust,no_run
use hubspot_contacts::Client;

let client = Client::new(
    String::from("api-key"),
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `HUBSPOT_CONTACTS_API_TOKEN`


And then you can create a client from the environment.

```rust,no_run
use hubspot_contacts::Client;

let client = Client::new_from_env();
```
