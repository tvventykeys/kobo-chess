//! One versioned atomic runtime store value. No direct filesystem writes on Kobo.
use crate::{pack::Pack, rules, session::Session};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Record {
    pub attempts: u32,
    pub incorrect_attempts: u32,
    pub hints_used: u32,
    pub revealed: bool,
    pub completed: bool,
    pub clean_solves: u32,
    pub completion_timestamp: Option<u64>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SavedSession {
    pub puzzle_id: String,
    pub cursor: usize,
    pub wrong: u32,
    pub hints: u32,
    pub revealed: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Progress {
    pub schema_version: u32,
    pub collection_id: String,
    pub revision: u64,
    pub current: usize,
    pub flipped: bool,
    #[serde(default)]
    pub coordinates: bool,
    pub records: BTreeMap<String, Record>,
    pub session: Option<SavedSession>,
}
impl Progress {
    pub fn new(pack: &Pack) -> Self {
        Self {
            schema_version: 1,
            collection_id: pack.header.collection_id.clone(),
            revision: 0,
            current: 0,
            flipped: false,
            coordinates: false,
            records: BTreeMap::new(),
            session: None,
        }
    }
    pub fn decode(bytes: &[u8], pack: &Pack) -> Result<Self, String> {
        if bytes.len() > 60000 {
            return Err("Progress too large".into());
        }
        let value: Self = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
        if value.schema_version != 1
            || value.collection_id != pack.header.collection_id
            || value.current >= pack.len()
            || value.records.len() > pack.len()
        {
            return Err("Incompatible progress".into());
        }
        for (id, r) in &value.records {
            if !pack.contains_id(id)
                || (r.attempts == 0
                    && (r.completed || r.revealed || r.hints_used > 0 || r.incorrect_attempts > 0))
                || (r.completion_timestamp.is_some() && !r.completed)
                || r.clean_solves > r.attempts
                || (r.clean_solves > 0 && !r.completed)
                || r.completion_timestamp.is_some_and(|t| t < 1_577_836_800)
            {
                return Err("Invalid progress record".into());
            }
        }
        value.restore(pack)?;
        Ok(value)
    }
    pub fn restore(&self, pack: &Pack) -> Result<Option<Session>, String> {
        let Some(saved) = &self.session else {
            return Ok(None);
        };
        let mut session = Session::new(pack.puzzle(self.current));
        if !self.records.get(&saved.puzzle_id).is_some_and(|r| {
            r.attempts > 0 && r.hints_used >= saved.hints && r.incorrect_attempts >= saved.wrong
        }) || saved.puzzle_id != session.puzzle.id
            || saved.cursor > session.puzzle.solution.len()
            || (saved.cursor % 2 != 0 && saved.cursor != session.puzzle.solution.len())
            || saved.hints > 1
            || (saved.revealed && saved.cursor != session.puzzle.solution.len())
        {
            return Err("Invalid saved session".into());
        }
        for m in session.puzzle.solution.iter().take(saved.cursor) {
            rules::play(&mut session.position, m)?;
        }
        session.cursor = saved.cursor;
        session.wrong = saved.wrong;
        session.hints = saved.hints;
        session.revealed = saved.revealed;
        let record = &self.records[&saved.puzzle_id];
        if (session.complete() && !session.revealed && !record.completed)
            || (session.revealed && !record.revealed)
            || (session.clean() && record.clean_solves == 0)
        {
            return Err("Saved result does not match history".into());
        }
        Ok(Some(session))
    }
    pub fn capture(&mut self, session: Option<&Session>) {
        self.revision = self.revision.saturating_add(1);
        self.session = session.map(|s| SavedSession {
            puzzle_id: s.puzzle.id.clone(),
            cursor: s.cursor,
            wrong: s.wrong,
            hints: s.hints,
            revealed: s.revealed,
        });
    }
    pub fn begin(&mut self, session: &Session) {
        let r = self.records.entry(session.puzzle.id.clone()).or_default();
        r.attempts = r.attempts.saturating_add(1);
    }
    pub fn finish(&mut self, session: &Session) {
        let r = self.records.entry(session.puzzle.id.clone()).or_default();
        if session.complete() && !session.revealed {
            r.completed = true;
            r.clean_solves = r.clean_solves.saturating_add(u32::from(session.clean()));
            r.completion_timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .ok()
                .map(|d| d.as_secs())
                .filter(|t| *t >= 1_577_836_800);
        }
        r.revealed |= session.revealed;
    }
}
