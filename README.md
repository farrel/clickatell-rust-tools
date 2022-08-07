# Clickatell Tools for Rust

Clickatell Tools for Rust is a collection of clients for interacting with messaging gateways from [Clickatell](http://www.clickatell.com) using Rust.

It currently consists of 

* `clickatell-api` - Crate for interacting with Clickatell messaging gateways from Rust applications 
* `clikcatell-cli` - A command line tool using clickatell-api to send SMS and Whatsapp messages, check their status and retrieve account balances

## Clickatell API Example

```rust,ignore
use clickatell_api::one_api::{message::Channel,send_messages, Client};

let client = Client::new(api_key)?;

let mut request = send_messages::Request::new();

request.add_message(Channel::SMS, number, "This is message one")?;
request.add_message(Channel::SMS, number, "This is message two")?;

let response = client.send_messages(request).await?;

for response in response.messages() {
  println!("Messge ID: {} - {:?}", response.to, response.message_api_id);
}
```

## Features

* Send text messages via SMS
* Query message status
* Account balance

## Untested Features

* Send text messages via Whatsapp

## Features To Implement

* Blocking version of the One API gateway client.
* Media upload, download and metadata
* Send media via Whatsapp
* Message read notifications
* Whatsapp markup

## License
Copyright 2022 Farrel Lifson

Released under the MIT License. See LICENSE-MIT for details.
