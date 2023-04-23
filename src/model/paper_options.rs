use dialoguer::Confirm;
use crate::DIALOGUE_THEME;

pub struct PaperOptions {
    pub run_server: bool
}

impl PaperOptions {

    pub fn from_dialogue() -> anyhow::Result<Self> {

        let run_server = Confirm::with_theme(&*DIALOGUE_THEME)
            .default(true)
            .with_prompt("Include Run Server Plugin")
            .interact()?;

        Ok( Self { run_server } )
    }
}