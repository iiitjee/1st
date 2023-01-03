/*
    Appellation: oai <module>
    Contrib: FL03 <j3mccain@gmail.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use super::OpenAI;

use async_openai::types::{
    CreateCompletionRequest, CreateCompletionRequestArgs, CreateCompletionResponse,
};
use async_openai::Completions;
use scsys::AsyncResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ChatGPTParams {
    pub max_tokens: u16,
    pub temperature: f32,
}

impl ChatGPTParams {
    pub fn new(max_tokens: Option<u16>, temperature: Option<f32>) -> Self {
        Self {
            max_tokens: max_tokens.unwrap_or(1000),
            temperature: temperature.unwrap_or(0.5),
        }
    }
}

impl Default for ChatGPTParams {
    fn default() -> Self {
        Self::new(None, None)
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum Driver {
    Ada = 0,
    #[default]
    Davinci = 1,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ChatGPT {
    pub driver: OpenAI,
    pub params: ChatGPTParams,
}

impl ChatGPT {
    pub fn new(driver: OpenAI, params: ChatGPTParams) -> Self {
        Self { driver, params }
    }
    pub fn client(&self) -> &OpenAI {
        &self.driver
    }
    pub fn request(&self, prompt: &str) -> CreateCompletionRequest {
        CreateCompletionRequestArgs::default()
            .model("text-davinci-003")
            .prompt(prompt)
            .max_tokens(self.params.max_tokens)
            .temperature(self.params.temperature)
            .build()
            .unwrap()
    }
    pub async fn response(
        &self,
        req: CreateCompletionRequest,
    ) -> AsyncResult<CreateCompletionResponse> {
        let res = Completions::new(&self.driver.client()).create(req).await?;
        Ok(res)
    }
}
