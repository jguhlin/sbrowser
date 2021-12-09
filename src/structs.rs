use crate::parsers::*;

use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::str::FromStr;

use bevy::prelude::*;
use twox_hash::RandomXxHashBuilder64;
use twox_hash::RandomXxh3HashBuilder64;

pub struct CameraMoved;

pub struct LoadLandmark {
    pub id: String,
}

pub struct Collider {
    pub size: Vec2,
}

pub struct DisplayDatabase {
    pub segments: Vec<Segment>,
    pub links: Vec<Link>,
}

pub struct CheckLinks;

pub struct ID {
    pub id: String,
}

pub struct ExpansionRounds {
    pub round: usize,
}

impl ID {
    pub fn from(id: String) -> Self {
        Self { id }
    }
}

#[derive(Clone, Debug, Copy)]
pub enum Orientation {
    Positive,
    Negative,
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Positive
    }
}

impl FromStr for Orientation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Orientation::Positive),
            "-" => Ok(Orientation::Negative),
            _ => Err(()),
        }
    }
}

#[derive(Default, Debug)]
pub struct EntityRegistry {
    pub registry: HashMap<String, Entity, RandomXxh3HashBuilder64>,
}

#[derive(Clone, Debug, Default)]
pub struct Link {
    pub from: String,
    pub from_orient: Orientation,
    pub to: String,
    pub to_orient: Orientation,
    pub overlap: Option<String>,
}

pub struct LinkEntities {
    pub from: Option<Entity>,
    pub to: Option<Entity>,
}

// From: https://github.com/GFA-spec/GFA-spec/blob/master/GFA1.md
#[derive(Clone, Debug, Default)]
pub struct Segment {
    pub id: String,
    pub length: Option<NonZeroUsize>,
    pub read_count: Option<NonZeroUsize>,
    pub fragment_count: Option<NonZeroUsize>,
    pub kmer_count: Option<NonZeroUsize>,
    pub checksum: u64,                    //SHA-256 checksum of the sequence
    pub path: Option<String>,             // URI or local file-system path of the sequence...
    pub reference_name: Option<String>,   // SN:Z: field
    pub sequence_order: Option<usize>,    // SO:i: field
    pub orientation: Option<Orientation>, // For genes, CDS, etc... None when not applicable...
}

pub struct BrowserState {
    pub landmark: Option<(String, usize)>, // ID, length
    pub gff3: Option<Gff3>,
    pub gfa: Option<Gfa>,
}

impl Default for BrowserState {
    fn default() -> BrowserState {
        BrowserState {
            landmark: None,
            gff3: None,
            gfa: None,
        }
    }
}

pub enum View {
    SequenceOverview,
    Chromosome,
    Gene,
    Protein,
}

pub struct UISetting {
    pub zoom_factor: f32,
    pub view: View,
}

impl Default for UISetting {
    fn default() -> UISetting {
        UISetting {
            zoom_factor: 1024.0,
            view: View::SequenceOverview,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct ClickableLandmark {
    pub id: String,
    pub length: usize,
}

impl ClickableLandmark {
    pub fn from(id: &str, length: usize) -> Self {
        ClickableLandmark {
            id: id.to_string(),
            length,
        }
    }
}
