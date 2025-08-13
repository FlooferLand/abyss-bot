use std::collections::HashSet;
use chrono::Utc;
use crate::logger::Logger;
use dotenvy::dotenv;
use log::{error, info};
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{ActivityData, OnlineStatus, UserId};
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use crate::commands::info::bot_info;
use crate::commands::{post_command, pre_command};
use crate::error::{BotError, BotErrorExt};
use crate::event_handler::{error_handler, event_handler};

mod logger;
mod error;
mod commands;
mod util;
mod event_handler;

struct BotData {
    postgres: Pool<Postgres>
}
type Context<'a> = poise::Context<'a, BotData, BotError>;

#[tokio::main]
async fn main() {
    dotenv().unwrap();
    let token = std::env::var("BOT_TOKEN").expect("env 'BOT_TOKEN' should be set");
    let intents = serenity::GatewayIntents::all();
    Logger::init().unwrap();

    // Creating the poise framework instance
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            commands: vec![
                bot_info()
            ],
            pre_command: |ctx| Box::pin(pre_command(ctx)),
            post_command: |ctx| Box::pin(post_command(ctx)),
            on_error: |error| Box::pin(error_handler(error)),
            owners: {
                let mut set = HashSet::new();
                set.insert(UserId::new(792764829689315349));
                set
            },
            .. Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                // Register commands
                poise::builtins::register_globally(ctx, &framework.options().commands).await.bot_err()?;

                // Loading database
                let database = std::env::var("DATABASE_URL").expect("env 'DATABASE_URL' should be set");
                let pool = PgPoolOptions::new()
                    .max_connections(5)
                    .connect(database.as_str()).await.unwrap();

                // Status
                println!();
                info!("Bot online! ({})", Utc::now().format("%Y/%m/%d"));
                ctx.set_presence(Some(ActivityData::custom(":3")), OnlineStatus::Online);
                //ctx.set_presence(Some(ActivityData::custom("% relics collected!")), OnlineStatus::Online);

                // Data
                Ok(BotData {
                    postgres: pool
                })
            })
        })
        .build();

    // Creating the client
    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await.unwrap();

    // Graceful shutdown
    let shard_manager = client.shard_manager.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        {
            let shard_runners = shard_manager.runners.lock().await;
            for (_id, runner) in shard_runners.iter() {
                runner.runner_tx.set_status(OnlineStatus::Offline);
            }
        }
        shard_manager.shutdown_all().await;
        info!("Bot stopped!");
    });

    // Starting the client
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
