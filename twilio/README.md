# `twilio-api`

A fully generated & opinionated API client for the Twilio API.

[![docs.rs](https://docs.rs/twilio-api/badge.svg)](https://docs.rs/twilio-api)

## API Details

This is the public Twilio REST API.

[API Terms of Service](https://www.twilio.com/legal/tos)

### Contact


| name | url | email |
|----|----|----|
| Twilio Support | <https://support.twilio.com> | support@twilio.com |

### License


| name | url |
|----|----|
| Apache 2.0 | <https://www.apache.org/licenses/LICENSE-2.0.html> |


## Client Details



The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
twilio-api = "0.1.0"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```rust,no_run
use twilio_api::Client;

let client = Client::new(
    String::from("username"),
    String::from("password"),
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `TWILIO_USERNAME`
- `TWILIO_PASSWORD`

And then you can create a client from the environment.

```rust,no_run
use twilio_api::Client;

let client = Client::new_from_env();
```
