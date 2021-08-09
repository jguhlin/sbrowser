pub struct Feature {
    pub name: String,
    pub landmark: String,
    pub start: usize,
    pub end: usize,
    pub subfeatures: Option<Vec<Feature>>,
}

impl Feature {
    pub fn from_gff3_line(line: &str) -> Result<Feature, String> {
        let split = line.splitn(9, "\t");

        Ok(Feature {
            name: String::from("Hello"),
            landmark: String::from("Chr1"),
            start: 0,
            end: 0,
            subfeatures: None,
        })
    }
}
