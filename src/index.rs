//! Immutable indexed CC0 starter format. Only the selected record becomes a session.
use crate::{import::LichessRow, pack::Puzzle};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, io::BufRead};

pub const MAX_BYTES: usize = 8 * 1024 * 1024;
pub const MAX_PUZZLES: usize = 20_000;
const MAGIC: &[u8; 8] = b"PCSIDX01";
const PREFIX: usize = 16;
const ENTRY: usize = 12;
const MAX_ROW: usize = 8192;

pub const THEMES: [(&str, &str); 5] = [
    ("mate", "Mate"),
    ("fork", "Fork"),
    ("pin", "Pin"),
    ("endgame", "Endgame"),
    ("middlegame", "Middlegame"),
];
const MIN_RATING: u16 = 80;
const MAX_RATING: u16 = 1500;
pub const RATINGS: [(u16, u16, &str); 4] = [
    (MIN_RATING, MAX_RATING, "Any rating"),
    (MIN_RATING, 1199, "Under 1200"),
    (1200, 1399, "1200–1399"),
    (1400, MAX_RATING, "1400–1500"),
];

pub const SOURCE: &str = "https://database.lichess.org/#puzzles";

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Provenance {
    source: String,
    license: String,
    input_sha256: String,
    selection: String,
}

#[derive(Serialize, Deserialize)]
struct Row(String, String, String, u16, Vec<String>);

impl Row {
    fn id(&self) -> Result<[u8; 5], String> {
        self.0
            .as_bytes()
            .try_into()
            .ok()
            .filter(|id: &[u8; 5]| id.iter().all(u8::is_ascii_alphanumeric))
            .ok_or_else(|| "Expected five ASCII alphanumerics for Lichess ID".into())
    }

    fn theme_mask(&self) -> u8 {
        THEMES.iter().enumerate().fold(0, |mask, (i, (tag, _))| {
            mask | (u8::from(self.4.iter().any(|t| t == tag)) << i)
        })
    }

    fn into_puzzle(self) -> Result<Puzzle, String> {
        if self.1.len() > 160 || self.2.len() > 384 || !(MIN_RATING..=MAX_RATING).contains(&self.3)
        {
            return Err("Invalid source FEN, move-list bounds or rating outside 80–1500".into());
        }
        let url = format!("https://lichess.org/training/{}", self.0);
        LichessRow {
            id: self.0,
            fen: self.1,
            moves: self.2,
            rating: u32::from(self.3),
            themes: self.4,
            game_url: Some(url),
        }
        .into_puzzle()
    }
}

pub struct Index<'a> {
    entries: &'a [u8],
    payload: &'a [u8],
}

fn number(bytes: &[u8]) -> usize {
    u32::from_le_bytes(bytes.try_into().expect("four bytes")) as usize
}

fn validate_provenance(bytes: &[u8]) -> Result<(), String> {
    let p: Provenance = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
    if p.source != SOURCE
        || p.license != "CC0-1.0"
        || p.input_sha256.len() != 64
        || !p
            .input_sha256
            .bytes()
            .all(|c| matches!(c, b'0'..=b'9' | b'a'..=b'f'))
        || p.selection.is_empty()
        || p.selection.len() > 400
    {
        return Err("Invalid CC0 provenance".into());
    }
    Ok(())
}

