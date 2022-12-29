/*
    Appellation: openai <module>
    Contrib: FL03 <j3mccain@gmail.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
pub use self::{service::*, specs::*, utils::*};

pub(crate) mod service;

const DEFAULT_OPENAI_ENV: &str = "OPENAI_API_KEY";

pub(crate) mod specs {
    use async_openai as oai;

    pub trait OpenAIClientSpec {
        fn api_key(&self) -> String;
        fn client(&self) -> oai::Client {
            oai::Client::default().with_api_key(self.api_key())
        }
    }
}

pub(crate) mod utils {
    use super::OpenAI;
    use async_openai::types::CreateCompletionResponse;
    use scsys::AsyncResult;

    pub async fn chatgpt(prompt: &str) -> AsyncResult {
        let oai = OpenAI::from_env(None);
        let req = oai.create_request(prompt);
        let res = oai.response(req).await?;
        println!("{:?}", res.choices);
        Ok(())
    }

    pub fn clean_choices(response: CreateCompletionResponse) -> String {
        let mut tmp = String::new();
        for i in response.choices {
            tmp.push_str(&i.text);
        }
        tmp.rsplit('\n').collect()
    }
}
