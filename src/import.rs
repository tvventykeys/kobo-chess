//! Offline conversion of explicit, selected Lichess exports. No database/network access.
use crate::{
    pack::{Header, Pack, Puzzle},
    rules,
};
use serde::Deserialize;
use shakmaty::{fen::Fen, EnPassantMode, Position};

#[derive(Deserialize)]
pub struct LichessRow {
    #[serde(rename = "puzzleId")]
    pub id: String,
    pub fen: String,
    pub moves: String,
    pub rating: u32,
    pub themes: Vec<String>,
    #[serde(rename = "gameUrl")]
    pub game_url: Option<String>,
}

pub fn lichess(bytes: &[u8]) -> Result<String, String> {
    if bytes.len() > 4 * 1024 * 1024 {
        return Err("Export too large; select a smaller subset".into());
    }
    let rows: Vec<LichessRow> = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
    if rows.is_empty() || rows.len() > 4096 {
        return Err("Expected 1–4096 selected puzzles".into());
    }
    let header = Header {
        schema_version: 1,
        collection_id: "lichess-tactics".into(),
        title: "Lichess · Selected study".into(),
        source: "https://database.lichess.org/#puzzles".into(),
        license: "CC0-1.0".into(),
    };
    let mut output = serde_json::to_string(&header).map_err(|e| e.to_string())? + "\n";
    for row in rows {
        let moves: Vec<_> = row.moves.split_whitespace().collect();
        let Some((setup, line)) = moves.split_first() else {
            return Err("Missing setup move".into());
        };
        let mut position = rules::position(&row.fen)?;
        rules::play(&mut position, setup)?;
        let puzzle = Puzzle {
            id: row.id,
            fen: Fen::from_position(position.clone(), EnPassantMode::Legal).to_string(),
            side_to_move: if position.turn().is_white() {
                "white"
            } else {
                "black"
            }
            .into(),
            solution: line.iter().map(|m| m.to_string()).collect(),
            explanation: String::new(),
            difficulty: Some(row.rating),
            tags: row.themes,
            source_url: row.game_url,
        };
        puzzle.validate()?;
        output.push_str(&serde_json::to_string(&puzzle).map_err(|e| e.to_string())?);
        output.push('\n');
    }
    Pack::parse(output.clone())?;
    Ok(output)
}

#[derive(Deserialize)]
struct BookExport {
    #[serde(rename = "bookId")]
    id: String,
    #[serde(rename = "bookTitle")]
    title: String,
    problems: Vec<BookProblem>,
}
#[derive(Deserialize)]
struct BookProblem {
    problemid: u32,
    fen: String,
    moves: String,
    #[serde(default)]
    explanation: String,
    #[serde(default)]
    difficulty: Option<u32>,
}
/// Only the inspected interactive `problems` format. Positional/self-assessed
/// books and inferred/engine-generated continuations are intentionally excluded.
pub fn trainer(bytes: &[u8], ids: &[u32]) -> Result<String, String> {
    if bytes.len() > 4 * 1024 * 1024 || ids.is_empty() || ids.len() > 100 {
        return Err("Select 1–100 problem IDs from an export no larger than 4 MiB".into());
    }
    let book: BookExport = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
    let header = Header {
        schema_version: 1,
        collection_id: book.id,
        title: book.title,
        source: "Selected local trainer export".into(),
        license: "Private; not for redistribution".into(),
    };
    let mut out = serde_json::to_string(&header).map_err(|e| e.to_string())? + "\n";
    for id in ids {
        let matches: Vec<_> = book
            .problems
            .iter()
            .filter(|p| p.problemid == *id)
            .collect();
        if matches.len() != 1 {
            return Err(format!("Problem {id} missing or duplicated"));
        }
        let row = matches[0];
        let pos = rules::position(&row.fen)?;
        let solution: Vec<String> = row
            .moves
            .split(';')
            .map(str::trim)
            .filter(|m| !m.is_empty())
            .map(|m| m.replace('-', "").to_ascii_lowercase())
            .collect();
        let puzzle = Puzzle {
            id: id.to_string(),
            fen: row.fen.clone(),
            side_to_move: if pos.turn().is_white() {
                "white"
            } else {
                "black"
            }
            .into(),
            solution,
            explanation: row.explanation.clone(),
            difficulty: row.difficulty,
            tags: vec![],
            source_url: None,
        };
        puzzle.validate()?;
        let mut final_position = pos;
        for m in &puzzle.solution {
            rules::play(&mut final_position, m)?;
        }
        if !final_position.is_checkmate() {
            return Err(format!("Problem {id} does not end in checkmate"));
        }
        out.push_str(&serde_json::to_string(&puzzle).map_err(|e| e.to_string())?);
        out.push('\n');
    }
    Pack::parse(out.clone())?;
    Ok(out)
}