impl<'a> Index<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self, String> {
        if bytes.len() < PREFIX || bytes.len() > MAX_BYTES || !bytes.starts_with(MAGIC) {
            return Err("Invalid indexed pack size or format version".into());
        }
        let metadata = number(&bytes[8..12]);
        let count = number(&bytes[12..16]);
        if metadata > MAX_ROW || !(1..=MAX_PUZZLES).contains(&count) {
            return Err("Indexed pack bounds exceeded".into());
        }
        let start = PREFIX + metadata;
        let data = start + count * ENTRY;
        if data > bytes.len() {
            return Err("Truncated index".into());
        }
        validate_provenance(&bytes[PREFIX..start])?;
        let index = Self {
            entries: &bytes[start..data],
            payload: &bytes[data..],
        };
        let mut previous: Option<&[u8]> = None;
        let mut end = 0;
        for (i, entry) in index.entries.chunks_exact(ENTRY).enumerate() {
            let id = &entry[..5];
            let next = number(&entry[5..9]);
            if previous.is_some_and(|old| old >= id)
                || next <= end
                || next - end > MAX_ROW
                || next > index.payload.len()
            {
                return Err(format!("Invalid index entry {i}"));
            }
            let row = index.row(i)?;
            if row.id()? != id || (row.3, row.theme_mask()) != index.facets(i) {
                return Err(format!("Index and puzzle disagree at entry {i}"));
            }
            row.into_puzzle()
                .map_err(|e| format!("Puzzle {}: {e}", String::from_utf8_lossy(id)))?;
            previous = Some(id);
            end = next;
        }
        if end != index.payload.len() {
            return Err("Trailing bytes in indexed pack".into());
        }
        Ok(index)
    }

    fn entry(&self, i: usize) -> &[u8] {
        &self.entries[i * ENTRY..(i + 1) * ENTRY]
    }
    pub fn id(&self, i: usize) -> &str {
        std::str::from_utf8(&self.entry(i)[..5]).expect("validated ASCII puzzle ID")
    }
    pub fn facets(&self, i: usize) -> (u16, u8) {
        let e = self.entry(i);
        (u16::from_le_bytes([e[9], e[10]]), e[11])
    }
    pub fn len(&self) -> usize {
        self.entries.len() / ENTRY
    }
    pub fn contains(&self, id: &str) -> bool {
        let mut lo = 0;
        let mut hi = self.len();
        while lo < hi {
            let mid = lo.midpoint(hi);
            match self.id(mid).cmp(id) {
                std::cmp::Ordering::Less => lo = mid + 1,
                std::cmp::Ordering::Greater => hi = mid,
                std::cmp::Ordering::Equal => return true,
            }
        }
        false
    }
    fn row(&self, i: usize) -> Result<Row, String> {
        let offset = |j| number(&self.entry(j)[5..9]);
        let start = if i == 0 { 0 } else { offset(i - 1) };
        serde_json::from_slice(&self.payload[start..offset(i)]).map_err(|e| e.to_string())
    }
    pub fn puzzle(&self, i: usize) -> Puzzle {
        self.row(i)
            .and_then(Row::into_puzzle)
            .expect("validated indexed puzzle")
    }
}

pub fn build(mut input: impl BufRead) -> Result<Vec<u8>, String> {
    let mut line = Vec::new();
    read_line(&mut input, &mut line)?;
    validate_provenance(&line)?;
    let provenance: Provenance = serde_json::from_slice(&line).map_err(|e| e.to_string())?;
    let provenance = serde_json::to_vec(&provenance).map_err(|e| e.to_string())?;
    let mut records = BTreeMap::new();
    let mut size = PREFIX + provenance.len();
    while read_line(&mut input, &mut line)? {
        let row: Row = serde_json::from_slice(&line).map_err(|e| e.to_string())?;
        let id = row.id()?;
        let rating = row.3;
        let mask = row.theme_mask();
        let bytes = serde_json::to_vec(&row).map_err(|e| e.to_string())?;
        row.into_puzzle()
            .map_err(|e| format!("Puzzle {}: {e}", String::from_utf8_lossy(&id)))?;
        size += ENTRY + bytes.len();
        if size > MAX_BYTES || records.len() == MAX_PUZZLES {
            return Err("Indexed pack exceeds milestone bounds".into());
        }
        if records.insert(id, (rating, mask, bytes)).is_some() {
            return Err(format!(
                "Duplicate puzzle ID {}",
                String::from_utf8_lossy(&id)
            ));
        }
    }
    if records.is_empty() {
        return Err("Empty indexed pack".into());
    }
    let u32_bytes = |n| u32::try_from(n).expect("bounded pack size").to_le_bytes();
    let mut output = Vec::with_capacity(size);
    output.extend_from_slice(MAGIC);
    output.extend(u32_bytes(provenance.len()));
    output.extend(u32_bytes(records.len()));
    output.extend(provenance);
    let mut end = 0;
    for (id, (rating, mask, bytes)) in &records {
        end += bytes.len();
        output.extend(id);
        output.extend(u32_bytes(end));
        output.extend(rating.to_le_bytes());
        output.push(*mask);
    }
    for (_, (_, _, bytes)) in records {
        output.extend(bytes);
    }
    Ok(output)
}

fn read_line(reader: &mut impl BufRead, line: &mut Vec<u8>) -> Result<bool, String> {
    use std::io::Read;
    line.clear();
    let size = reader
        .take((MAX_ROW + 1) as u64)
        .read_until(b'\n', line)
        .map_err(|e| e.to_string())?;
    if size > MAX_ROW {
        return Err("Indexed input row exceeds 8 KiB".into());
    }
    Ok(size != 0)
}
