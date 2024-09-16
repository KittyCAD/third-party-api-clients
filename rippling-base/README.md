# `rippling-base-api`

A fully generated & opinionated API client for the Rippling Base API.

[![docs.rs](https://docs.rs/rippling-base-api/badge.svg)](https://docs.rs/rippling-base-api)

## API Details

Using Rippling's API requires either an API key or an access token retrieved from an OAuth exchange. Each is tied to a single Rippling Company.

If you are a partner building an integration to Rippling,you can use [Rippling's Installation Guide](https://developer.rippling.com/docs/rippling-api/fucwnbc121hiu-installation-guide) to learn how to retrieve an access token to start using Rippling APIs.

If you are a customer, you can go [here](https://developer.rippling.com/docs/rippling-api/9rw6guf819r5f-introduction-for-customers) to learn create your API keys to start using Rippling APIs.

### Using the Interactive Documentation

Rippling's Documentation Portal allows you to test the API endpoints directly within the documentation. To do so, provide your API key or Access Token as a header parameter with the form Authorization Bearer: Bearer.

[API Terms of Service](https://app.rippling.com/developer/tos)

### Contact


| name | email |
|----|----|
| Rippling Support | support@rippling.com |

### License


| name |
|----|
| MIT |


## Client Details



The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
rippling-base-api = "0.1.0"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```rust,no_run
use rippling_base_api::Client;

let client = Client::new(
    String::from("api-key"),
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `RIPPLING_BASE_API_TOKEN`

And then you can create a client from the environment.

```rust,no_run
use rippling_base_api::Client;

let client = Client::new_from_env();
```
