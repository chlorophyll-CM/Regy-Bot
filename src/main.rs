use poise::async_trait;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::ChannelId;
use poise::serenity_prelude::CreateEmbed;
use poise::serenity_prelude::EventHandler;
use poise::serenity_prelude::Message;
use poise::serenity_prelude::Reaction;
use poise::serenity_prelude::ReactionType;
use poise::serenity_prelude::Ready;
use poise::serenity_prelude::UserId;

/*
struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}
*/

mod commands;
mod utils;

use std::path::Path;
use regex::Regex;

use crate::utils::toml::*;
use crate::utils::logger::*;
//use crate::commands::dev::*;
//use crate::commands::staff::*;
use crate::commands::user::*;

struct Handler;

pub struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: poise::serenity_prelude::Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: poise::serenity_prelude::Context, msg: Message) {
        let content = msg.content.chars().rev().collect::<String>();
        if !content.is_empty() {
            //Ignores messages from bots
            if msg.author.bot {
                return;
            }

            //Reply to dm messages
            if msg.guild_id.is_none() {
                msg.reply(ctx, "I wish I could dm you but because to my new fav Discord Developer Compliance worker named Gatito I cant. :upside_down: Lots of to you :heart:").await.expect("Unable to reply to dm");
                return;
            }
            
            //Reply to pings
            if msg.mentions_user_id(ctx.cache.current_user_id()) {
                let ctx = ctx.clone();
                msg.reply(ctx, "To use Regy use the prefix `<|`").await.expect("Unable to reply to ping");
            }

            //Ignores moderation from devs
            if msg.author.id == 687897073047306270 || msg.author.id == 598280691066732564 {
                return;
            }

            //Ignores moderation from staff
            for staff in get_config().staff {
                if msg.author.id == UserId(staff.parse::<u64>().unwrap()) {
                    return;
                }
            }

            let list_block_phrases = list_block_phrases();

            for (_id, phrase) in list_block_phrases {
                let re = Regex::new(&phrase).unwrap();
                if re.is_match(&msg.content) {
                    if let Err(why) = msg.delete(&ctx.http).await {
                        println!("Error deleting message: {:?}", why);
                    }

                    let temp_msg_content = format!("<@{}> You are not allowed to send that due to the server setup regex rules", msg.author.id).to_string();
                    let temp_msg = msg.channel_id.say(&ctx.http, temp_msg_content).await.expect("Unable to send message");
                    let ctx_clone = ctx.clone();
                    std::thread::spawn(move || {
                        std::thread::sleep(std::time::Duration::from_secs(5));
                        let _ = temp_msg.delete(&ctx_clone.http);
                    });

                    msg.author.dm(&ctx.http, |m| m.content("You are not allowed to send that due to the server setup regex rules, this has been reported to the server staff, continued infractions will result in greater punishment.")).await.expect("Unable to dm user");
                    let log_channel = ChannelId(get_config().log_channel);

                    let mut embed = CreateEmbed::default();
                    embed.color(0xFFA500);
                    embed.title("Message blocked due to matching a set regex pattern");
                    embed.field("The user who broke a regx pattern is below:", format!("<@{}>", msg.author.id), false);
                    embed.field("Their message is the following below:", format!("||{}||", msg.content), false);
                    embed.footer(|f| f.text("React with 🚫 to dismiss this infraction"));
                    let embed_message_id = log_channel.send_message(&ctx.http, |m| m.set_embed(embed)).await.expect("Unable to send embed").id;
                    let embed_message = log_channel.message(&ctx.http, embed_message_id).await.ok();
                    embed_message.unwrap().react(&ctx.http, ReactionType::Unicode("🚫".to_string())).await.ok();

                    //log_channel.say(&ctx.http, format!("<@{}> sent a message that matched a regex pattern, their message is the following below:\n||```{}```||", msg.author.id, msg.content.replace('`', "\\`"))).await.unwrap();

                    let data = LogData {
                        importance: "INFO".to_string(),
                        message: format!("{} has sent a message which is not allowed due to the set regex patterns", msg.author.id),
                    };

                    log_this(data);

                    println!("{} sent a message that matched a blocked regex pattern, their message is the following below:\n{}\n\nThere message broke the following pattern:\n{}", msg.author.id, msg.content, phrase);
                    add_infraction(msg.author.id.into());
                    return;
                }
            }            
        }
    }

    async fn reaction_add(&self, ctx: poise::serenity_prelude::Context, reaction: Reaction) {
        //Only looks in the log channel
        if reaction.channel_id != ChannelId(get_config().log_channel) {
            return;
        }

        //Only allow staff to use reactions
        if !get_config().staff.contains(&reaction.user_id.unwrap().to_string()) {
            return;
        }

        //Ignores reactions from the bot
        if reaction.user_id.unwrap() == ctx.cache.current_user_id() {
            return;
        }

        if reaction.emoji == ReactionType::Unicode("🚫".to_string()) {
            let ctx_clone = ctx.clone();
            let reaction_clone = reaction.clone();
            tokio::spawn(async move {
                let mut msg = reaction_clone.channel_id.message(&ctx_clone.http, reaction_clone.message_id).await.unwrap();
                let user_id = &msg.embeds[0].fields[0].value[2..msg.embeds[0].fields[0].value.len() - 1];

                let data = LogData {
                    importance: "INFO".to_string(),
                    message: format!("{} Has dismissed a report", reaction_clone.user_id.unwrap()),
                };
                log_this(data);

                dismiss_infraction(user_id.parse::<u64>().unwrap());

                let user = UserId(user_id.parse::<u64>().unwrap()).to_user(&ctx_clone.http).await.unwrap();
                user.dm(&ctx_clone.http, |m| m.content("Your report has been dismissed by a staff member due to it being found as being a false positive.")).await.expect("Unable to dm user");

                let mut embed = CreateEmbed::default();
                embed.color(0x00FF00);
                embed.title("Message blocked due to matching a set regex pattern");
                embed.field("The user who broke a regx pattern is below:", format!("<@{}>", user_id), false);
                embed.field("Their message is the following below:", format!("||{}||", &msg.embeds[0].fields[1].value[2..msg.embeds[0].fields[1].value.len() - 2]), false);
                embed.footer(|f| f.text("This infraction has been dismissed by a staff member"));
                msg.edit(&ctx_clone.http, |m| m.set_embed(embed)).await.ok();

                msg.delete_reaction_emoji(&ctx_clone.http, ReactionType::Unicode("🚫".to_string())).await.ok();

                //Delete the embed
                /*if let Err(why) = msg.delete(&ctx_clone.http).await {
                //    println!("Error deleting message: {:?}", why);
                }*/
            });
        }
    }
}

#[tokio::main]
async fn main() {
    //check for config file
    if !Path::new("config.toml").exists() {
        gen_config();
    }

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![user()],
            ..Default::default()
        })
        .token(get_config().token)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
    });

    framework.run().await.unwrap();
}