#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    SequenceOverview,
    Overview,
    ChromosomeView,
    GeneView,
    ProteinView,
}
