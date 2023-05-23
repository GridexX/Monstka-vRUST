use anyhow::{Ok, Result};
use log::{info};
use rand::Rng;
use reqwest::get;

pub struct WordEngine {
    random_word_generator_api_url: String,
}

impl WordEngine {
    pub fn new(api_url: String) -> Self {
        Self {
            random_word_generator_api_url: api_url,
        }
    }

    pub async fn generate_random_word(&self) -> Result<String> {
        let response = get(&self.random_word_generator_api_url).await?;
        let content = response.text().await?;
        let words: Vec<String> = serde_json::from_str(&content)?;
        let word = words[0].clone();
        info!("Random word generated : {}", word);
        Ok(word)
    }

    pub async fn get_word_or_dot(&self) -> String {
        let mut rng = rand::thread_rng();
        let random_number: f64 = rng.gen();

        if random_number < 0.5 {
            info!("Return Arsène.");
            "Arsène.".to_string();
        }
        self.generate_random_word()
            .await
            .unwrap_or_else(|_| "Arsène.".to_string())
    }
}
