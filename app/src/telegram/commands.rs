/*
    Appellation: commands <module>
    Contrib: FL03 <j3mccain@gmail.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use crate::openai::{clean_choices, ChatGPT};

use teloxide::prelude::{Bot, Message, Requester, ResponseResult};
use teloxide::utils::command::BotCommands;

/// Defines the desired command structure for the [TelegramBot]
#[derive(BotCommands, Clone, Debug, PartialEq)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    #[command(description = "Rolls a 6-sided die")]
    Dice,
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Given a topic or url, return a concise summary")]
    Query(String),
}

impl Command {
    pub fn dice() -> Self {
        Self::Dice
    }
    pub fn help() -> Self {
        Self::Help
    }
    pub fn query(data: String) -> Self {
        Self::Query(data)
    }
}

/// A verbose handler for dealing with chatgpt related queries; returns a [ResponseResult]
async fn handle_oai_query(bot: &Bot, msg: Message, prompt: String) -> ResponseResult<()> {
    let gpt = ChatGPT::default();
    let res = gpt.response(gpt.request(prompt.as_str())).await.expect("");
    bot.send_message(msg.chat.id, clean_choices(res)).await?;
    Ok(())
}

/// Handles the commands issued to the bot and returns a [ResponseResult]
pub async fn handler(bot: Bot, cmd: Command, msg: Message) -> ResponseResult<()> {
    match cmd {
        Command::Dice => {
            bot.send_dice(msg.chat.id).await?;
        }
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Query(prompt) => {
            handle_oai_query(&bot, msg, prompt).await?;
        }
    };

    Ok(())
}
