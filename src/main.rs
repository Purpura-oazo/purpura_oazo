use std::process::exit;

use clap::Command;
use clap_builder::{crate_description, crate_version, Arg};
use model::discord::Client as DSClient;
use once_cell::sync::Lazy;
use serenity::prelude::*;
use surrealdb::engine::remote::ws::Client as WSClient;
use surrealdb::Surreal;
use tracing::{error, span, Level};
use tracing::log::debug;

mod model;

static DB: Lazy<Surreal<WSClient>> = Lazy::new(Surreal::init);

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt()
        .pretty()
        .with_thread_names(true)
        // enable everything
        .with_max_level(Level::DEBUG)
        // sets this to be the default, global collector for this application.
        .init();
  
  let matches = Command::new("purpura_oazo")
    .version(crate_version!())
    .about(crate_description!())
    .subcommands([Command::new("start")
      .args([
        Arg::new("discord_token")
          .env("DISCORD_TOKEN")
          .required(true)
          .help("The discord token")
          .long_help("The discord token, e.g. `<KEY>`"),

        Arg::new("rabbitmq_host")
          .env("RABBITMQ_HOST")
          .required({
            #[cfg(not(debug_assertions))]
            let owo = true;
            #[cfg(debug_assertions)]
            let owo = false;

            owo
          })
          .help("The address of the rabbitmq server")
          .long_help("The address of the rabbitmq server, without the port, e.g. `rabbitmq.example.com`"),

        Arg::new("rabbitmq_port")
          .env("RABBITMQ_PORT")
          .required({
            #[cfg(not(debug_assertions))]
            let owo = true;
            #[cfg(debug_assertions)]
            let owo = false;

            owo
          })
          .help("The port of the rabbitmq server")
          .long_help("The port of the rabbitmq server, e.g. `5672`"),

        Arg::new("rabbitmq_username")
          .env("RABBITMQ_USERNAME")
          .required({
            #[cfg(not(debug_assertions))]
            let owo = true;
            #[cfg(debug_assertions)]
            let owo = false;

            owo
          })
          .help("The username used to connect to the rabbitmq server")
          .long_help("The username used to connect to the rabbitmq server, e.g. `guest`"),

        Arg::new("rabbitmq_password")
          .env("RABBITMQ_PASSWORD")
          .required({
            #[cfg(not(debug_assertions))]
            let owo = true;
            #[cfg(debug_assertions)]
            let owo = false;

            owo
          })
          .help("The password used to connect to the rabbitmq server")
          .long_help("The password used to connect to the rabbitmq server, e.g. `owo69`"),
      ])
      .long_about("Launches x numbers of shards to handle discord events where x is the number of cores on the machine")
      .about("Starts the bot")])
    .subcommand_required(true)
    .get_matches();

  #[cfg(debug_assertions)]
  debug!("Data : {:?}", matches);
  match matches
    .subcommand_name()
    .expect("A subcommand was not specified")
  {
    "start" => {
      debug!("Initializing client");
      let c = matches.subcommand().unwrap().1;
      let client = DSClient::new(c).await;

      let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::GUILD_MESSAGE_TYPING;

      let num = num_cpus::get() as u32;
      debug!("Building shards from 1 to {}", num);
      if let Err(why) = Client::builder(c.get_one::<String>("discord_token").unwrap(), intents)
        .event_handler(client)
        .await
        .unwrap()
        .start_shard_range(0..num-1, num)
        .await
      {
        error!("Event handler failed to start because of : {why}");
        exit(1);
      }
    }
    _ => unreachable!(),
  }
}
