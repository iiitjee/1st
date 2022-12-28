/*
    Appellation: telegram <module>
    Contrib: FL03 <j3mccain@gmail.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use scsys::BoxResult;
use serde::{Deserialize, Serialize};
use teloxide::prelude::*;
use teloxide::dispatching::repls::CommandReplExt;
use teloxide::utils::command::BotCommands;

trait TelegramBotSpec {
    fn name(&self) -> String where Self: Sized;
    fn username(&self) -> String where Self: Sized;
    fn bot_from_env() -> Bot where Self: Sized {
       Bot::from_env()
    }
    fn bot_with_token(token: String) -> Bot where Self: Sized {
        Bot::new(token)
     }
}
type UnitOfTime = u8;

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Username(username) => {
            bot.send_message(msg.chat.id, format!("Your username is @{username}.")).await?
        }
        Command::UsernameAndAge { username, age } => {
            bot.send_message(msg.chat.id, format!("Your username is @{username} and age is {age}."))
                .await?
        }
    };

    Ok(())
}

#[derive(BotCommands, Debug, PartialEq)]
#[command(rename_rule = "lowercase", parse_with = "split")]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
}

#[derive(Clone, Debug, Default, Deserialize, Hash, Eq, PartialEq, Serialize)]
pub struct TelegramBot {
    pub name: String,
    token: String,
    pub username: String,
}

impl TelegramBot {
    pub fn new(name: String, token: String, username: String) -> Self {
        Self { name, token, username }
    }
    pub fn from_env(token: Option<&str>) -> Self {
        let token = std::env::var(token.unwrap_or("TELOXIDE_TOKEN")).ok().unwrap();
        Self::new(Default::default(), token, Default::default())
    }
    pub fn bot(&self) -> Bot {
        Bot::new(self.token.clone())
    }
    pub async fn spawn(&self) -> BoxResult {
        teloxide::repl(self.bot(), |bot: Bot, msg: Message| async move {
            bot.send_dice(msg.chat.id).await?;
            Ok(())
        })
        .await;
        // Command::repl(self.bot(), answer).await;
        Ok(())
    }
}

impl TelegramBotSpec for TelegramBot {
    fn name(&self) -> String where Self: Sized {
        self.name.clone()
    }

    fn username(&self) -> String where Self: Sized {
        self.username.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let a = "";
        assert!(a == "")
    }
}
