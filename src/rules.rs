//! Full standard-chess legality comes from shakmaty, as in Cobalt Lichess.
use shakmaty::{fen::Fen, uci::UciMove, CastlingMode, Chess, Position};

pub fn position(fen: &str) -> Result<Chess, String> {
    fen.parse::<Fen>()
        .map_err(|e| format!("Invalid FEN: {e}"))?
        .into_position(CastlingMode::Standard)
        .map_err(|e| format!("Invalid position: {e}"))
}

pub fn movement(pos: &Chess, uci: &str) -> Result<shakmaty::Move, String> {
    UciMove::from_ascii(uci.as_bytes())
        .map_err(|_| "Malformed UCI".to_owned())?
        .to_move(pos)
        .map_err(|_| "Illegal move".to_owned())
}

pub fn play(pos: &mut Chess, uci: &str) -> Result<(), String> {
    let movement = movement(pos, uci)?;
    pos.play_unchecked(&movement);
    Ok(())
}

pub fn legal_from(pos: &Chess, from: &str) -> Vec<String> {
    pos.legal_moves()
        .iter()
        .map(|m| m.to_uci(CastlingMode::Standard).to_string())
        .filter(|m| m.starts_with(from))
        .collect()
}
