use amqprs::{connection::{Connection, OpenConnectionArguments}, channel::Channel};
use clap::ArgMatches;
use serenity::{all::Ready, async_trait, client::EventHandler, prelude::*};
use tracing::{info, log::debug};

pub struct Client {
  scheduler: Channel,
}

impl Client {
  pub async fn new(c: &ArgMatches) -> Self {
    debug!("Connecting to RabbitMQ");
    let scheduler = Connection::open(&OpenConnectionArguments::new(
      c.get_one::<String>("rabbitmq_host").unwrap_or(&"localhost".to_string()),
      c.get_one::<u16>("rabbitmq_port").unwrap_or(&5672).to_owned(),
      c.get_one::<String>("rabbitmq_username").unwrap_or(&String::from("guest")),
      c.get_one::<String>("rabbitmq_password").unwrap_or(&String::from("guest")),
    ))
    .await
    .unwrap();
    debug!("Opening a channel");
    let channel = scheduler.open_channel(None).await.unwrap();
    debug!("Client ready");
    Client { scheduler: channel }
  }
}

#[async_trait]
impl EventHandler for Client {
  async fn shards_ready(&self, ctx: Context, total_shards: u32) {
    info!("{} shards ready", total_shards);
  }

  async fn ready(&self, ctx: Context, data_about_bot: Ready) {
    info!(
      "Logged in as {}#{}",
      data_about_bot.user.name,
      data_about_bot.user.discriminator.unwrap().to_string()
    );
  }
}
