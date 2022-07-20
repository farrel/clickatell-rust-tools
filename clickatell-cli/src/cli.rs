use crate::{BALANCE, SEND_MESSAGE, MESSAGE_STATUS, NUMBER, MESSAGE_ID, MESSAGE, SMS, WHATSAPP, API_KEY};
use clap::{Arg, ArgMatches, Command};

pub(crate) fn parse_args() -> ArgMatches {
    let api_key_args = 
            Arg::with_name(API_KEY)
                .short('a')
                .long("api-key")
                .takes_value(true)
                .help("Clickatell API Key - not required if the CLICKATELL_API_KEY environment variable is set but will override the value in that environment variable if given.");

    Command::new("clickatell-cli")
        .subcommand_required(true)
        .subcommand(
            Command::new(SEND_MESSAGE).about("Send a message to the specified number")
                .arg(&api_key_args)
                .arg(
                    Arg::with_name(SMS)
                        .short('s')
                    .long("sms")
                        .help("Send via SMS")
                        .conflicts_with(WHATSAPP),
                )
                .arg(
                    Arg::with_name(WHATSAPP)
                        .short('w')
                        .long("whatsapp")
                        .help("Send via WhatsApp")
                        .conflicts_with(SMS),
                )
                .arg(
                    Arg::with_name(NUMBER)
                        .short('n')
                        .long("number")
                        .required(true)
                        .takes_value(true).help("The destination number"),
                )
                .arg(
                    Arg::with_name(MESSAGE)
                        .short('m')
                        .long("message")
                        .required(true)
                        .takes_value(true).help("The message to send"),
                ),
        )
        .subcommand(
            Command::new(MESSAGE_STATUS).about("Retrieve the status of a message").arg(&api_key_args).arg(
                Arg::with_name(MESSAGE_ID)
                    .short('m')
                    .long("message-id")
                    .required(true)
                    .takes_value(true).help("The Message ID returned if a message is sent successfully")
            ),
        )
        .subcommand(Command::new(BALANCE).about("Return your account balance").arg(&api_key_args))
        .get_matches()
}
