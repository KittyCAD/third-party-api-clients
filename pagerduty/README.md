# `pagerduty-api`

A fully generated & opinionated API client for the PagerDuty API.

[![docs.rs](https://docs.rs/pagerduty-api/badge.svg)](https://docs.rs/pagerduty-api)

## API Details

This document describes the PagerDuty REST APIs.

For guides and examples please visit our [Documentation.](https://developer.pagerduty.com/docs/get-started/getting-started/)

Our REST APIs are defined in OpenAPI v3.x. You can view the schema at [github.com/PagerDuty/api-schema](https://github.com/PagerDuty/api-schema).

Note that properties in some schemas have fields not shown by default such as `readOnly`, `format`, and `default`. Hover your cursor over the right column that looks like `optional+1` to see the full list of fields.




### Contact


| name | url | email |
|----|----|----|
| PagerDuty Support | <http://www.pagerduty.com/support> | support@pagerduty.com |



## Client Details



The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
pagerduty-api = "0.0.1"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```rust,no_run
use pagerduty_api::Client;

let client = Client::new(
    String::from("api-key"),
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `PAGERDUTY_API_TOKEN`

And then you can create a client from the environment.

```rust,no_run
use pagerduty_api::Client;

let client = Client::new_from_env();
```
