use std::str::FromStr;

pub struct Feature {
    pub name: String,
    pub landmark: String,
    pub start: usize,
    pub end: usize,
    pub feature_type: String,
    pub subfeatures: Option<Vec<Feature>>,
}

impl Feature {
    pub fn from_gff3_line(line: &str) -> Result<Feature, String> {
        let split = line.splitn(9, "\t").collect::<Vec<&str>>();

        Ok(Feature {
            name: String::from("Hello"),
            landmark: split[0].to_string(),
            start: usize::from_str(split[3]).unwrap(),
            end: usize::from_str(split[4]).unwrap(),
            feature_type: split[2].to_string(),
            subfeatures: None,
        })
    }
}
