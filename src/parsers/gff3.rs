use bytelines::*;
use simdutf8::basic::from_utf8;
use twox_hash::RandomXxh3HashBuilder64;

use std::collections::HashMap;
use std::fs::File;
use std::hash::BuildHasherDefault;
use std::io::{BufRead, BufReader, Seek};

use super::feature::*;

#[derive(Clone, Debug)]
pub struct Gff3 {
    pub filename: String,
    pub landmarks: Vec<(String, usize, usize)>, // Landmark name, byte offset, length of data
}

impl Gff3 {
    pub fn parse<T>(filename: T) -> Result<Gff3, String>
    where
        T: ToString,
    {
        let filename = filename.to_string();

        let mut file = match File::open(&filename) {
            Ok(x) => BufReader::new(x),
            Err(_) => return Err(format!("Unable to open file {}", &filename)),
        };

        // let mut lines = BufReader::new(file).byte_lines();

        let mut line_number: usize = 0;

        let mut landmarks: HashMap<String, usize, RandomXxh3HashBuilder64> = Default::default();

        let mut current_offset: usize = 0;

        // while let Some(Ok(line)) = lines.next() {
        let mut line: Vec<u8> = Vec::with_capacity(8192);
        let mut bytes_read = 42;

        while bytes_read > 0 {
            line.clear();
            current_offset = file.stream_position().expect("Seek API is broken") as usize;
            line_number += 1;

            if let Ok(bytes) = file.read_until(b'\n', &mut line) {
                bytes_read = bytes;
            } else {
                return Err(format!("Error parsing file {}", &filename));
            }

            // EOF
            if bytes_read == 0 {
                break;
            }

            // Skip blank lines
            if line.is_empty() {
                continue;
            }

            // Skip comments (TODO: Maybe store them in the future though? For context?)
            if line[0] == b'#' {
                continue;
            }

            let line = match from_utf8(&line) {
                Ok(x) => x.trim(),
                Err(err) => {
                    // TODO: Formal logging library...
                    println!(
                        "Error while parsing GFF File -- Line {} skipped: {}",
                        line_number, err
                    );
                    continue;
                }
            };

            // After trimming, is the line blank?
            if line.is_empty() {
                continue;
            }

            // Parse GFF3 Lines to identify Landmark starting sites (and landmarks)
            if let Some((landmark, _)) = line.split_once("\t") {
                if !landmarks.contains_key(landmark) {
                    landmarks.insert(landmark.to_string(), current_offset);
                }
            } else {
                println!("Error while parsing GFF File -- Line {}", line_number);
                continue;
            }
        }

        let mut landmarks: Vec<(String, usize)> = landmarks.drain().collect();
        landmarks.sort_by_key(|x| x.1);

        let mut landmarks_final_vec = Vec::with_capacity(landmarks.len());
        let mut lengths = Vec::with_capacity(landmarks.len());

        for x in landmarks.windows(2) {
            let len = x[1].1 - x[0].1;
            lengths.push(len);
        }

        lengths.push(current_offset - landmarks.last().unwrap().1);

        for (n, x) in landmarks.into_iter().enumerate() {
            landmarks_final_vec.push((x.0, x.1, lengths[n]));
        }

        landmarks_final_vec.sort_by_key(|x| x.2);
        landmarks_final_vec.reverse();

        Ok(Gff3 {
            filename,
            landmarks: landmarks_final_vec,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_test_gff3() {
        let j = Gff3::parse("test_data/test.gff3");
        println!("{:#?}", j);
        panic!("Ok");
    }

    #[test]
    fn test_parse_large_gff3() {
        let j = Gff3::parse("test_data/kakapo_large.gff3");
        println!("{:#?}", j);
        panic!("Ok");
    }
}
