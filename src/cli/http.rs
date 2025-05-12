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
    stream: bool,
    extra_body: RequestExtraBody,
}

impl Default for Request {
    fn default() -> Self {
        let empty_string = String::new();
        Self {
            model: empty_string,
            messages: Default::default(),
            temperature: 0.3,
            stream: false,
            extra_body: RequestExtraBody {
                enable_thinking: false,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct RequestExtraBody {
    enable_thinking: bool,
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
    log_info: bool,
) -> Result<String, Box<dyn Error>> {
    let mut request_body = Request {
        model: model_name.into(),
        messages: vec![Message {
            role: "user".to_string(),
            content: user.to_string(),
        }],
        ..Default::default()
    };
    if let Some(system) = system {
        request_body.messages.push(Message {
            role: "system".to_string(),
            content: system.to_string(),
        });
    }
    let url = check_url(url, model_name);
    if log_info {
        println!("ai request url: {}", url);
    }

    // 发送请求
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;
    let status = response.status();
    if status.as_u16() >= 400 {
        return Err(format!("ai request failed: {}", status).into());
    }

    let res_text = response.text().await?;
    if log_info {
        println!("ai request result ({}): {}", status, res_text);
    }

    let res: Response = serde_json::from_str(&res_text)?;

    if let Some(res) = res.choices {
        return Ok(res[0].message.content.clone());
    }
    eprintln!("{:?}", res);
    if let Some(res) = res.error {
        return Err(res.message.into());
    }
    Err("unkown exception".into())
}

fn check_url(url: &str, model_name: &str) -> String {
    if url.starts_with("http") {
        return url.to_string();
    }
    if model_name.starts_with("moonshot") {
        return crate::constants::MONOSHOT_URL.to_string();
    }
    if model_name.starts_with("qwen") {
        return crate::constants::QWEN_URL.to_string();
    }
    if model_name.starts_with("gpt") {
        return crate::constants::GPT_URL.to_string();
    }
    if model_name.starts_with("gemini") {
        return crate::constants::GEMINI_URL.to_string();
    }
    eprintln!("Error: please setup ai url first");
    std::process::exit(1);
}

#[cfg(test)]
mod tests {
    use crate::cli::http::chat;

    #[tokio::test]
    async fn test_chat_success() {
        let result = chat(
            crate::constants::QWEN_URL,
            "qwen2.5-0.5b-instruct",
            "sk-",
            Some("You are a helpful assistant."),
            "讲个笑话",
            false,
        )
        .await;
        if result.is_err() {
            println!("模型报错: {}", result.unwrap_err());
        } else {
            println!("模型回复: {}", result.unwrap());
        }
    }
}
