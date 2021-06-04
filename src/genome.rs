// Placeholder


pub struct Genome {
    pub chromosomes: Vec<Chromosome>,
}

#[derive(Clone)]
pub struct Chromosome {
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

    //let genes = (0..256).into_iter().map(|pos| Gene { start: pos * 8192, end: (pos * 8192) + 2048} ).collect::<Vec<Gene>>();
    // let genes = (0..20).into_iter().map(|pos| Gene { start: pos * 10000, end: (pos * 10000) + 2048} ).collect::<Vec<Gene>>();
    let genes = vec![
        Gene { start: 1000, end: 5000 }, 
        Gene { start: 10000, end: 20000 }, 
        Gene { start: 500000, end: 600000 },
        
    ];

    Genome {
        chromosomes: vec![
            Chromosome {
                length: 1 * 1024 * 1024, // 1Mbp genome
                genes,
            }],
    }
}