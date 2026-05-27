use crate::api::models::{
    Content, GenerateContentRequest, GenerateContentResponse, GenerationConfig, Part,
};
use crate::error::{Error, Result};
use std::env;
use std::time::Duration;

/// Core HTTP client for wrapping connection routing and deserialization checks with the Google Gemini API.
pub struct GeminiClient {
    client: reqwest::Client,
    api_key: String,
}

impl GeminiClient {
    /// Instantiates the client by reading validation keys from the system environment.
    /// Returns `Error::MissingApiKey` if `GEMINI_API_KEY` is not configured.
    pub fn new() -> Result<Self> {
        // Read API key from environment variable
        let api_key = env::var("GEMINI_API_KEY").map_err(|_| Error::MissingApiKey)?;

        if api_key.trim().is_empty() {
            return Err(Error::MissingApiKey);
        }

        // Initialize connection client with a strict 30-second timeout guardrail
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;

        Ok(Self { client, api_key })
    }

    /// Asynchronously transmits a single text prompt to the Gemini API and extracts the raw textual candidate.
    pub async fn ask(&self, prompt: &str, model: &str, temp: Option<f32>) -> Result<String> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            model, self.api_key
        );

        // Build Gemini request payload structure
        let request_payload = GenerateContentRequest {
            contents: vec![Content {
                role: "user".to_string(),
                parts: vec![Part {
                    text: prompt.to_string(),
                }],
            }],
            generation_config: temp.map(|t| GenerationConfig {
                temperature: Some(t),
            }),
        };

        // Execute HTTPS POST request
        let response = self.client.post(&url).json(&request_payload).send().await?;

        let status = response.status();

        // Translate specific API error categories cleanly without panicking
        if !status.is_success() {
            match status.as_u16() {
                429 => return Err(Error::RateLimit),
                401 | 403 => return Err(Error::Unauthorized),
                _ => {
                    let err_body = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "Unknown error payload".to_string());
                    return Err(Error::ApiError {
                        status,
                        message: err_body,
                    });
                }
            }
        }

        // Deserialize response JSON schema
        let response_payload: GenerateContentResponse = response.json().await?;

        // Extract first generated part safely adhering to the Zero-Unwrap policy
        let candidate = response_payload.candidates.first().ok_or_else(|| {
            Error::Cli("The model response candidates array is empty.".to_string())
        })?;

        let content = candidate.content.as_ref().ok_or_else(|| {
            Error::Cli("The candidate was returned without a valid content segment.".to_string())
        })?;

        let part = content.parts.first().ok_or_else(|| {
            Error::Cli("The candidate content did not contain any text parts.".to_string())
        })?;

        Ok(part.text.clone())
    }

    /// Asynchronously streams generated text chunks from the Gemini API and renders them to stdout.
    pub async fn ask_stream(&self, prompt: &str, model: &str, temp: Option<f32>) -> Result<()> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:streamGenerateContent?alt=sse&key={}",
            model, self.api_key
        );

        // Build Gemini request payload structure
        let request_payload = GenerateContentRequest {
            contents: vec![Content {
                role: "user".to_string(),
                parts: vec![Part {
                    text: prompt.to_string(),
                }],
            }],
            generation_config: temp.map(|t| GenerationConfig {
                temperature: Some(t),
            }),
        };

        // Execute HTTPS POST request to streamGenerateContent endpoint
        let response = self.client.post(&url).json(&request_payload).send().await?;

        let status = response.status();

        // Translate specific API error categories cleanly without panicking
        if !status.is_success() {
            match status.as_u16() {
                429 => return Err(Error::RateLimit),
                401 | 403 => return Err(Error::Unauthorized),
                _ => {
                    let err_body = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "Unknown error payload".to_string());
                    return Err(Error::ApiError {
                        status,
                        message: err_body,
                    });
                }
            }
        }

        // Retrieve bytes stream from the response body
        let bytes_stream = response.bytes_stream();

        // Pass the stream directly to the Real-Time Stream Renderer
        crate::io::stream::render_stream(bytes_stream).await?;

        Ok(())
    }
}
