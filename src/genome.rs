// Placeholder

pub struct Genome {
    pub length: usize,
    pub genes: Vec<Gene>,
}

#[derive(Copy, Clone)]
pub struct Gene {
    pub start: usize,
    pub end: usize,
    // strand, name, etc...
}

pub fn get_genome() -> Genome {

    let genes = (0..256).into_iter().map(|pos| Gene { start: pos * 8192, end: (pos * 8192) + 2048} ).collect::<Vec<Gene>>();
    // let genes = (0..20).into_iter().map(|pos| Gene { start: pos * 10000, end: (pos * 10000) + 2048} ).collect::<Vec<Gene>>();

    Genome {
        length: 1 * 1024 * 1024,
        genes,
    }
}