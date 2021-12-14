use bytelines::*;
use memchr::memchr_iter;
use simdutf8::basic::from_utf8;
use std::str::FromStr;
use twox_hash::RandomXxh3HashBuilder64;

use crate::structs::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::num::NonZeroUsize;
use std::sync::Arc;

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

        let mut segments: HashMap<String, Segment, RandomXxh3HashBuilder64> =
            HashMap::with_capacity_and_hasher(1024 * 10, RandomXxh3HashBuilder64::default()); //Default::default();
        let mut lengths: HashMap<String, usize, RandomXxh3HashBuilder64> =
            HashMap::with_capacity_and_hasher(1024 * 10, RandomXxh3HashBuilder64::default()); // Default::default();
        let mut links: Vec<Arc<Link>> = Vec::with_capacity(5 * 1024 * 1024);
        let mut links_atlas: HashMap<String, Vec<Arc<Link>>, RandomXxh3HashBuilder64> =
            HashMap::with_capacity_and_hasher(1024 * 10, RandomXxh3HashBuilder64::default());

        let mut lines = file.byte_lines();

        while let Some(line) = lines.next() {
            let line = line.unwrap();
            //let split = line[..].split(|&x| x == '\t' as u8).collect::<Vec<&[u8]>>();

            if line[0] == 'S' as u8 {
                let split = memchr_iter('\t' as u8, &line[..]).collect::<Vec<usize>>();
                // Segment line
                let mut segment = Segment::default();
                let id = from_utf8(&line[split[0] + 1..split[1]])
                    .unwrap()
                    .to_string();
                let length = split[2] - split[1] - 1;
                segment.id = id.clone();
                lengths.insert(id.clone(), length);

                //for tag in split[3..].iter() {
                for tag_loc in 2..split.len() {
                    let tag = from_utf8(if tag_loc + 1 >= split.len() {
                        &line[split[tag_loc] + 1..]
                    } else {
                        &line[split[tag_loc] + 1..split[tag_loc + 1]]
                    })
                    .unwrap();
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
                segments.insert(id.clone(), segment);
                // println!("{} {}", id, length);
            } else if line[0] == 'L' as u8 {
                // Won't have Mbs of data so we can just parse the entire line here without a slowdown...
                let linkline = from_utf8(&line[2..])
                    .unwrap()
                    .split('\t')
                    .collect::<Vec<&str>>();
                // Link line
                let link = Link {
                    from: linkline[0].to_string(),
                    from_orient: linkline[1].parse::<Orientation>().unwrap(),
                    to: linkline[2].to_string(),
                    to_orient: linkline[3].parse::<Orientation>().unwrap(),
                    overlap: None,
                    //from: from_utf8(&line[split[1]+1..split[2]]).unwrap().to_string(),
                    //from_orient: from_utf8(&line[split[2]+1..split[3]]).unwrap().parse::<Orientation>().unwrap(),
                    //to: from_utf8(&line[split[3]+1..split[4]]).unwrap().to_string(),
                    //to_orient: from_utf8(&line[split[4]+1..split[5]]).unwrap().parse::<Orientation>().unwrap(),
                    //overlap: None,
                };

                let link = Arc::new(link);

                links.push(Arc::clone(&link));
                links_atlas
                    .entry(link.from.clone())
                    .or_default()
                    .push(Arc::clone(&link));
                links_atlas
                    .entry(link.to.clone())
                    .or_default()
                    .push(Arc::clone(&link));
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
