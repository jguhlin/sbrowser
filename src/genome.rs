// Placeholder

pub struct Genome {
    pub length: usize,
    pub genes: Vec<Gene>,
}
pub struct Gene {
    pub start: usize,
    pub end: usize,
    // strand, name, etc...
}

pub fn get_genome() -> Genome {

    // let genes = (0..1000).into_iter().map(|pos| Gene { start: pos * 1000, end: (pos * 1000) + 4096} ).collect::<Vec<Gene>>();
    let genes = (0..2).into_iter().map(|pos| Gene { start: pos * 1000, end: (pos * 1000) + 4096} ).collect::<Vec<Gene>>();

    Genome {
        length: 5 * 1024 * 1024,
        genes,
    }
}