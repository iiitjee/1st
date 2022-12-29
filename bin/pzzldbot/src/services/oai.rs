/*
    Appellation: oai <module>
    Contrib: FL03 <j3mccain@gmail.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use async_openai as openai;

use openai::{
    types::{CreateCompletionRequest, CreateCompletionResponse},
    Completion,
};
use scsys::AsyncResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Hash, Eq, PartialEq, Serialize)]
pub struct OpenAI(String);

impl OpenAI {
    pub fn new(secret_key: Option<String>) -> Self {
        let secret_key = secret_key.unwrap_or_default();
        Self(secret_key)
    }
    pub fn from_env(secret_key: Option<&str>) -> Self {
        let secret_key = match std::env::var(secret_key.unwrap_or("OPENAI_API_KEY")) {
            Err(_) => None,
            Ok(v) => Some(v),
        };
        Self::new(secret_key)
    }
    pub fn is_auth(&self) -> bool {
        self.0.len() > 1 && self.0.starts_with("sk-")
    }
    pub fn client(&self) -> openai::Client {
        openai::Client::new().with_api_key(self.0.as_str())
    }
    pub fn create_request(&self, prompt: &str) -> CreateCompletionRequest {
        CreateCompletionRequest {
            max_tokens: Some(1000),
            model: "text-davinci-003".to_owned(),
            prompt: Some(prompt.to_string()),
            temperature: Some(0.5),
            ..Default::default()
        }
    }
    pub async fn response(
        &self,
        req: CreateCompletionRequest,
    ) -> AsyncResult<CreateCompletionResponse> {
        let res = Completion::create(&self.client(), req).await?;
        Ok(res)
    }
}

impl Default for OpenAI {
    fn default() -> Self {
        Self::from_env(None)
    }
}

pub async fn chatgpt(prompt: &str) -> AsyncResult {
    let oai = OpenAI::from_env(Some("OPENAI_SECRET_KEY"));
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
    tmp.rsplit("\n").collect()
}

pub struct ChatGPT(OpenAI);

impl ChatGPT {
    pub fn client(&self) -> openai::Client {
        self.0.client()
    }
}
