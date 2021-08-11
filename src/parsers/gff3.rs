use bytelines::*;
use simdutf8::basic::from_utf8;
use twox_hash::RandomXxh3HashBuilder64;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

use super::feature::*;

#[derive(Clone, Debug)]
pub struct Gff3 {
    pub filename: String,
    pub landmarks: Vec<(String, usize, usize, usize, usize)>, // Landmark name, byte offset, length of data, est. length of landmark, number of features
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
        let mut est_lengths: HashMap<String, usize, RandomXxh3HashBuilder64> = Default::default();
        let mut num_features: HashMap<String, usize, RandomXxh3HashBuilder64> = Default::default();

        let mut current_offset: usize = 0;

        let mut chr_length: usize = 0;
        let mut features_count: usize = 0;

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
            // TODO: This will need much nicer error handling...
            let line_parsed: Vec<&str> = line.splitn(6, "\t").collect();
            let landmark = line_parsed[0];
            let start = line_parsed[3].parse::<usize>().expect("Invalid start position");
            let end = line_parsed[4].parse::<usize>().expect("Invalid end position");

            //if let Some((landmark, _)) = line.split_once("\t") {
            if !landmarks.contains_key(landmark) {
                landmarks.insert(landmark.to_string(), current_offset);
                num_features.insert(landmark.to_string(), features_count);
                est_lengths.insert(landmark.to_string(), chr_length);
                features_count = 0;
                chr_length = 0;
            } else {
                features_count = features_count.saturating_add(1);
                chr_length = std::cmp::max(chr_length, start);
                chr_length = std::cmp::max(chr_length, end);
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
            let chr_size = *est_lengths.get(&x.0).unwrap();
            let num_feat = *num_features.get(&x.0).unwrap();

            landmarks_final_vec.push((x.0, x.1, lengths[n], chr_size, num_feat));
        }

        landmarks_final_vec.sort_by_key(|x| x.2);
        landmarks_final_vec.reverse();

        Ok(Gff3 {
            filename,
            landmarks: landmarks_final_vec,
        })
    }

    pub fn parse_region(&self, landmark: &str) -> Result<Vec<Feature>, String> {
        let mut file = match File::open(&self.filename) {
            Ok(x) => BufReader::new(x),
            Err(_) => return Err(format!("Unable to open file {}", &self.filename)),
        };

        let mut features = Vec::new();

        for (id, pos, l, _, _) in self.landmarks.iter() {
            if id == landmark {
                file.seek(SeekFrom::Start(*pos as u64)).expect("Seek IO not working!");
                break;
            }
        }

        let mut lines = file.byte_lines();

        while let Some(line) = lines.next() {
            let line = line.unwrap();

            if line[0] == b'#' {
                continue;
            }

            let x = match from_utf8(&line) {
                Ok(x) => x.trim(),
                Err(err) => {
                    println!("Unable to parse a line from GFF3 file... {}", err);
                    continue;
                }
            };

            let feat = Feature::from_gff3_line(x).unwrap();
            if feat.landmark == landmark {
                features.push(feat);
            } else {
                break;
            }
        }

        Ok(features)
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
