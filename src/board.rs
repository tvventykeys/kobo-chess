use crate::rules;
use kobo_sdk::{action_id, ActionId, Glyph};
use shakmaty::{Chess, Color, Position, Role, Square};

pub fn square_action(action: ActionId) -> Option<String> {
    Square::ALL
        .into_iter()
        .map(|s| s.to_string())
        .find(|s| action == action_id(&format!("square-{s}")))
}
pub fn cells(
    pos: &Chess,
    black: bool,
    selected: Option<&str>,
    hint: Option<&str>,
    coordinates: bool,
) -> Vec<(String, String, Option<Glyph>, bool)> {
    let legal = selected
        .map(|s| rules::legal_from(pos, s))
        .unwrap_or_default();
    let checked = pos
        .is_check()
        .then(|| pos.board().king_of(pos.turn()))
        .flatten();
    let mut cells = Vec::with_capacity(64);
    for row in 0..8 {
        for col in 0..8 {
            let file = if black { 7 - col } else { col };
            let rank = if black { row } else { 7 - row };
            let sq = Square::from_coords(shakmaty::File::new(file), shakmaty::Rank::new(rank));
            let name = sq.to_string();
            let piece = pos.board().piece_at(sq);
            let destination = legal.iter().any(|m| m[2..4] == name);
            let glyph = piece
                .map(|p| piece_glyph(p.color, p.role))
                .or_else(|| destination.then_some(Glyph::LegalPoint));
            let marked = selected == Some(name.as_str())
                || hint == Some(name.as_str())
                || checked == Some(sq)
                || (destination && piece.is_some());
            // Coordinate is also the accessible name, even when a glyph draws it.
            let label = if glyph.is_some() || coordinates {
                name.clone()
            } else {
                " ".into()
            };
            cells.push((format!("square-{name}"), label, glyph, marked));
        }
    }
    cells
}
fn piece_glyph(color: Color, role: Role) -> Glyph {
    match (color, role) {
        (Color::White, Role::King) => Glyph::ChessWhiteKing,
        (Color::White, Role::Queen) => Glyph::ChessWhiteQueen,
        (Color::White, Role::Rook) => Glyph::ChessWhiteRook,
        (Color::White, Role::Bishop) => Glyph::ChessWhiteBishop,
        (Color::White, Role::Knight) => Glyph::ChessWhiteKnight,
        (Color::White, Role::Pawn) => Glyph::ChessWhitePawn,
        (Color::Black, Role::King) => Glyph::ChessBlackKing,
        (Color::Black, Role::Queen) => Glyph::ChessBlackQueen,
        (Color::Black, Role::Rook) => Glyph::ChessBlackRook,
        (Color::Black, Role::Bishop) => Glyph::ChessBlackBishop,
        (Color::Black, Role::Knight) => Glyph::ChessBlackKnight,
        (Color::Black, Role::Pawn) => Glyph::ChessBlackPawn,
    }
}
