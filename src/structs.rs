use crate::parsers::*;

pub struct CameraMoved;

pub struct LoadLandmark {
    pub id: String,
}

pub struct BrowserState {
    pub landmark: Option<(String, usize)>, // ID, length
    pub gff3: Option<Gff3>,
}

impl Default for BrowserState {
    fn default() -> BrowserState {
        BrowserState {
            landmark: None,
            gff3: None,
        }
    }
}

pub enum View {
    SequenceOverview,
    Chromosome,
    Gene,
    Protein,
}

pub struct UISetting {
    pub zoom_factor: f32,
    pub view: View,
}

impl Default for UISetting {
    fn default() -> UISetting {
        UISetting {
            zoom_factor: 1024.0,
            view: View::SequenceOverview,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct ClickableLandmark {
    pub id: String,
    pub length: usize,
}

impl ClickableLandmark {
    pub fn from(id: &str, length: usize) -> Self {
        ClickableLandmark {
            id: id.to_string(),
            length,
        }
    }
}
