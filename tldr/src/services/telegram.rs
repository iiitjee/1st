/*
    Appellation: telegram <module>
    Contrib: FL03 <j3mccain@gmail.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use scsys::AsyncResult;
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

type Prompt = String;

async fn handler(bot: Bot, cmd: Command, msg: Message) -> ResponseResult<()> {
    match cmd {
        Command::Dice => {
            bot.send_dice(msg.chat.id).await?;
        }
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
        }
        Command::Query(_, prompt) => {
            let oai = crate::services::OpenAI::from_env(None);
            let req = oai.create_request(prompt.as_str());
            let res = format!("{:?}", req);
            bot.send_message(msg.chat.id, res).await?;
        },
    };

    Ok(())
}

#[derive(BotCommands, Clone, Debug, PartialEq)]
#[command(rename_rule = "lowercase", parse_with = "split")]
pub enum Command {
    #[command(description = "Rolls a 6-sided die")]
    Dice,
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Given a topic or url, return a concise summary")]
    Query(String, String),
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
    pub async fn spawn_example(&self) -> AsyncResult {
        teloxide::repl(self.bot(), |bot: Bot, msg: Message| async move {
            bot.send_dice(msg.chat.id).await?;
            Ok(())
        }).await;
        Ok(())
    }
    pub async fn spawn(&self) -> AsyncResult {
        Command::repl(self.bot(), handler).await;
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