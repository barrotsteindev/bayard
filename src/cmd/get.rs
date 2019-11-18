use clap::ArgMatches;

use crate::client::client::{create_client, Clerk};
use crate::util::log::set_logger;

pub fn run_get_cli(matches: &ArgMatches) -> Result<(), String> {
    set_logger();

    let servers: Vec<_> = matches
        .values_of("SERVERS")
        .unwrap()
        .map(|addr| create_client(addr))
        .collect();
    let key = matches.value_of("KEY").unwrap();

    let client_id = rand::random();

    let mut client = Clerk::new(&servers, client_id);
    let value = client.get(key);
    print!("{}", value);

    Ok(())
}
