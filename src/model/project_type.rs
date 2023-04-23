use std::fmt::{Display, Formatter};
use dialoguer::Select;
use crate::DIALOGUE_THEME;

#[derive(Clone, Eq, PartialEq)]
pub enum ProjectType {
    Paper,
    Velocity,
    Hybrid,
}

const SELECTIONS: [ProjectType; 3] = [
    ProjectType::Paper,
    ProjectType::Velocity,
    ProjectType::Hybrid,
];

impl ProjectType {

    pub fn from_dialogue() -> anyhow::Result<Self> {

        let selection = Select::with_theme(&*DIALOGUE_THEME)
            .with_prompt("Project Type")
            .default(0)
            .items(&SELECTIONS)
            .interact()?;

        let result = SELECTIONS[selection].clone();

        Ok(result)
    }

    pub fn as_str(&self) -> &str {
        match self {
            ProjectType::Paper => "Paper",
            ProjectType::Velocity => "Velocity",
            ProjectType::Hybrid => "Hybrid",
        }
    }
}

impl Display for ProjectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Platform {
    Paper,
    Velocity,
}