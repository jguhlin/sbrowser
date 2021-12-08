use bytelines::*;
use simdutf8::basic::from_utf8;
use twox_hash::RandomXxh3HashBuilder64;
use std::str::FromStr;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::num::NonZeroUsize;
use std::sync::Arc;
use crate::structs::*;

use super::feature::*;

#[derive(Clone, Debug)]
pub struct Gfa {
    pub filename: String,
    pub segments: HashMap<String, Segment, RandomXxh3HashBuilder64>,
    pub lengths: HashMap<String, usize, RandomXxh3HashBuilder64>,
    pub links: Vec<Arc<Link>>,
    pub links_atlas: HashMap<String, Vec<Arc<Link>>, RandomXxh3HashBuilder64>,
}

impl Gfa {
    pub fn parse<T>(filename: T) -> Result<Gfa, String>
    where
        T: ToString,
    {
        let filename = filename.to_string();

        let file = match File::open(&filename) {
            Ok(x) => BufReader::new(x),
            Err(_) => return Err(format!("Unable to open file {}", &filename)),
        };

        let mut segments: HashMap<String, Segment, RandomXxh3HashBuilder64> = Default::default();
        let mut lengths: HashMap<String, usize, RandomXxh3HashBuilder64> = Default::default();
        let mut links: Vec<Arc<Link>> = Vec::with_capacity(1 * 1024 * 1024);
        let mut links_atlas: HashMap<String, Vec<Arc<Link>>, RandomXxh3HashBuilder64> = Default::default();

        let mut lines = file.byte_lines();

        while let Some(line) = lines.next() {
            let line = line.unwrap();
            let split = line[..].split(|&x| x == b"\t"[0]).collect::<Vec<&[u8]>>();
            if split[0] == b"S" {
                // Segment line
                let mut segment = Segment::default();
                let id = from_utf8(split[1]).unwrap();
                let length = split[2].len();
                segment.id = id.to_string();
                lengths.insert(id.to_string(), length);

                for tag in split[3..].iter() {
                    let tag = from_utf8(tag).unwrap();
                    println!("{:#?}", tag);
                    if tag.starts_with("LN:i:") {
                        segment.length = Some(tag[5..].parse::<NonZeroUsize>().unwrap());
                        assert!(segment.length.unwrap().get() == length);
                    } else if tag.starts_with("RC:i:") {
                        segment.read_count = Some(tag[5..].parse::<NonZeroUsize>().unwrap());
                    } else if tag.starts_with("FC:i:") {
                        segment.fragment_count = Some(tag[5..].parse::<NonZeroUsize>().unwrap());
                    } else if tag.starts_with("KC:i:") {
                        segment.kmer_count = Some(tag[5..].parse::<NonZeroUsize>().unwrap());
                    } else if tag.starts_with("CS:Z:") {
                        segment.checksum = tag[5..].parse::<u64>().unwrap();
                    } else if tag.starts_with("UR:Z:") {
                        segment.path = Some(tag[5..].to_string());
                    }
                }
                segments.insert(id.to_string(), segment);
                // println!("{} {}", id, length);
            } else if split[0] == b"L" {
                // Link line
                let link = Link {
                    from: from_utf8(split[1]).unwrap().to_string(),
                    from_orient: from_utf8(split[2]).unwrap().parse::<Orientation>().unwrap(),
                    to: from_utf8(split[2]).unwrap().to_string(),
                    to_orient: from_utf8(split[2]).unwrap().parse::<Orientation>().unwrap(),
                    overlap: None,
                };

                let link = Arc::new(link);

                links.push(Arc::clone(&link));
                links_atlas.entry(link.from.clone()).or_default().push(Arc::clone(&link));
                links_atlas.entry(link.to.clone()).or_default().push(Arc::clone(&link));
            }
        }

        Ok(Gfa {
            filename,
            segments,
            lengths,
            links,
            links_atlas,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
