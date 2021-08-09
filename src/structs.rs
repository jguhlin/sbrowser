pub struct CameraMoved;

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
}

impl ClickableLandmark {
    pub fn from(id: &str) -> Self {
        ClickableLandmark {
            id: id.to_string(),
        }
    }
}