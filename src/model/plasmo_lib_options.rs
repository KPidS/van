use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use dialoguer::MultiSelect;
use crate::DIALOGUE_THEME;

pub struct PlasmoLibOptions {
    pub features: HashSet<PlasmoLibFeature>
}

const SELECTIONS: [PlasmoLibFeature; 3] = [
    PlasmoLibFeature::Locale,
    PlasmoLibFeature::Config,
    PlasmoLibFeature::Database,
];

impl PlasmoLibOptions {
    pub fn from_dialogue() -> anyhow::Result<Self> {
        let selections = MultiSelect::with_theme(&*DIALOGUE_THEME)
            .with_prompt("PlasmoLib Features")
            .items(&SELECTIONS)
            .interact()?;

        let features = selections.into_iter()
            .map(|index| SELECTIONS[index].clone() )
            .collect::<HashSet<_>>();

        Ok( Self { features } )
    }

}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum PlasmoLibFeature {
    Locale,
    Config,
    Database,
}

impl PlasmoLibFeature {
    pub fn as_str(&self) -> &str {
        match self {
            PlasmoLibFeature::Locale => "Locale",
            PlasmoLibFeature::Config => "Config",
            PlasmoLibFeature::Database => "Database",
        }
    }
}

impl Display for PlasmoLibFeature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())?;
        Ok(())
    }
}