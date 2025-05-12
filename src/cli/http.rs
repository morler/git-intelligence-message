use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, validator::Validate)]
struct Message {
    #[validate(length(min = 1))]
    role: String,
    #[validate(length(min = 1))]
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    choices: Option<Vec<Choice>>,
    error: Option<ResponseError>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Choice {
    message: Message,
    finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseError {
    message: String,
    r#type: Option<String>,
}

pub async fn chat(
    url: &str,
    model_name: &str,
    api_key: &str,
    system: Option<&str>,
    user: &str,
) -> Result<String, Box<dyn Error>> {
    // 构造请求体
    let mut request_body = Request {
        model: model_name.into(),
        messages: vec![Message {
            role: "user".to_string(),
            content: user.to_string(),
        }],
        temperature: 0.1,
        max_tokens: 512,
    };
    if let Some(system) = system {
        request_body.messages.push(Message {
            role: "system".to_string(),
            content: system.to_string(),
        });
    }

    // 发送请求
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?
        .json::<Response>()
        // .text()
        .await?;

    if let Some(res) = res.choices {
        return Ok(res[0].message.content.clone());
    }
    if let Some(res) = res.error {
        return Err(res.message.into());
    }
    Err("unkown exception".into())
}

#[cfg(test)]
mod tests {
    use crate::cli::http::chat;

    #[tokio::test]
    async fn test_chat_success() {
        let result = chat(
            "https://api.moonshot.cn/v1/chat/completions",
            "moonshot-v1-8k",
            "sk-",
            Some("You are a helpful assistant."),
            "讲个笑话",
        )
        .await;
        if result.is_err() {
            println!("模型报错: {}", result.unwrap_err());
        } else {
            println!("模型回复: {}", result.unwrap());
        }
    }
}
