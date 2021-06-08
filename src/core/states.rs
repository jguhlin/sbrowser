#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    Overview,
    ChromosomeView,
    GeneView,
    ProteinView,
}