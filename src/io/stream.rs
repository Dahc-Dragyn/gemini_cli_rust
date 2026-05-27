use crate::api::models::GenerateContentResponse;
use crate::error::{Error, Result};
use futures_util::StreamExt;
use std::io::Write;

/// Consumes an asynchronous byte stream from the API and flushes tokens directly to stdout.
/// Evaluates Server-Sent Events (SSE) in real-time while protecting against UTF-8 multi-byte
/// character truncation across chunk bounds.
pub async fn render_stream<S, E>(mut stream: S) -> Result<()>
where
    S: futures_util::Stream<Item = std::result::Result<bytes::Bytes, E>> + Unpin,
    E: std::error::Error + Send + Sync + 'static,
{
    let mut byte_buffer = Vec::new();

    // Ingest chunks from the async byte stream
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.map_err(|e| {
            Error::Cli(format!("Stream chunk retrieval failed: {}", e))
        })?;
        byte_buffer.extend_from_slice(&chunk);

        // Process all complete lines in the byte buffer.
        // UTF-8 multi-byte sequences never contain the newline byte (10) as part of their code points.
        // Therefore, splitting on newline bytes is guaranteed to protect character boundaries.
        while let Some(newline_idx) = byte_buffer.iter().position(|&b| b == b'\n') {
            let line_bytes = byte_buffer[..newline_idx].to_vec();
            byte_buffer.drain(..=newline_idx);

            let line = String::from_utf8(line_bytes).map_err(|_| {
                Error::Cli("API stream chunk contained invalid UTF-8 bytes.".to_string())
            })?;

            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            // Google Gemini alt=sse streams each chunk prefixing with "data: "
            if let Some(data_str) = trimmed.strip_prefix("data: ") {
                let clean_data = data_str.trim();
                if clean_data.is_empty() || clean_data == "[DONE]" {
                    continue;
                }

                // Deserialize JSON data chunk into the mapped struct contract
                let response: GenerateContentResponse = serde_json::from_str(clean_data)?;

                // Extract text from the candidate and print immediately
                if let Some(candidate) = response.candidates.first() {
                    if let Some(content) = &candidate.content {
                        if let Some(part) = content.parts.first() {
                            print!("{}", part.text);
                            std::io::stdout().flush().unwrap();
                        }
                    }
                }
            }
        }
    }

    // Final flush to guarantee standard output completeness
    std::io::stdout().flush().unwrap();

    Ok(())
}
