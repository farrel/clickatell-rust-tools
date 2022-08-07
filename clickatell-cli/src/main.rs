mod cli;

use clap::ArgMatches;
use clickatell_api::one_api::{message_status, send_messages, BlockingClient, Channel};

use std::process;

pub const NUMBER: &str = "number";
pub const MESSAGE: &str = "message";
pub const SMS: &str = "sms";
pub const WHATSAPP: &str = "whatsapp";

fn send_message(client: BlockingClient, command_line_arguments: &ArgMatches) {
  let number = command_line_arguments
    .get_one::<String>(NUMBER)
    .expect("ERROR: No number given");

  let message = command_line_arguments
    .get_one::<String>(MESSAGE)
    .expect("ERROR: No message given");

  let channel = if command_line_arguments.contains_id(SMS) {
    Channel::SMS
  } else {
    Channel::WhatsApp
  };

  let mut message_request = send_messages::Request::new();

  if let Err(error) = message_request.add_message(channel, number, message) {
    eprintln!("ERROR: Could not add message to request - {}", error);
    process::exit(exitcode::SOFTWARE);
  }

  let send_message_response = match client.send_messages(message_request) {
    Ok(message_reponse) => message_reponse,
    Err(error) => {
      eprintln!("ERROR: Could not send message - {}", error);
      process::exit(exitcode::UNAVAILABLE);
    }
  };

  if let Some(error) = send_message_response.error {
    eprintln!(
      "ERROR: Request could not be processed: {} ({})",
      error.description, error.code
    );
    process::exit(exitcode::UNAVAILABLE);
  }

  for message in send_message_response.messages() {
    match message.error {
      Some(error) => eprintln!(
        "ERROR: Message could not be processed: {} : {} ({})",
        message.to, error.description, error.code
      ),
      None => println!("SUCCESS: {} : {}", message.to, message.message_id()),
    }
  }
}

pub const MESSAGE_ID: &str = "message-id";

fn message_status(client: BlockingClient, command_line_arguments: &ArgMatches) {
  let message_id = command_line_arguments
    .get_one::<String>(MESSAGE_ID)
    .expect("No Message ID given");

  let message_status_request = message_status::Request::new(message_id);

  let message_status_response = client
    .message_status(message_status_request)
    .expect("Error retrieving message status");

  match message_status_response.channel() {
    Channel::Unknown => {
      eprintln!("ERROR: {} : Could not retrieve status", message_id);
      process::exit(exitcode::UNAVAILABLE);
    }
    _ => {
      println!(
        "STATUS: {} : {} : {:?} : {}",
        message_id,
        message_status_response.channel(),
        message_status_response.status(),
        message_status_response.updated_at()
      );
    }
  }
}

fn balance(client: BlockingClient, _command_line_arguments: &ArgMatches) {
  let response = client.balance();
  match response {
    Ok(balance_response) => println!(
      "BALANCE: {} {}",
      balance_response.currency, balance_response.balance
    ),

    Err(error) => {
      eprintln!("ERROR: {}", error);
      process::exit(exitcode::UNAVAILABLE);
    }
  }
}

pub const API_KEY: &str = "api-key";

fn get_api_key(command_line_arguments: &ArgMatches) -> String {
  match (
    command_line_arguments.get_one::<String>(API_KEY),
    std::env::var(CLICKATELL_API_KEY),
  ) {
    (Some(api_key), _) => api_key.to_string(),
    (_, Ok(api_key)) => api_key,
    _ => {
      eprintln!("ERROR: Clickatell API key is not set. It must be set in the CLICKATELL_API_KEY environment variable or via the -a command line parameter.");
      process::exit(exitcode::USAGE);
    }
  }
}

pub const CLICKATELL_API_KEY: &str = "CLICKATELL_API_KEY";
pub const SEND_MESSAGE: &str = "send-message";
pub const MESSAGE_STATUS: &str = "message-status";
pub const BALANCE: &str = "balance";

fn main() {
  let command_line_arguments = crate::cli::parse_args();

  let (command, command_arguments) = match command_line_arguments.subcommand() {
    Some(command_and_arguments) => command_and_arguments,
    None => {
      eprintln!("ERROR: No command given");
      process::exit(exitcode::USAGE);
    }
  };

  let api_key = get_api_key(command_arguments);
  let client = BlockingClient::new(&api_key).expect("Could not build client");

  match command {
    SEND_MESSAGE => send_message(client, command_arguments),
    MESSAGE_STATUS => message_status(client, command_arguments),
    BALANCE => balance(client, command_arguments),
    unknown_command => {
      eprintln!("ERROR: Unhandled command '{}'", unknown_command);
      process::exit(exitcode::USAGE);
    }
  }
}
