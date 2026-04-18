use crate::commands::VoiceCommand;
use crate::errors::NvError;

pub struct EditorEngine;

impl EditorEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn apply(&mut self, command: VoiceCommand) -> Result<(), NvError> {
        match command {
            VoiceCommand::CreateFile { path } => {
                println!("[editor] create file request: {path}");
                Ok(())
            }
            VoiceCommand::SaveFile => {
                println!("[editor] save file request");
                Ok(())
            }
            VoiceCommand::FindAndReplace { from, to } => {
                println!("[editor] find and replace: '{from}' -> '{to}'");
                Ok(())
            }
            VoiceCommand::GoToLine { line } => {
                println!("[editor] go to line: {line}");
                Ok(())
            }
            VoiceCommand::CommentLine => {
                println!("[editor] comment current line");
                Ok(())
            }
            VoiceCommand::RunCode => {
                println!("[editor] run code request");
                Ok(())
            }
            VoiceCommand::Unknown(raw) => Err(NvError::NotImplemented(format!(
                "unknown voice command: {raw}"
            ))),
        }
    }
}
