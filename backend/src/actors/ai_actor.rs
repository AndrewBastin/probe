use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub struct AIActor {
    client: reqwest::Client,
    together_ai_api_key: String,
    jina_reader_api_key: String
}

const PROMPT_GET_FUNDING_STATS: &str = r#"
You are to help get funding information for an open source project. You will be given the page contents from a funding website (like Github Sponsors or Open Collective etc.) you are to provide back a JSON with the given structure:

1. `monthly_raised_amount`: A string showing how much amount was raised for the monthly project make sure to specify what currency it is, if no info is found about this, let this value be null. Try to calculate from percentages if that data is given, for example, Github Sponsors might say something like "3% raised in $1000 goal", if that is the case, try calculating the 3% of $1000 and that will be the raised amount.
2. `target_monthly_raised_amount`: A string showing how much amount the funding target is set to (a feature available in Github Sponsors and Open Collective etc.). Make sure to specify what currency it is and if no info is found about the target, let the value be null.
3. `total_raised_amount`: A string showing how much amount was raised for project in total make sure to specify what currency it is, if no info is found about this, let this value be null. Try to calculate from percentages if that data is given, for example, Github Sponsors might say something like "3% raised in $1000 goal", if that is the case, try calculating the 3% of $1000 and that will be the raised amount.
4. `target_raised_amount`: A string showing how much amount the funding target is set to (a feature available in Github Sponsors and Open Collective etc.). Make sure to specify what currency it is and if no info is found about the target, let the value be null.
5. `is_funding`: A boolean value indicating whether the given summary says the project can be donated to or not. Some funding options like Tidelift is not a funding platform but a subscription service.

Only return the JSON, do not return the markdown formatting or any other data out. The response you give is sent directly to a JSON parser.
"#;

#[derive(Clone, Deserialize, Serialize)]
pub struct ParsedFundingInfo {
    monthly_raised_amount: Option<String>,
    target_monthly_raised_amount: Option<String>,
    total_raised_amount: Option<String>,
    target_raised_amount: Option<String>,
    is_funding: bool
}

impl AIActor {
    pub fn new(together_ai_api_key: String, jina_reader_api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            together_ai_api_key,
            jina_reader_api_key
        }
    }

    pub async fn summarize_page(&self, url: &str) -> Option<String> {
        println!("Summarizing page: {}", url);

        let response = self.client
            .get(&format!("https://r.jina.ai/{}", url))
            .header("Authorization", &format!("Bearer {}", self.jina_reader_api_key))
            .send().await
            .ok()?
            .text().await
            .ok()?;

        Some(response)
    }

    pub async fn get_funding_stats_from_page_summary(&self, url: &str, summary: &str) -> Result<ParsedFundingInfo, Box<dyn std::error::Error>> {
        println!("Getting funding stats from page summary. url: {}", url);

        let response = self.client
            .post("https://api.together.xyz/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.together_ai_api_key))
            .json(&json!({
                "model": "meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo",
                "messages": [
                    {"role": "system", "content": PROMPT_GET_FUNDING_STATS },
                    {"role": "user", "content": summary }
                ],
                "temperature": 0.7,
                "max_tokens": 8000
            }))
            .send()
            .await?
            .json::<Value>()
            .await?;

        dbg!(&response);

        let content = response.as_object()
            .unwrap()
            ["choices"].as_array()
            .unwrap()
            .first().unwrap()
            .as_object().unwrap()
            ["message"].as_object()
            .unwrap()
            ["content"].as_str()
            .unwrap()
            .trim_start_matches("```")
            .trim_end_matches("```");

        dbg!(&content);

        let parsed: ParsedFundingInfo = serde_json::from_str(content)?;

        Ok(parsed)
    }
}
