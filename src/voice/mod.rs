use crate::errors::NvError;

pub struct VoicePipeline;

impl VoicePipeline {
    pub fn new() -> Self {
        Self
    }

    pub fn capture_and_transcribe(&mut self) -> Result<String, NvError> {
        Ok(String::from("save the file"))
    }
}
