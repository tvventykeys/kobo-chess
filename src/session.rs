use crate::{pack::Puzzle, rules};
use shakmaty::Chess;

#[derive(Clone)]
pub struct Session {
    pub puzzle: Puzzle,
    pub position: Chess,
    pub cursor: usize,
    pub wrong: u32,
    pub hints: u32,
    pub revealed: bool,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Played {
    Wrong,
    Correct,
    Complete,
}
impl Session {
    pub fn new(puzzle: Puzzle) -> Self {
        let position = rules::position(&puzzle.fen).expect("validated puzzle");
        Self {
            puzzle,
            position,
            cursor: 0,
            wrong: 0,
            hints: 0,
            revealed: false,
        }
    }
    pub fn complete(&self) -> bool {
        self.cursor == self.puzzle.solution.len()
    }
    pub fn active(&self) -> bool {
        !self.complete() && (self.cursor > 0 || self.wrong > 0 || self.hints > 0)
    }
    pub fn clean(&self) -> bool {
        self.complete() && self.wrong == 0 && self.hints == 0 && !self.revealed
    }
    pub fn play(&mut self, uci: &str) -> Result<Played, String> {
        if self.complete() {
            return Err("Exercise finished; Retry to study again".into());
        }
        rules::movement(&self.position, uci)?;
        if self.puzzle.solution[self.cursor] != uci {
            self.wrong = self.wrong.saturating_add(1);
            return Ok(Played::Wrong);
        }
        rules::play(&mut self.position, uci)?;
        self.cursor += 1;
        if !self.complete() {
            rules::play(&mut self.position, &self.puzzle.solution[self.cursor])?;
            self.cursor += 1;
        }
        Ok(if self.complete() {
            Played::Complete
        } else {
            Played::Correct
        })
    }
    pub fn hint(&mut self) -> Option<String> {
        if self.complete() {
            return None;
        }
        // One hint level, one charge per attempt. Only the source is exposed.
        self.hints = 1;
        Some(self.puzzle.solution[self.cursor][..2].to_owned())
    }
    pub fn reveal(&mut self) {
        if self.complete() {
            return;
        }
        self.revealed = true;
        while !self.complete() {
            rules::play(&mut self.position, &self.puzzle.solution[self.cursor])
                .expect("validated line");
            self.cursor += 1;
        }
    }
    pub fn retry(&mut self) {
        *self = Self::new(self.puzzle.clone());
    }
}
