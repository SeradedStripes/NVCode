# NVCode

NVCode or Non Visual Code, is a code editor that allows you to write code without looking at the screen. It is designed to help you focus on your code and avoid distractions.

## How it works

NVCode requires a microphone to capture your voice commands. You can use voice commands for anything in the editor.
Some examples of voice commands include:
- "Create a new file"
- "Save the file"
- "Find and replace"
- "Go to line 10"
- "Comment out this line"
- "Run the code"
- "Change line X to `new code`"

NVCode works by using speech recognition to transcribe your voice commands into text, it then parses the text to determine your action. NVCode does not have a visual interface except for its realtime transcription menu and settings menu, so all feedback is given through audio cues and the transcription menu.

The speech recognition is custom written and optimized for this project. See it in `speech-recognition/`.  
NVCode itself is written in Rust with inline assembly and you can see it in `non-visual-code/`.

## Stack

- Rust
- Rust
- Rust
- Rust
- Inline Assembly

### Rust Crates

- `cpal` for audio capture
- `vosk` for speech recognition
- `egui` for the transcription menu and settings menu

## Installation

To install NVCode, you can download the latest release from the [releases page](https://github.com/SeradedStripes/NVCode/releases) and follow the instructions for your operating system.

## Contributing

Contributions are welcome!  
Fork the repo and do a pr with your changes.

## License

NVCode is licensed under the MIT License. See the [LICENSE](LICENSE.md) file for more details.