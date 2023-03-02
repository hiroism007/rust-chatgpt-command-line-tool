use regex::Regex;
use reqwest::*;
use serde::{Deserialize, Serialize};
use std::env;
use text_io::read;

static OPENAI_ENDPOINT_CHAT: &str = "https://api.openai.com/v1/chat/completions";

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct Question {
    model: String,
    messages: Vec<Message>,
}
#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
}
#[derive(Debug, Deserialize)]
struct ResponseChoice {
    message: ResponseMessage,
}
#[derive(Debug, Deserialize)]
struct GPTResponse {
    choices: Vec<ResponseChoice>,
}

fn generate_question(questions: Vec<&str>) -> Question {
    Question {
        model: "gpt-3.5-turbo".to_string(),
        messages: questions
            .iter()
            .map(|q| Message {
                role: "user".to_string(),
                content: q.to_string(),
            })
            .collect(),
    }
}

async fn ask_chat_gpt(client: &Client, api_key: &str, q: &Question, r: &Regex) {
    let response = client
        .post(OPENAI_ENDPOINT_CHAT)
        .json(q)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .expect("Failed to send request");
    let gpt_response: GPTResponse = response.json().await.expect("Failed to parse response");
    println!(
        "{:?}",
        r.replace_all(&gpt_response.choices[0].message.content, "")
    );
}

#[tokio::main]
async fn main() {
    let re = Regex::new(r"\r?\n").unwrap();
    let client = reqwest::Client::new();
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    println!("Hi, what can I help you with? (type 'quit' to exit)");
    loop {
        let line: String = read!("{}\n");
        if line == "quit" {
            break;
        }
        let q = generate_question(vec![&line]);
        ask_chat_gpt(&client, &api_key, &q, &re).await;
    }
}
