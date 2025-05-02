mod askllama;

use std::env;
use serenity::prelude::*;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use dotenv::dotenv;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        // Check if the bot is mentioned
        if msg.mentions_me(&ctx.http).await.unwrap_or(false) {
            let content = msg.content.clone();

            // Use askllama to get a response
            match askllama::ask(&content).await {
                Ok(response) => {
                    if let Err(e) = msg.reply(&ctx.http, response).await {
                        println!("Error sending message: {:?}", e);
                    }
                },
                Err(e) => {
                    println!("Error getting response: {:?}", e);
                    // Clone the error message before the await to avoid Send issue
                    let error_msg = format!("{:?}", e);
                    // Drop the error object before the await
                    drop(e);
                    if let Err(e) = msg.reply(&ctx.http, "Sorry, I couldn't process that request.").await {
                        println!("Error sending error message: {:?}", e);
                        println!("Original error: {}", error_msg);
                    }
                }
            }
        }
    }
}

fn main() {
    // Configure the client
    // with your Discord bot token in the environment.
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new Runtime and block on our async code
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed to create runtime");
    rt.block_on(async {
        // Create a new instance of the Client, logging in as a bot
        let mut client = Client::builder(&token, intents)
            .event_handler(Handler)
            .await
            .expect("Err creating client");

        // Start the client
        if let Err(why) = client.start().await {
            println!("Client error: {:?}", why);
        }
    });
}
