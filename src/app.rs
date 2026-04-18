use crate::commands::CommandRouter;
use crate::editor::EditorEngine;
use crate::errors::NvError;
use crate::voice::VoicePipeline;

pub struct App {
    command_router: CommandRouter,
    editor_engine: EditorEngine,
    voice_pipeline: VoicePipeline,
}

impl App {
    pub fn new() -> Self {
        Self {
            command_router: CommandRouter::new(),
            editor_engine: EditorEngine::new(),
            voice_pipeline: VoicePipeline::new(),
        }
    }

    pub fn bootstrap(&mut self) {
        println!("NVCode skeleton initialized.");
        println!("Voice pipeline, command router, and editor engine are connected.");
    }

    pub fn tick(&mut self) -> Result<(), NvError> {
        let transcript = self.voice_pipeline.capture_and_transcribe()?;
        let command = self.command_router.parse(&transcript);
        self.editor_engine.apply(command)
    }
}

pub fn run() -> Result<(), NvError> {
    let mut app = App::new();
    app.bootstrap();

    match app.tick() {
        Ok(()) => Ok(()),
        Err(NvError::NotImplemented(_)) => {
            println!("Skeleton mode: runtime features are placeholders.");
            Ok(())
        }
        Err(err) => Err(err),
    }
}
