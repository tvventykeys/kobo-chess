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
        let puzzle = row.into_puzzle()?;
        output.push_str(&serde_json::to_string(&puzzle).map_err(|e| e.to_string())?);
        output.push('\n');
    }
    Pack::parse(output.clone())?;
    Ok(output)
}

impl LichessRow {
    pub fn into_puzzle(self) -> Result<Puzzle, String> {
        let moves: Vec<_> = self.moves.split_whitespace().collect();
        let Some((setup, line)) = moves.split_first() else {
            return Err("Missing setup move".into());
        };
        let mut position = rules::position(&self.fen)?;
        let movement = rules::movement(&position, setup)?;
        if movement
            .to_uci(shakmaty::CastlingMode::Standard)
            .to_string()
            != *setup
        {
            return Err("Setup move must use canonical standard UCI".into());
        }
        position.play_unchecked(&movement);
        let puzzle = Puzzle {
            id: self.id,
            fen: Fen::from_position(position.clone(), EnPassantMode::Legal).to_string(),
            side_to_move: if position.turn().is_white() {
                "white"
            } else {
                "black"
            }
            .into(),
            solution: line.iter().copied().map(str::to_owned).collect(),
            explanation: String::new(),
            difficulty: Some(self.rating),
            tags: self.themes,
            source_url: self.game_url,
        };
        puzzle.validate()?;
        Ok(puzzle)
    }
}
