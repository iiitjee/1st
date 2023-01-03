/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use scsys::prelude::{fnl_remove, Locked, Message, StatePack};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum::{EnumString, EnumVariantNames};

pub type State = scsys::prelude::State<States>;

#[derive(
    Clone, Copy, Debug, Deserialize, EnumString, EnumVariantNames, Eq, Hash, PartialEq, Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum States {
    Error = 0,
    Idle = 1,
    Complete = 2,
    Derive = 3,
    Process = 4,
    Request = 5,
    Response = 6,
}

impl States {
    pub fn idle() -> Self {
        Self::Idle
    }
}

impl StatePack for States {}

impl Default for States {
    fn default() -> Self {
        Self::idle()
    }
}

impl std::fmt::Display for States {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            fnl_remove(serde_json::to_string(&self).unwrap()).to_ascii_lowercase()
        )
    }
}

impl Into<Locked<States>> for States {
    fn into(self) -> Locked<States> {
        std::sync::Arc::new(std::sync::Mutex::new(self))
    }
}

impl Into<Locked<State>> for States {
    fn into(self) -> Locked<State> {
        std::sync::Arc::new(std::sync::Mutex::new(State::new(None, None, Some(self))))
    }
}

impl From<States> for i64 {
    fn from(val: States) -> Self {
        val as i64
    }
}

impl From<i64> for States {
    fn from(data: i64) -> Self {
        match data {
            0 => Self::Error,
            1 => Self::Idle,
            2 => Self::Complete,
            3 => Self::Derive,
            4 => Self::Process,
            5 => Self::Request,
            6 => Self::Response,
            _ => Self::Error,
        }
    }
}

impl TryInto<Value> for States {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_into(self) -> Result<Value, <States as TryInto<Value>>::Error> {
        let res = serde_json::to_value(State::new(None, None, Some(self)))?;
        Ok(res)
    }
}

impl TryInto<Message> for States {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_into(self) -> Result<Message, <States as TryInto<Message>>::Error> {
        let res: Value = self.try_into()?;
        Ok(Message::from(res))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use scsys::prelude::{State, Stateful, StatefulExt};

    #[test]
    fn test_default_state() {
        let a = State::<States>::default();
        let mut b = a.clone();

        assert_eq!(&a, &b);
        assert_eq!(a.state() as i64, 1);

        b.update_state(None, States::Complete);
        assert_eq!(b.state(), States::Complete)
    }
}
