mod board;
mod browse;
mod import;
mod index;
mod pack;
mod progress;
mod rules;
mod session;
#[cfg(test)]
mod tests;

use crate::{
    pack::Pack,
    progress::Progress,
    session::{Played, Session},
};
use kobo_sdk::{action_id, ActionId, Context, KoboApp, Screen, ScreenBuilder, StoreResult};
use shakmaty::{Position, Square};

const KEY: &str = "pocket.progress.v1";
const PREVIEW_KEY: &str = "pocket.preview.80-1500.v1";
const STARTER: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/pocket-starter.idx"));
#[derive(Clone, Copy, PartialEq)]
enum Page {
    Home,
    Collections,
    Browse,
    Line,
    Study,
    Settings,
}
#[derive(Clone, Copy)]
enum Leave {
    Index(usize),
    Browse(usize, browse::Filter),
    Home,
}
#[allow(
    clippy::struct_excessive_bools,
    reason = "UI selection and independent runtime storage acknowledgements are separate flags"
)]
struct Pocket {
    pack: Pack,
    browser: browse::Browser,
    starter_matches: Vec<usize>,
    capacity_notice: bool,
    line_page: usize,
    progress: Progress,
    session: Option<Session>,
    page: Page,
    selected: Option<String>,
    hint: Option<String>,
    promotion: Option<String>,
    leave: Option<Leave>,
    feedback: String,
    storage_notice: Option<String>,
    storage_open: bool,
    ready: bool,
    writable: bool,
    dirty: bool,
    saving: bool,
    storage_waiver: bool,
    exit_requested: bool,
    last_screen: Option<Screen>,
}
impl Pocket {
    fn new(pack: Pack) -> Self {
        let progress = Progress::new(&pack);
        Self {
            pack,
            browser: browse::Browser::default(),
            starter_matches: Vec::new(),
            capacity_notice: false,
            line_page: 0,
            progress,
            session: None,
            page: Page::Home,
            selected: None,
            hint: None,
            promotion: None,
            leave: None,
            feedback: String::new(),
            storage_notice: None,
            storage_open: false,
            ready: false,
            writable: true,
            dirty: false,
            saving: false,
            storage_waiver: false,
            exit_requested: false,
            last_screen: None,
        }
    }
    fn preview(&self) -> bool {
        self.pack.demo_len() < self.pack.len()
    }

