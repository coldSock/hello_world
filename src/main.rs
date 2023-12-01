mod commands;

use dotenv::dotenv;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use chrono::offset::Utc;
use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::gateway::{Ready, ActivityType, GatewayIntents};
use serenity::gateway::ActivityData;
use serenity::builder::{
    CreateInteractionResponse, 
    CreateInteractionResponseMessage, 
    CreateMessage, 
    CreateEmbed, 
    CreateEmbedAuthor, 
    CreateEmbedFooter
};
use serenity::framework::standard::macros::{command, group};
use serenity::model::application::{Command, Interaction};
use serenity::model::id::{ChannelId, GuildId};
use serenity::framework::standard::{StandardFramework, CommandResult};
use serenity::model::channel::ReactionType;

#[group]
#[commands(ping)]
struct General;

struct Handler {
    is_loop_running: AtomicBool,
}


#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "ping" => Some(commands::ping::run(&command.data.options())),
                "id" => Some(commands::id::run(&command.data.options())),
                "attachmentinput" => Some(commands::attachmentinput::run(&command.data.options())),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }
    async fn message(&self, context: Context, msg: Message) {
        if msg.content.starts_with("!") && msg.channel_id == ChannelId::new(1173231671718449265_u64) {
            // If the `utils`-feature is enabled, then model structs will have a lot of useful
            // methods implemented, to avoid using an often otherwise bulky Context, or even much
            // lower-level `rest` method.
            //
            // In this case, you can direct message a User directly by simply calling a method on
            // its instance, with the content of the message.
            // create a timer for when the message was created and when the embed was sent
            let mut xs: Vec<&str> = msg.content.split_whitespace().collect();
            if xs.len() < 2 {
                return;
            }
            let index = xs.iter().position(|x| *x == "!").unwrap();
            xs.remove(index);

            // let reaction = msg.react(&context, '✅').await;
            let reaction_type1 = ReactionType::Custom { animated: (false), id: 1178476355160125460.into(), name: Some("tup".to_string()) };
            let reaction_type2 = ReactionType::Custom { animated: (false), id: 1178476382842519724.into(), name: Some("tdown".to_string()) };

            let reaction = vec![reaction_type1, reaction_type2];

            let author_a = CreateEmbedAuthor::new(&msg.author.name).name(&msg.author.name).icon_url(&msg.author.face());

            let embed_a = CreateEmbed::new()
            .author(author_a)
            .description(xs.join(" "))
            .color(0x2c2d31);

            let message_a = CreateMessage::new()
                .content("Suggestion received!")
                .embed(embed_a)
                .reactions(reaction);

            let message = ChannelId::new(1173231671718449265).send_message(&context, message_a).await;
            if let Err(why) = message {
                eprintln!("Error sending message: {why:?}");
            }

            let author_b = CreateEmbedAuthor::new(String::from("Suggestion")).name(&msg.author.name).icon_url(&msg.author.face());

            let footer_b = CreateEmbedFooter::new("Suggestion").text(format!("Suggestion from {} sent successfully. Great job!", &msg.author.name));

            let embed_b = CreateEmbed::new()
            .author(author_b)
            .footer(footer_b)
            .color(0x2c2d31);

            let message_b = CreateMessage::new()
                .embed(embed_b);

            let suggestion = msg
                .author
                .dm(&context, message_b).await;

            if let Err(why) = suggestion {
                println!("Error when direct messaging user: {why:?}");
            }

            let prune = msg.delete(&context).await;
            if let Err(why) = prune {
                println!("Error when direct messaging user: {why:?}");
            }
        }
        if msg.content.starts_with("&") && msg.channel_id == ChannelId::new(1173075205267148861_u64) {
            // // remove the & symbol from the message without splitting it
            // let mut xs: Vec<&str> = msg.content.split_whitespace().collect();
            // if xs.len() < 2 {
            //     return;
            // }
            // let index = xs.iter().position(|x| *x == "&").unwrap();
            // xs.remove(index);


            let xs: String = msg.content.replace("&", "");

        

            // let reaction = msg.react(&context, '✅').await;



            let author = CreateEmbedAuthor::new(&msg.author.name).name(&msg.author.nick_in(&context, &msg.guild_id.unwrap()).await.unwrap()).icon_url(&msg.author.face());

            let footer = CreateEmbedFooter::new("Update").text(format!("On behalf of Vexus Command, {}", &msg.author.name));

            let embed = CreateEmbed::new()
            .author(author)
            .description(xs)
            .color(0x2c2d31)
            .footer(footer);


            let message = CreateMessage::new()
                .embed(embed);

            let message = ChannelId::new(1173075205267148861).send_message(&context, message).await;
            if let Err(why) = message {
                eprintln!("Error sending message: {why:?}");
            }

            let prune = msg.delete(&context).await;
            if let Err(why) = prune {
                println!("Error when direct messaging user: {why:?}");
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        dotenv().ok();

        let guild_id = GuildId::new(
            std::env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = guild_id
            .set_commands(&ctx.http, vec![
                commands::ping::register(),
                commands::id::register(),
                commands::welcome::register(),
                commands::numberinput::register(),
                commands::attachmentinput::register(),
            ])
            .await;

        println!("I now have the following guild slash commands: {commands:#?}");

        let guild_command =
            Command::create_global_command(&ctx.http, commands::wonderful_command::register())
                .await;

        println!("I created the following global slash command: {guild_command:#?}");
    }

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        println!("Cache built successfully!");

        // It's safe to clone Context, but Arc is cheaper for this use case.
        // Untested claim, just theoretically. :P
        let ctx = Arc::new(ctx);

        // We need to check that the loop is not already running when this event triggers, as this
        // event triggers every time the bot enters or leaves a guild, along every time the ready
        // shard event triggers.
        //
        // An AtomicBool is used because it doesn't require a mutable reference to be changed, as
        // we don't have one due to self being an immutable reference.
        if !self.is_loop_running.load(Ordering::Relaxed) {
            // We have to clone the Arc, as it gets moved into the new thread.
            let ctx1 = Arc::clone(&ctx);
            // tokio::spawn creates a new green thread that can run in parallel with the rest of
            // the application.
            tokio::spawn(async move {
                loop {
                    log_system_load(&ctx1).await;
                    tokio::time::sleep(Duration::from_secs(9000)).await;
                }
            });

            // And of course, we can run more than one thread at different timings.
            let ctx2 = Arc::clone(&ctx);
            tokio::spawn(async move {
                loop {
                    set_activity_to_current_time(&ctx2).await;
                    tokio::time::sleep(Duration::from_secs(60)).await;
                }
            });

            // Now that the loop is running, we set the bool to true
            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
    }
}

async fn log_system_load(ctx: &Context) {
    let cpu_load = sys_info::loadavg().unwrap();
    let mem_use = sys_info::mem_info().unwrap();

    // We can use ChannelId directly to send a message to a specific channel; in this case, the
    // message would be sent to the #testing channel on the discord server.

    let embed_obj = CreateEmbed::new()
    .title("System Resource Load")
    .field("CPU Load Average / per 900 tics", format!("<:Asset5:1177282027066757170> {:.2}%\n", cpu_load.fifteen * 10.0), false)
    .field(
    "Memory Usage",
    format!(
        "<:Asset5:1177282027066757170> {:.2} MB Free out of {:.2} MB\n",
        mem_use.free as f32 / 1000.0,
        mem_use.total as f32 / 1000.0), 
        false,
    )
    .field("CPU Load Average / per 60 tics", format!("<:Asset5:1177282027066757170> {:.2}%\n", cpu_load.one * 10.0), false)
    .color(0x2c2d31);

    let embed = CreateMessage::new()
        .embed(embed_obj);
    
    let message = ChannelId::new(1173231671718449265_u64).send_message(&ctx, embed).await;
    if let Err(why) = message {
        eprintln!("Error sending message: {why:?}");
    }
    }


async fn set_activity_to_current_time(ctx: &Context) {
    use serenity::model::user::OnlineStatus;

    let current_time = Utc::now();
    let formatted_time = current_time.to_rfc2822();

    let activity = ActivityData {
        name: format!("with time: {}", formatted_time),
        kind: ActivityType::Custom,
        state: Some(String::from("I'm a bot!")),
        url: None,
    };
    let status = OnlineStatus::Idle;

    ctx.set_presence(Some(activity), status);
}


#[tokio::main]
async fn main() {
    dotenv().ok();
    let framework = StandardFramework::new().group(&GENERAL_GROUP);
    
    // Login with a bot token from the environment
    let token = std::env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler {
            is_loop_running: AtomicBool::new(false),
        })
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    // if let Err(why) = client.start().await {
    //     println!("An error occurred while running the client: {:?}", why);
    // }
    if let Err(why) = client.start_shards(1).await {
        println!("Client error: {why:?}");
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    // Respond with pong and ping time in milliseconds

    msg.reply(ctx, "Pong!").await?;
    // msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}