#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VoiceCommand {
    CreateFile { path: String },
    SaveFile,
    FindAndReplace { from: String, to: String },
    GoToLine { line: usize },
    CommentLine,
    RunCode,
    Unknown(String),
}

pub struct CommandRouter;

impl CommandRouter {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, transcript: &str) -> VoiceCommand {
        let input = transcript.trim();
        let lower = input.to_ascii_lowercase();

        if lower.starts_with("create a new file") {
            return VoiceCommand::CreateFile {
                path: String::from("untitled.rs"),
            };
        }
        if lower == "save the file" {
            return VoiceCommand::SaveFile;
        }
        if lower == "comment out this line" {
            return VoiceCommand::CommentLine;
        }
        if lower == "run the code" {
            return VoiceCommand::RunCode;
        }

        VoiceCommand::Unknown(input.to_string())
    }
}