    fn store_key(&self) -> &'static str {
        if self.preview() {
            PREVIEW_KEY
        } else {
            KEY
        }
    }

    fn step_index(&self, forward: bool) -> usize {
        let current = self.progress.current;
        if current < self.pack.demo_len() {
            return step(current, self.pack.demo_len(), forward);
        }
        let i = self
            .starter_matches
            .binary_search(&current)
            .expect("active filter includes session");
        self.starter_matches[step(i, self.starter_matches.len(), forward)]
    }

    fn show(&mut self, cx: &mut Context) {
        let screen = self.screen();
        if self.last_screen.as_ref() != Some(&screen) {
            self.last_screen = Some(screen.clone());
            cx.set_screen(screen);
        }
    }
    fn request_exit(&mut self, cx: &mut Context) {
        if !self.dirty && !self.saving {
            cx.exit();
        } else {
            self.exit_requested = true;
            self.flush(cx);
        }
    }
    #[allow(
        clippy::too_many_lines,
        reason = "Keep one declarative screen description for the related study controls"
    )]
    fn screen(&self) -> Screen {
        let mut b = ScreenBuilder::new("polgar-pocket")
            .top_bar("Pocket Chess")
            .owns_back(true);
        if !self.ready {
            return b.heading("Opening your study book…").build();
        }
        if self.exit_requested {
            let mut exit = b.heading("Leaving your study book");
            if self.storage_notice.is_some() {
                exit =
                    exit.text("Progress could not be saved. Exit loses changes from this session.");
                if self.writable {
                    exit = exit.button("save-progress", "Retry save");
                }
                exit = exit.button("exit-unsaved", "Exit without saving");
            } else {
                exit = exit.text("Saving progress before exit…");
            }
            return exit.button("cancel-exit", "Keep studying").build();
        }
        match self.page {
            Page::Home => {
                let completed = self
                    .progress
                    .records
                    .values()
                    .filter(|r| r.completed)
                    .count();
                let clean = self
                    .progress
                    .records
                    .values()
                    .filter(|r| r.clean_solves > 0)
                    .count();
                b = b
                    .heading("Pocket Chess Studies")
                    .secondary("A quiet chess puzzle book. Entirely offline.")
                    .section("Your collection")
                    .button("collections", &self.pack.header.title)
                    .text(format!(
                        "{completed} / {} solved · {clean} clean",
                        self.pack.len()
                    ))
                    .primary_button("continue", "Continue studying")
                    .button("settings", "Settings")
                    .button("exit", "Exit to launcher");
            }
            Page::Browse => b = self.browser.screen(&self.pack, b),
            Page::Line => {
                let session = self.session.as_ref().expect("line has session");
                b = b.heading("Solution line");
                let moves = session.solution_moves();
                for (i, line) in moves.chunks(4).enumerate().skip(self.line_page * 4).take(4) {
                    b = b.secondary(format!(
                        "{}–{}  {}",
                        i * 4 + 1,
                        i * 4 + line.len(),
                        line.join(" ")
                    ));
                }
                if moves.len() > 16 {
                    b = b.buttons([("line-prev", "Earlier moves"), ("line-next", "Later moves")]);
                }
            }
            Page::Collections => {
                b = b
                    .heading("Study collections")
                    .button("select-pack", &self.pack.header.title)
                    .secondary(format!(
                        "{} verified puzzles · {}",
                        self.pack.demo_len(),
                        self.pack.header.license
                    ))
                    .text("Selected for offline study. Original Lichess puzzle IDs are preserved.");
                if self.preview() {
                    b = b.button(
                        "browse-starter",
                        format!("Browse {} more", self.pack.len() - self.pack.demo_len()),
                    );
                }
                b = b.button("home", "Home");
            }
            Page::Settings => {
                b=b.heading("Study settings")
                    .toggle("flip","Flip board","By default your side is at the bottom.",self.progress.flipped)
                    .toggle("coordinates", "Square coordinates", "Label empty squares with their absolute coordinates.", self.progress.coordinates)
                    .text("Text size follows Cobalt Settings. Page buttons go to the previous or next puzzle, with confirmation during an attempt.")
                    .secondary("No Wi-Fi, engine, timers or background tasks. Progress is stored on this reader.")
                    .button("home","Home");
            }
            Page::Study => {
                let s = self.session.as_ref().expect("study has session");
                let black = (s.puzzle.side_to_move == "black") ^ self.progress.flipped;
                let turn = if s.position.turn().is_white() {
                    "White"
                } else {
                    "Black"
                };
                let state = if s.complete() {
                    if s.revealed {
                        "Solution revealed"
                    } else {
                        "Solved"
                    }
                } else if s.position.is_check() {
                    "Check"
                } else {
                    "Find the best move"
                };
                let current = self.progress.current;
                let status = if current < self.pack.demo_len() {
                    format!(
                        "{} · {} / {}",
                        self.pack.header.title,
                        current + 1,
                        self.pack.demo_len()
                    )
                } else {
                    format!(
                        "Lichess {} · Rating {}",
                        s.puzzle.id,
                        self.pack.facets(current).0
                    )
                };
                let (action, label) = if current >= self.pack.demo_len() && s.revealed {
                    ("open-line", "Line")
                } else if s.complete() {
                    ("previous", "Prev")
                } else {
                    ("coordinates", "Coords")
                };
                b = b
                    .top_bar_action(action, label)
                    .secondary(status)
                    .secondary(if s.complete() {
                        format!(
                            "{} exercise · {state}",
                            if s.puzzle.side_to_move == "white" {
                                "White"
                            } else {
                                "Black"
                            }
                        )
                    } else {
                        format!("{turn} to move · {state}")
                    })
                    .board_with_selection(
                        8,
                        board::cells(
                            &s.position,
                            black,
                            self.selected.as_deref(),
                            self.hint.as_deref(),
                            self.progress.coordinates,
                        ),
                    );
                // Coordinate display is optional to keep the board quiet.
                let message = if self.feedback.is_empty() {
                    if s.complete() {
                        if s.revealed {
                            "Solution shown. Retry to practice."
                        } else if s.clean() {
                            "Solved independently."
                        } else {
                            "Solved with assistance."
                        }
                    } else {
                        "Tap a piece, then its destination."
                    }
                } else {
                    &self.feedback
                };
                b = b.secondary(message);
                if s.complete() && !s.puzzle.explanation.is_empty() {
                    b = b.secondary(&s.puzzle.explanation);
                }
                if s.revealed && current < self.pack.demo_len() {
                    b = b.secondary(format!("Line: {}", s.solution_text()));
                }
                b = if s.complete() {
                    b.buttons([("retry", "Retry"), ("home", "Home"), ("next", "Next")])
                } else {
                    b.buttons([("hint", "Hint"), ("retry", "Retry"), ("reveal", "Reveal")])
                        .buttons([("previous", "Prev"), ("home", "Home"), ("next", "Next")])
                };
                if let Some(base) = &self.promotion {
                    b = b.dialog("Promote pawn", "cancel", |d| {
                        d.text(format!("Choose a piece for {base}"))
                            .buttons([("promote-q", "Queen"), ("promote-r", "Rook")])
                            .buttons([("promote-b", "Bishop"), ("promote-n", "Knight")])
                            .button("cancel", "Cancel")
                    });
                }
            }
        }
        if let Some(text) = &self.storage_notice {
            b = b.top_bar_action("storage-open", "Storage");
            if self.storage_open && self.leave.is_none() && self.promotion.is_none() {
                b = b.dialog("Storage needs attention", "storage-dismiss", |d| {
                    let d = d.text(text);
                    let d = if self.writable && !self.storage_waiver {
                        d.text("Allowing sleep may lose unsaved changes.").buttons([
                            ("save-progress", "Retry save"),
                            ("allow-sleep", "Allow sleep"),
                        ])
                    } else {
                        d.button("save-progress", "Retry storage")
                    };
                    d.button("storage-dismiss", "Keep studying")
                });
            }
        }
        if self.capacity_notice {
            b = b.dialog("Preview history is full", "dismiss-capacity", |d| {
                d.text("Saved history is kept. Choose a puzzle you have already studied.")
                    .button("dismiss-capacity", "Close")
            });
        } else if self.leave.is_some() {
            b = b.dialog("Leave this attempt?", "cancel", |d| {
                d.text("History is kept. Switching puzzles starts a fresh attempt.")
                    .buttons([("confirm-leave", "Leave"), ("cancel", "Keep studying")])
            });
        }
        b.build()
    }
    fn save(&mut self, cx: &mut Context) {
        self.progress.capture(self.session.as_ref());
        self.dirty = true;
        self.flush(cx);
    }
    fn flush(&mut self, cx: &mut Context) {
        if self.ready
            && self.writable
            && self.dirty
            && !self.saving
            && self.storage_notice.is_none()
        {
            let bytes = serde_json::to_vec(&self.progress).expect("serializable progress");
            cx.store().save(self.store_key(), bytes);
            self.saving = true;
            self.storage_waiver = false;
            self.dirty = false;
        }
    }
    fn start(&mut self, cx: &mut Context, index: usize) {
        self.progress.current = index;
        let s = Session::new(self.pack.puzzle(index));
        self.progress.begin(&s);
        self.session = Some(s);
        self.page = Page::Study;
        self.clear_selection();
        self.feedback.clear();
        self.save(cx);
    }
    fn clear_selection(&mut self) {
        self.selected = None;
        self.hint = None;
        self.promotion = None;
    }
    fn request_leave(&mut self, cx: &mut Context, target: Leave) {
        let index = match target {
            Leave::Index(i) | Leave::Browse(i, _) => Some(i),
            Leave::Home => None,
        };
        if self.preview()
            && self.progress.records.len() >= progress::PREVIEW_RECORD_LIMIT
            && index.is_some_and(|i| !self.progress.records.contains_key(&self.pack.puzzle(i).id))
        {
            self.capacity_notice = true;
        } else if index.is_some_and(|i| i != self.progress.current)
            && self.session.as_ref().is_some_and(Session::active)
        {
            self.leave = Some(target);
        } else {
            self.navigate(cx, target);
        }
    }
    fn navigate(&mut self, cx: &mut Context, target: Leave) {
        self.leave = None;
        self.clear_selection();
        match target {
            Leave::Index(i) => {
                if self.session.is_none() || self.progress.current != i {
                    self.start(cx, i);
                } else {
                    self.page = Page::Study;
                }
            }
            Leave::Browse(i, filter) => {
                self.starter_matches = filter.select(&self.pack);
                self.navigate(cx, Leave::Index(i));
            }
            Leave::Home => {
                self.page = Page::Home;
                self.save(cx);
            }
        }
    }
    fn move_piece(&mut self, cx: &mut Context, uci: &str) {
        let Some(s) = self.session.as_mut() else {
            return;
        };
        match s.play(uci) {
            Err(e) => {
                self.feedback = e;
                return;
            }
            Ok(Played::Wrong) => {
                let r = self
                    .progress
                    .records
                    .get_mut(&s.puzzle.id)
                    .expect("started attempt");
                r.incorrect_attempts = r.incorrect_attempts.saturating_add(1);
                self.feedback = "Legal move, but not this solution. Try again.".into();
            }
            Ok(Played::Correct) => self.feedback = "Correct. Opponent replied; your move.".into(),
            Ok(Played::Complete) => {
                self.feedback = if s.clean() {
                    "Solved independently."
                } else {
                    "Solved with assistance."
                }
                .into();
                self.progress.finish(s);
            }
        }
        self.clear_selection();
        self.save(cx);
    }
    fn tap_square(&mut self, cx: &mut Context, square: String) {
        let Some(s) = self.session.as_ref() else {
            return;
        };
        if s.complete() {
            return;
        }
        let sq: Square = square.parse().expect("square action");
        if s.position
            .board()
            .piece_at(sq)
            .is_some_and(|p| p.color == s.position.turn())
        {
            if self.selected.as_ref() == Some(&square) {
                self.selected = None;
            } else {
                self.selected = Some(square);
            }
            self.hint = None;
            return;
        }
        let Some(from) = self.selected.clone() else {
            return;
        };
        let base = format!("{from}{square}");
        let choices = rules::legal_from(&s.position, &from);
        if choices.iter().any(|m| m.starts_with(&base) && m.len() == 5) {
            self.promotion = Some(base);
            return;
        }
        self.move_piece(cx, &base);
    }
}
impl KoboApp for Pocket {
    fn on_start(&mut self, cx: &mut Context) {
        cx.store().load(self.store_key());
        self.show(cx);
    }
    #[allow(
        clippy::too_many_lines,
        reason = "Keep action routing and its modal guards together"
    )]
    fn on_action(&mut self, cx: &mut Context, a: ActionId) {
        let back = a == ActionId::BACK;
        if !self.ready {
            if back {
                cx.exit();
            }
            return;
        }
        let is = |name: &str| a == action_id(name);
        if self.exit_requested {
            if is("cancel-exit") || back {
                self.exit_requested = false;
            } else if is("save-progress") && self.storage_notice.is_some() && self.writable {
                self.storage_notice = None;
                self.flush(cx);
            } else if is("exit-unsaved") && self.storage_notice.is_some() && !self.saving {
                cx.exit();
            }
            self.show(cx);
            return;
        }
        if self.capacity_notice {
            if is("dismiss-capacity") || back {
                self.capacity_notice = false;
                self.show(cx);
            }
            return;
        }
        if is("storage-open") && self.storage_notice.is_some() {
            self.storage_open = true;
            self.show(cx);
            return;
        }
        if is("storage-dismiss") || (back && self.storage_open) {
            self.storage_open = false;
            self.show(cx);
            return;
        }
        if self.leave.is_some() {
            if is("confirm-leave") {
                let target = self.leave.take().unwrap();
                self.navigate(cx, target);
            } else if is("cancel") || back {
                self.leave = None;
            }
            self.show(cx);
            return;
        }
        if self.promotion.is_some() {
            if is("cancel") || back {
                self.promotion = None;
            } else {
                for role in ['q', 'r', 'b', 'n'] {
                    if is(&format!("promote-{role}")) {
                        let base = self.promotion.take().unwrap();
                        self.move_piece(cx, &format!("{base}{role}"));
                        break;
                    }
                }
            }
            self.show(cx);
            return;
        }
        if is("allow-sleep") && self.storage_notice.is_some() && !self.saving {
            self.storage_waiver = true;
            self.storage_open = false;
        } else if is("exit") || (back && self.page == Page::Home) {
            self.request_exit(cx);
        } else if is("save-progress") {
            self.storage_open = false;
            if self.writable {
                self.storage_notice = None;
                self.flush(cx);
            } else {
                self.ready = false;
                cx.store().load(self.store_key());
            }
        } else if self.page == Page::Line {
            if back {
                self.page = Page::Study;
            } else if is("line-prev") {
                self.line_page = self.line_page.saturating_sub(1);
            } else if is("line-next")
                && (self.line_page + 1) * 16
                    < self
                        .session
                        .as_ref()
                        .expect("line has session")
                        .puzzle
                        .solution
                        .len()
            {
                self.line_page += 1;
            }
        } else if back && self.page == Page::Browse {
            self.page = Page::Collections;
        } else if back || is("home") {
            self.request_leave(cx, Leave::Home);
        } else if is("browse-starter") && self.page == Page::Collections && self.preview() {
            self.browser.refresh(&self.pack);
            self.page = Page::Browse;
        } else if is("collections") {
            self.page = Page::Collections;
        } else if is("settings") {
            self.page = Page::Settings;
        } else if is("coordinates") {
            self.progress.coordinates = !self.progress.coordinates;
            self.save(cx);
        } else if is("flip") {
            self.progress.flipped = !self.progress.flipped;
            self.save(cx);
        } else if is("continue") || is("select-pack") {
            let index = if is("select-pack") && self.progress.current >= self.pack.demo_len() {
                0
            } else {
                self.progress.current
            };
            self.request_leave(cx, Leave::Index(index));
        } else if self.page == Page::Browse {
            if let Some(index) = self.browser.action(a, &self.pack) {
                self.request_leave(cx, Leave::Browse(index, self.browser.filter));
            }
        } else if self.page == Page::Study {
            if is("next") || is("previous") {
                self.request_leave(cx, Leave::Index(self.step_index(is("next"))));
            } else if is("open-line")
                && self.progress.current >= self.pack.demo_len()
                && self.session.as_ref().is_some_and(|s| s.revealed)
            {
                self.line_page = 0;
                self.page = Page::Line;
            } else if is("retry") {
                let s = self.session.as_mut().unwrap();
                s.retry();
                self.progress.begin(s);
                self.clear_selection();
                self.feedback = "Fresh attempt. Earlier history is kept.".into();
                self.save(cx);
            } else if is("hint") {
                let s = self.session.as_mut().unwrap();
                let prior = s.hints;
                if let Some(square) = s.hint() {
                    self.selected = None;
                    self.hint = Some(square.clone());
                    self.feedback = format!("Hint: move the piece on {square}.");
                    if prior == 0 {
                        let record = self.progress.records.get_mut(&s.puzzle.id).unwrap();
                        record.hints_used = record.hints_used.saturating_add(1);
                        self.save(cx);
                    }
                }
            } else if is("reveal") {
                let s = self.session.as_mut().unwrap();
                if s.complete() {
                    self.show(cx);
                    return;
                }
                s.reveal();
                self.progress.finish(s);
                self.clear_selection();
                self.feedback = "Revealed. Retry to practice the line.".into();
                self.save(cx);
            } else if let Some(sq) = board::square_action(a) {
                self.tap_square(cx, sq);
            }
        }
        self.show(cx);
    }
    fn on_page_turn(&mut self, cx: &mut Context, forward: bool) {
        if self.page == Page::Study {
            self.on_action(cx, action_id(if forward { "next" } else { "previous" }));
        }
    }
    fn on_load(&mut self, cx: &mut Context, key: &str, result: StoreResult) {
        if key != self.store_key() && !(self.preview() && key == KEY && !self.ready) {
            return;
        }
        if self.preview()
            && key == PREVIEW_KEY
            && matches!(&result, StoreResult::Loaded { value: None, .. })
        {
            cx.store().load(KEY);
            return;
        }
        if let StoreResult::Loaded { value, .. } = result {
            let restored = value
                .map(|bytes| {
                    if self.preview() && key == KEY {
                        Progress::decode(&bytes, &bundled_pack(&[])?)
                    } else {
                        Progress::decode(&bytes, &self.pack)
                    }
                })
                .transpose();
            if let Ok(p) = restored {
                self.progress = p.unwrap_or_else(|| Progress::new(&self.pack));
                self.session = self.progress.restore(&self.pack).unwrap();
                self.starter_matches = if self.progress.current >= self.pack.demo_len() {
                    browse::Filter::default().select(&self.pack)
                } else {
                    Vec::new()
                };
                self.writable = true;
                self.storage_notice = None;
            } else {
                self.writable = false;
                self.storage_notice =
                    Some("Progress damaged. Original preserved; study is read-only.".into());
            }
        } else {
            self.writable = false;
            self.storage_notice =
                Some("Cannot read progress. Study is read-only; retry storage.".into());
        }
        // A storage retry can replace the active snapshot entirely. Always land
        // on Home, so a missing session cannot leave Study without a board.
        self.storage_open = self.storage_notice.is_some();
        self.page = Page::Home;
        self.clear_selection();
        self.feedback.clear();
        self.dirty = false;
        self.saving = false;
        self.storage_waiver = false;
        self.exit_requested = false;
        self.ready = true;
        self.show(cx);
    }
    fn on_save(&mut self, cx: &mut Context, key: &str, result: StoreResult) {
        if key != self.store_key() {
            return;
        }
        self.saving = false;
        if matches!(result, StoreResult::Saved { .. }) {
            self.storage_waiver = false;
            self.flush(cx);
            if self.exit_requested && !self.saving && !self.dirty {
                cx.exit();
            }
        } else {
            self.dirty = true;
            self.storage_open = true;
            self.storage_notice = Some("Progress not saved. Retry storage before leaving.".into());
            self.show(cx);
        }
    }
    fn can_suspend(&self) -> bool {
        self.ready && (!self.writable || self.storage_waiver || (!self.saving && !self.dirty))
    }
    fn on_resume(&mut self, cx: &mut Context) {
        self.last_screen = None;
        self.show(cx);
    }
}
fn step(current: usize, len: usize, forward: bool) -> usize {
    if forward {
        (current + 1) % len
    } else {
        (current + len - 1) % len
    }
}

fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 4 && args[1] == "--build-lichess-index" {
        return cli_result(
            std::fs::File::open(&args[2])
                .map_err(|e| e.to_string())
                .and_then(|file| index::build(std::io::BufReader::new(file)))
                .and_then(|bytes| std::fs::write(&args[3], bytes).map_err(|e| e.to_string())),
        );
    }
    if args.len() == 3 && args[1] == "--validate-lichess-index" {
        use std::io::Read;
        return cli_result(
            std::fs::File::open(&args[2])
                .and_then(|file| {
                    let mut bytes = Vec::new();
                    file.take((index::MAX_BYTES + 1) as u64)
                        .read_to_end(&mut bytes)?;
                    Ok(bytes)
                })
                .map_err(|e| e.to_string())
                .and_then(|bytes| {
                    let pack = index::Index::parse(&bytes)?;
                    let demo = Pack::parse(pack::BUILTIN.to_owned())?;
                    if (0..demo.len()).any(|i| pack.contains(&demo.puzzle(i).id)) {
                        return Err("Starter repeats a demo puzzle ID".into());
                    }
                    println!("Validated {} indexed CC0 puzzles", pack.len());
                    Ok(())
                }),
        );
    }
    if args.len() == 4 && args[1] == "--import-lichess" {
        let result = std::fs::File::open(&args[2])
            .map_err(|e| e.to_string())
            .and_then(pack::read_input)
            .and_then(|bytes| import::lichess(&bytes))
            .and_then(|pack| std::fs::write(&args[3], pack).map_err(|e| e.to_string()));
        return match result {
            Ok(()) => {
                println!("Validated selected pack written to {}", args[3]);
                std::process::ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("Import failed: {e}");
                std::process::ExitCode::FAILURE
            }
        };
    }
    if args.len() == 3 && args[1] == "--validate-pack" {
        return match std::fs::File::open(&args[2])
            .map_err(|e| e.to_string())
            .and_then(pack::read_input)
            .and_then(|bytes| String::from_utf8(bytes).map_err(|e| e.to_string()))
            .and_then(Pack::parse)
        {
            Ok(pack) => {
                println!(
                    "Validated {} puzzles in {}",
                    pack.len(),
                    pack.header.collection_id
                );
                std::process::ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("Invalid pack: {e}");
                std::process::ExitCode::FAILURE
            }
        };
    }
    if args.len() > 1 {
        eprintln!("Usage: polgar-pocket [--validate-pack FILE | --import-lichess INPUT OUTPUT | --build-lichess-index ROWS OUTPUT | --validate-lichess-index FILE]");
        return std::process::ExitCode::FAILURE;
    }
    let pack = match bundled_pack(STARTER) {
        Ok(pack) => pack,
        Err(e) => return cli_result(Err(format!("Invalid bundled starter: {e}"))),
    };
    match kobo_sdk::run("polgar-pocket", Pocket::new(pack)) {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("polgar-pocket: {e}");
            std::process::ExitCode::FAILURE
        }
    }
}

fn bundled_pack(starter: &'static [u8]) -> Result<Pack, String> {
    let demo = Pack::parse(pack::BUILTIN.to_owned())?;
    if starter.is_empty() {
        Ok(demo)
    } else {
        demo.with_starter(starter)
    }
}

fn cli_result(result: Result<(), String>) -> std::process::ExitCode {
    match result {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            std::process::ExitCode::FAILURE
        }
    }
}
