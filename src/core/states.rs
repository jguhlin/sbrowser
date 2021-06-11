#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    SequenceOverview,
    Overview,
    SequenceView,
    ChromosomeView,
    GeneView,
    ProteinView,
}
