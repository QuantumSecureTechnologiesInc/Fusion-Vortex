/// Production Streaming Parser.
///
/// Implements a state machine for robust, non-blocking JSON parsing from LLM streams.
use fusion_std::error::{StdError, StdResult};
use std::collections::VecDeque;

#[derive(Debug)]
enum JsonParseState {
    WaitingForStart,
    InObject,
    Completed,
}

pub struct StreamingJsonParser {
    buffer: VecDeque<char>,
    state: JsonParseState,
    brace_count: i32,
}

impl StreamingJsonParser {
    pub fn new() -> Self {
        Self {
            buffer: VecDeque::new(),
            state: JsonParseState::WaitingForStart,
            brace_count: 0,
        }
    }

    /// Feeds a chunk of streaming text into the parser.
    /// Returns the first completed JSON object if found.
    pub fn ingest_chunk(&mut self, chunk: &str) -> StdResult<Option<String>> {
        let mut _start_index = None;
        let mut _end_index = None;

        for (i, char) in chunk.chars().enumerate() {
            match self.state {
                JsonParseState::WaitingForStart => {
                    if char == '{' {
                        self.state = JsonParseState::InObject;
                        self.brace_count = 1;
                        _start_index = Some(i);
                        self.buffer.push_back(char);
                    }
                }
                JsonParseState::InObject => {
                    self.buffer.push_back(char);
                    if char == '{' {
                        self.brace_count += 1;
                    } else if char == '}' {
                        self.brace_count -= 1;
                        if self.brace_count == 0 {
                            _end_index = Some(i);
                            self.state = JsonParseState::Completed;
                            break; // Done with this object
                        }
                    }
                }
                JsonParseState::Completed => {
                    // Subsequent data is ignored until buffer is consumed
                }
            }
        }

        if let JsonParseState::Completed = self.state {
            // Extract and clean buffer
            let result: String = self.buffer.drain(..).collect();
            self.state = JsonParseState::WaitingForStart;

            // Final validation (ensure it actually parses as JSON)
            if serde_json::from_str::<serde_json::Value>(&result).is_ok() {
                Ok(Some(result))
            } else {
                // If the stream breaks the JSON mid-key/value, we reset and skip.
                Err(StdError::Serialization(
                    "Malformed stream resulted in incomplete JSON object.".into(),
                ))
            }
        } else {
            Ok(None)
        }
    }
}
