//! JSON Lines: bounded header plus one independently decoded puzzle per line.
use crate::rules;
use serde::{Deserialize, Serialize};
use shakmaty::Position;
use std::{collections::BTreeSet, ops::Range};

pub const BUILTIN: &str = include_str!("../packs/lichess-20.jsonl");
const MAX_LINE: usize = 8192;
const MAX_PACK: usize = 4 * 1024 * 1024;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Header {
    pub schema_version: u32,
    pub collection_id: String,
    pub title: String,
    pub source: String,
    pub license: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Puzzle {
    pub id: String,
    pub fen: String,
    pub side_to_move: String,
    pub solution: Vec<String>,
    #[serde(default)]
    pub explanation: String,
    #[serde(default)]
    pub difficulty: Option<u32>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub source_url: Option<String>,
}

pub struct Pack {
    pub header: Header,
    data: String,
    records: Vec<Range<usize>>,
    ids: BTreeSet<String>,
}

fn identifier(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 96
        && s.bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"-_.".contains(&c))
}
impl Puzzle {
    pub fn validate(&self) -> Result<(), String> {
        if !identifier(&self.id)
            || self.fen.len() > 160
            || self.explanation.chars().count() > 180
            || self.solution.is_empty()
            || self.solution.len() > 63
            || self.solution.len() % 2 == 0
            || self.tags.len() > 16
            || self.tags.iter().any(|t| t.len() > 48)
            || self.source_url.as_ref().is_some_and(|u| u.len() > 256)
        {
            return Err("Puzzle bounds or solution parity invalid".into());
        }
        let mut pos = rules::position(&self.fen)?;
        let side = if pos.turn().is_white() {
            "white"
        } else {
            "black"
        };
        if self.side_to_move != side {
            return Err("Side does not match FEN".into());
        }
        for uci in &self.solution {
            let movement = rules::movement(&pos, uci)?;
            if movement
                .to_uci(shakmaty::CastlingMode::Standard)
                .to_string()
                != *uci
            {
                return Err("Solution must use canonical standard UCI".into());
            }
            pos.play_unchecked(&movement);
        }
        Ok(())
    }
}
impl Pack {
    pub fn parse(data: String) -> Result<Self, String> {
        if data.len() > MAX_PACK {
            return Err("Pack exceeds 4 MiB".into());
        }
        let mut lines = data.split_inclusive('\n');
        let first = lines.next().ok_or("Missing header")?;
        if first.len() > MAX_LINE {
            return Err("Header too large".into());
        }
        let header: Header = serde_json::from_str(first).map_err(|e| e.to_string())?;
        if header.schema_version != 1
            || !identifier(&header.collection_id)
            || header.title.is_empty()
            || header.title.chars().count() > 48
            || header.source.len() > 512
            || header.license.len() > 96
        {
            return Err("Incompatible pack header".into());
        }
        let mut offset = first.len();
        let mut records = Vec::new();
        let mut ids = BTreeSet::new();
        for line in lines {
            if line.len() > MAX_LINE || records.len() >= 4096 {
                return Err("Pack bounds exceeded".into());
            }
            let puzzle: Puzzle = serde_json::from_str(line).map_err(|e| e.to_string())?;
            puzzle
                .validate()
                .map_err(|e| format!("{}: {e}", puzzle.id))?;
            if !ids.insert(puzzle.id) {
                return Err("Duplicate puzzle ID".into());
            }
            records.push(offset..offset + line.len());
            offset += line.len();
        }
        if records.is_empty() {
            return Err("Empty pack".into());
        }
        Ok(Self {
            header,
            data,
            records,
            ids,
        })
    }
    pub fn contains_id(&self, id: &str) -> bool {
        self.ids.contains(id)
    }
    pub fn len(&self) -> usize {
        self.records.len()
    }
    pub fn puzzle(&self, index: usize) -> Puzzle {
        // Bytes and offsets are immutable after full validation.
        serde_json::from_str(&self.data[self.records[index].clone()]).expect("validated pack")
    }
}

/// Bound host import reads before allocation; a whole database is never needed.
pub fn read_input(reader: impl std::io::Read) -> Result<Vec<u8>, String> {
    use std::io::Read;
    let mut bytes = Vec::new();
    reader
        .take((MAX_PACK + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() > MAX_PACK {
        return Err("Input exceeds 4 MiB; select a subset".into());
    }
    Ok(bytes)
}
