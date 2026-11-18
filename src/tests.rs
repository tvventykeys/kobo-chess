use super::*;
use kobo_sdk::{Command, StoreError};
use shakmaty::{fen::Fen, EnPassantMode};

const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
fn pack() -> Pack {
    Pack::parse(pack::BUILTIN.to_owned()).unwrap()
}
fn app() -> (Pocket, Context) {
    let mut p = Pocket::new(pack());
    let mut cx = Context::default();
    p.on_start(&mut cx);
    p.on_load(
        &mut cx,
        KEY,
        StoreResult::Loaded {
            key: KEY.into(),
            value: None,
        },
    );
    let _ = cx.take_commands();
    (p, cx)
}
fn action(p: &mut Pocket, cx: &mut Context, a: &str) {
    p.on_action(
        cx,
        if a == "back" {
            ActionId::BACK
        } else {
            action_id(a)
        },
    );
}
fn fen(pos: &shakmaty::Chess) -> String {
    Fen::from_position(pos.clone(), EnPassantMode::Always).to_string()
}
fn custom(fen: &str, line: &[&str]) -> Session {
    let p = pack::Puzzle {
        id: "test".into(),
        fen: fen.into(),
        side_to_move: if rules::position(fen).unwrap().turn().is_white() {
            "white"
        } else {
            "black"
        }
        .into(),
        solution: line.iter().copied().map(str::to_owned).collect(),
        explanation: String::new(),
        difficulty: None,
        tags: vec![],
        source_url: None,
    };
    p.validate().unwrap();
    Session::new(p)
}
#[test]
fn fen_rejects_malformed() {
    assert!(rules::position("bad FEN").is_err());
}
#[test]
fn fen_rejects_missing_king() {
    assert!(rules::position("8/8/8/8/8/8/8/8 w - - 0 1").is_err());
}
#[test]
fn legal_and_illegal_moves() {
    let p = rules::position(START).unwrap();
    assert!(rules::movement(&p, "e2e4").is_ok());
    assert!(rules::movement(&p, "e2e5").is_err());
    assert!(rules::movement(&p, "e7e5").is_err());
}
#[test]
fn pinned_piece_cannot_expose_king() {
    let p = rules::position("4r1k1/8/8/8/8/8/4R3/4K3 w - - 0 1").unwrap();
    assert!(rules::movement(&p, "e2d2").is_err());
}
#[test]
fn castle_moves_king_and_rook() {
    let mut p = rules::position("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
    rules::play(&mut p, "e1g1").unwrap();
    assert_eq!(
        p.board().piece_at(Square::F1).unwrap().role,
        shakmaty::Role::Rook
    );
    assert_eq!(p.board().king_of(shakmaty::Color::White), Some(Square::G1));
}
#[test]
fn castle_through_check_rejected() {
    let p = rules::position("r3kr1r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
    assert!(rules::movement(&p, "e1g1").is_err());
}
#[test]
fn en_passant_removes_captured_pawn() {
    let mut p = rules::position("4k3/8/8/3pP3/8/8/8/4K3 w - d6 0 2").unwrap();
    rules::play(&mut p, "e5d6").unwrap();
    assert!(p.board().piece_at(Square::D5).is_none());
    assert!(p.board().piece_at(Square::D6).is_some());
}
#[test]
fn en_passant_expiration() {
    let mut p = rules::position("4k3/8/8/3pP3/8/8/8/4K3 w - d6 0 2").unwrap();
    rules::play(&mut p, "e1f1").unwrap();
    rules::play(&mut p, "e8f8").unwrap();
    assert!(rules::movement(&p, "e5d6").is_err());
}
#[test]
fn promotion_requires_piece_and_supports_underpromotion() {
    let p = rules::position("7k/P7/8/8/8/8/8/7K w - - 0 1").unwrap();
    assert!(rules::movement(&p, "a7a8").is_err());
    for role in ['q', 'r', 'b', 'n'] {
        assert!(rules::movement(&p, &format!("a7a8{role}")).is_ok());
    }
}
#[test]
fn multistep_with_deterministic_reply() {
    let mut s = custom(START, &["e2e4", "e7e5", "g1f3"]);
    assert_eq!(s.play("e2e4").unwrap(), Played::Correct);
    assert_eq!(s.cursor, 2);
    assert!(s.position.board().piece_at(Square::E5).is_some());
    assert_eq!(s.play("g1f3").unwrap(), Played::Complete);
    assert!(s.clean());
}
#[test]
fn wrong_legal_move_preserves_position() {
    let mut s = custom(START, &["e2e4"]);
    let before = fen(&s.position);
    assert_eq!(s.play("d2d4").unwrap(), Played::Wrong);
    assert_eq!(s.cursor, 0);
    assert_eq!(fen(&s.position), before);
    assert_eq!(s.wrong, 1);
    s.play("e2e4").unwrap();
    assert!(!s.clean());
}
#[test]
fn illegal_move_cannot_advance() {
    let mut s = custom(START, &["e2e4"]);
    assert!(s.play("e2e6").is_err());
    assert_eq!(s.cursor, 0);
}
#[test]
fn hint_only_identifies_source_once() {
    let mut s = custom(START, &["e2e4"]);
    let before = fen(&s.position);
    assert_eq!(s.hint().as_deref(), Some("e2"));
    assert_eq!(s.hint().as_deref(), Some("e2"));
    assert_eq!(s.hints, 1);
    assert_eq!(fen(&s.position), before);
    assert_eq!(s.cursor, 0);
    s.play("e2e4").unwrap();
    assert!(!s.clean());
}
#[test]
fn revealed_is_never_clean() {
    let mut s = custom(START, &["e2e4", "e7e5", "g1f3"]);
    s.reveal();
    assert!(s.complete());
    assert!(s.revealed);
    assert!(!s.clean());
    assert!(s.play("f1c4").is_err());
}
#[test]
fn retry_resets_board_not_history() {
    let (mut p, mut cx) = app();
    p.start(&mut cx, 0);
    let s = p.session.as_mut().unwrap();
    s.reveal();
    p.progress.finish(s);
    let id = s.puzzle.id.clone();
    action(&mut p, &mut cx, "retry");
    assert_eq!(p.progress.records[&id].attempts, 2);
    assert!(p.progress.records[&id].revealed);
    assert!(!p.progress.records[&id].completed);
    assert_eq!(p.session.as_ref().unwrap().cursor, 0);
}
#[test]
fn completion_only_after_player_line() {
    let (mut p, mut cx) = app();
    p.start(&mut cx, 0);
    let id = p.session.as_ref().unwrap().puzzle.id.clone();
    assert!(!p.progress.records[&id].completed);
    while !p.session.as_ref().unwrap().complete() {
        let s = p.session.as_ref().unwrap();
        let m = s.puzzle.solution[s.cursor].clone();
        p.move_piece(&mut cx, &m);
    }
    assert!(p.progress.records[&id].completed);
    assert_eq!(p.progress.records[&id].clean_solves, 1);
    action(&mut p, &mut cx, "reveal");
    assert_eq!(p.progress.records[&id].clean_solves, 1);
}
#[test]
fn twenty_original_unique_puzzles_replay() {
    let p = pack();
    assert_eq!(p.len(), 20);
    assert_eq!(p.header.collection_id, "lichess-tactics");
    for i in 0..p.len() {
        let mut s = Session::new(p.puzzle(i));
        while !s.complete() {
            let m = s.puzzle.solution[s.cursor].clone();
            s.play(&m).unwrap();
        }
        assert!(s.clean());
    }
}
#[test]
fn pack_rejects_unknown_schema() {
    assert!(
        Pack::parse(pack::BUILTIN.replacen("\"schema_version\":1", "\"schema_version\":2", 1))
            .is_err()
    );
}
#[test]
fn pack_rejects_illegal_line() {
    let p = pack();
    let mut q = p.puzzle(0);
    q.solution[0] = "a1a8".into();
    assert!(q.validate().is_err());
}
#[test]
fn pack_rejects_mismatched_side() {
    let p = pack();
    let mut q = p.puzzle(0);
    q.side_to_move = "purple".into();
    assert!(q.validate().is_err());
}
#[test]
fn pack_rejects_duplicates() {
    let mut data = pack::BUILTIN.to_owned();
    data.push_str(pack::BUILTIN.lines().nth(1).unwrap());
    assert!(Pack::parse(data).is_err());
}
#[test]
fn pack_rejects_truncated_data() {
    assert!(Pack::parse(pack::BUILTIN[..pack::BUILTIN.len() - 20].to_owned()).is_err());
}
#[test]
fn pack_rejects_even_line() {
    let mut q = pack().puzzle(0);
    q.solution = vec!["a1a2".into(), "a8a7".into()];
    assert!(q.validate().is_err());
}
#[test]
fn progress_restores_incomplete_attempt() {
    let (mut p, mut cx) = app();
    p.start(&mut cx, 0);
    let s = p.session.as_mut().unwrap();
    s.hint();
    p.progress.records.get_mut(&s.puzzle.id).unwrap().hints_used = 1;
    p.save(&mut cx);
    let bytes = serde_json::to_vec(&p.progress).unwrap();
    let saved = Progress::decode(&bytes, &p.pack).unwrap();
    let s = saved.restore(&p.pack).unwrap().unwrap();
    assert_eq!(s.hints, 1);
    assert!(!s.complete());
}
#[test]
fn progress_restores_completed_and_revealed() {
    let (mut p, mut cx) = app();
    p.start(&mut cx, 0);
    action(&mut p, &mut cx, "reveal");
    let bytes = serde_json::to_vec(&p.progress).unwrap();
    let saved = Progress::decode(&bytes, &p.pack).unwrap();
    assert!(saved.restore(&p.pack).unwrap().unwrap().revealed);
    assert!(!saved.records.values().any(|r| r.completed));
}
#[test]
fn progress_rejects_corruption_without_overwriting() {
    let (mut p, mut cx) = app();
    p.on_load(
        &mut cx,
        KEY,
        StoreResult::Loaded {
            key: KEY.into(),
            value: Some(b"broken".to_vec()),
        },
    );
    assert!(!p.writable);
    p.start(&mut cx, 0);
    assert!(!cx
        .take_commands()
        .iter()
        .any(|c| matches!(c, Command::Store(kobo_sdk::StoreRequest::Save { .. }))));
}
#[test]
fn pending_saves_coalesce_and_block_suspend() {
    let (mut p, mut cx) = app();
    p.start(&mut cx, 0);
    assert!(!p.can_suspend());
    action(&mut p, &mut cx, "hint");
    assert!(p.dirty);
    p.on_save(&mut cx, KEY, StoreResult::Saved { key: KEY.into() });
    assert!(p.saving);
    p.on_save(&mut cx, KEY, StoreResult::Saved { key: KEY.into() });
    assert!(p.can_suspend());
}
#[test]
fn save_failure_is_visible_and_retryable() {
    let (mut p, mut cx) = app();
    p.start(&mut cx, 0);
    p.on_save(&mut cx, KEY, StoreResult::Denied(StoreError::Unwritable));
    assert!(p.dirty);
    assert!(p.storage_notice.is_some());
    action(&mut p, &mut cx, "save-progress");
    assert!(p.saving);
    assert!(p.storage_notice.is_none());
}
#[test]
fn source_selection_is_not_a_storage_write() {
    let (mut p, mut cx) = app();
    p.start(&mut cx, 0);
    let _ = cx.take_commands();
    let sq = p.session.as_ref().unwrap().puzzle.solution[0][..2].to_string();
    action(&mut p, &mut cx, &format!("square-{sq}"));
    assert!(cx
        .take_commands()
        .iter()
        .all(|c| !matches!(c, Command::Store(_))));
}
#[test]
fn page_buttons_confirm_active_navigation() {
    let (mut p, mut cx) = app();
    p.start(&mut cx, 0);
    action(&mut p, &mut cx, "hint");
    p.on_page_turn(&mut cx, true);
    assert!(p.leave.is_some());
    assert_eq!(p.progress.current, 0);
    action(&mut p, &mut cx, "cancel");
    assert_eq!(p.session.as_ref().unwrap().hints, 1);
    p.on_page_turn(&mut cx, true);
    action(&mut p, &mut cx, "confirm-leave");
    assert_eq!(p.progress.current, 1);
    p.on_page_turn(&mut cx, false);
    assert_eq!(p.progress.current, 0);
}
#[test]
fn board_orientation_and_hint_has_no_destinations() {
    let p = pack();
    let s = Session::new(p.puzzle(0));
    let a = board::cells(&s.position, false, None, Some("d8"), false);
    let b = board::cells(&s.position, true, None, None, false);
    assert_eq!(a[0].0, "square-a8");
    assert_eq!(b[0].0, "square-h1");
    assert!(!a.iter().any(|c| c.2 == Some(kobo_sdk::Glyph::LegalPoint)));
}
#[test]
fn app_makes_no_network_or_polling_commands() {
    let (mut p, mut cx) = app();
    action(&mut p, &mut cx, "continue");
    action(&mut p, &mut cx, "hint");
    action(&mut p, &mut cx, "retry");
    action(&mut p, &mut cx, "reveal");
    for command in cx.take_commands() {
        assert!(matches!(command, Command::SetScreen(_) | Command::Store(_)));
    }
}
#[test]
fn restored_app_keeps_attempt_count() {
    let (mut p, mut cx) = app();
    p.start(&mut cx, 0);
    let bytes = serde_json::to_vec(&p.progress).unwrap();
    let mut q = Pocket::new(pack());
    q.on_load(
        &mut cx,
        KEY,
        StoreResult::Loaded {
            key: KEY.into(),
            value: Some(bytes),
        },
    );
    action(&mut q, &mut cx, "continue");
    assert_eq!(q.progress.records.values().next().unwrap().attempts, 1);
}
#[test]
fn completion_timestamp_only_when_solved() {
    let (mut p, mut cx) = app();
    p.start(&mut cx, 0);
    action(&mut p, &mut cx, "reveal");
    assert!(p
        .progress
        .records
        .values()
        .next()
        .unwrap()
        .completion_timestamp
        .is_none());
}

#[test]
fn lichess_import_applies_setup_and_preserves_ids() {
    let data=br#"[{"puzzleId":"00sO1","fen":"1k1r4/pp3pp1/2p1p3/4b3/P3n1P1/8/KPP2PN1/3rBR1R b - - 2 31","moves":"b8c7 e1a5 b7b6 f1d1","rating":998,"themes":["discoveredAttack"]}]"#;
    let pack = Pack::parse(import::lichess(data).unwrap()).unwrap();
    assert_eq!(pack.header.collection_id, "lichess-tactics");
    let q = pack.puzzle(0);
    assert_eq!(q.id, "00sO1");
    assert_eq!(q.side_to_move, "white");
    assert_eq!(q.solution, vec!["e1a5", "b7b6", "f1d1"]);
    assert!(rules::position(&q.fen)
        .unwrap()
        .board()
        .piece_at(Square::C7)
        .is_some());
}
#[test]
fn importer_rejects_corrupt_and_empty_exports() {
    assert!(import::lichess(b"[]").is_err());
    assert!(import::lichess(b"bad").is_err());
}
#[test]
fn promotion_dialog_requires_explicit_choice() {
    let (mut p, mut cx) = app();
    let s = custom("7k/P7/8/8/8/8/8/7K w - - 0 1", &["a7a8n"]);
    p.progress.begin(&s);
    p.session = Some(s);
    p.page = Page::Study;
    action(&mut p, &mut cx, "square-a7");
    action(&mut p, &mut cx, "square-a8");
    assert!(p.promotion.is_some());
    assert_eq!(p.session.as_ref().unwrap().cursor, 0);
    action(&mut p, &mut cx, "promote-n");
    assert!(p.session.as_ref().unwrap().complete());
}
#[test]
fn coordinates_toggle_persists_and_does_not_flip_actions() {
    let (mut p, mut cx) = app();
    p.start(&mut cx, 0);
    action(&mut p, &mut cx, "coordinates");
    assert!(p.progress.coordinates);
    let cells = board::cells(
        &p.session.as_ref().unwrap().position,
        true,
        None,
        None,
        true,
    );
    assert_eq!(cells[0].0, "square-h1");
    assert_eq!(cells[0].1, "h1");
}
#[test]
fn corrupt_saved_cursor_and_missing_record_rejected() {
    let (mut p, mut cx) = app();
    p.start(&mut cx, 0);
    p.progress.session.as_mut().unwrap().cursor = 999;
    assert!(Progress::decode(&serde_json::to_vec(&p.progress).unwrap(), &p.pack).is_err());
    p.progress.session.as_mut().unwrap().cursor = 0;
    p.progress.records.clear();
    assert!(Progress::decode(&serde_json::to_vec(&p.progress).unwrap(), &p.pack).is_err());
}
// v0.2 regressions found by auditing the real lifecycle and storage paths.
#[test]
fn storage_reload_while_studying_returns_to_valid_home() {
    let (mut p, mut cx) = app();
    p.writable = false;
    p.start(&mut cx, 0);
    action(&mut p, &mut cx, "save-progress");
    p.on_load(
        &mut cx,
        KEY,
        StoreResult::Loaded {
            key: KEY.into(),
            value: None,
        },
    );
    assert!(p.page == Page::Home);
    assert!(p.session.is_none());
}
#[test]
fn home_keeps_partial_attempt_without_leave_confirmation() {
    let (mut p, mut cx) = app();
    p.start(&mut cx, 0);
    action(&mut p, &mut cx, "hint");
    action(&mut p, &mut cx, "home");
    assert!(p.page == Page::Home);
    assert!(p.leave.is_none());
    action(&mut p, &mut cx, "continue");
    assert_eq!(p.session.as_ref().unwrap().hints, 1);
}
#[test]
fn castling_pack_requires_the_standard_uci_the_ui_can_play() {
    let mut puzzle = pack().puzzle(0);
    puzzle.fen = "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1".into();
    puzzle.side_to_move = "white".into();
    puzzle.solution = vec!["e1h1".into()];
    assert!(puzzle.validate().is_err());
    puzzle.solution = vec!["e1g1".into()];
    assert!(puzzle.validate().is_ok());
}
#[test]
fn progress_rejects_fabricated_completion_without_an_attempt() {
    let mut p = Progress::new(&pack());
    let r = progress::Record {
        completed: true,
        ..Default::default()
    };
    p.records.insert(pack().puzzle(0).id, r);
    assert!(Progress::decode(&serde_json::to_vec(&p).unwrap(), &pack()).is_err());
}
#[test]
fn restored_completed_session_requires_matching_history() {
    let (mut p, mut cx) = app();
    p.start(&mut cx, 0);
    let id = p.session.as_ref().unwrap().puzzle.id.clone();
    p.progress.session.as_mut().unwrap().cursor = p.session.as_ref().unwrap().puzzle.solution.len();
    p.progress.records.get_mut(&id).unwrap().completed = false;
    assert!(Progress::decode(&serde_json::to_vec(&p.progress).unwrap(), &p.pack).is_err());
}
#[test]
fn ignored_input_and_repeated_hint_do_not_redraw_or_write() {
    let (mut p, mut cx) = app();
    p.start(&mut cx, 0);
    action(&mut p, &mut cx, "hint");
    let _ = cx.take_commands();
    action(&mut p, &mut cx, "hint");
    action(&mut p, &mut cx, "unrecognized-input");
    assert!(cx.take_commands().is_empty());
}

#[test]
fn exit_waits_for_all_coalesced_writes() {
    let (mut p, mut cx) = app();
    p.start(&mut cx, 0);
    action(&mut p, &mut cx, "hint");
    let _ = cx.take_commands();
    action(&mut p, &mut cx, "home");
    action(&mut p, &mut cx, "back");
    assert!(p.exit_requested);
    assert!(!cx
        .take_commands()
        .iter()
        .any(|c| matches!(c, Command::Exit)));
    p.on_save(&mut cx, KEY, StoreResult::Saved { key: KEY.into() });
    assert!(!cx
        .take_commands()
        .iter()
        .any(|c| matches!(c, Command::Exit)));
    p.on_save(&mut cx, KEY, StoreResult::Saved { key: KEY.into() });
    assert!(cx
        .take_commands()
        .iter()
        .any(|c| matches!(c, Command::Exit)));
}
#[test]
fn clean_home_back_exits_and_failed_exit_requires_explicit_loss_consent() {
    let (mut p, mut cx) = app();
    action(&mut p, &mut cx, "back");
    assert!(cx
        .take_commands()
        .iter()
        .any(|c| matches!(c, Command::Exit)));
    p.start(&mut cx, 0);
    p.on_save(&mut cx, KEY, StoreResult::Denied(StoreError::Unwritable));
    action(&mut p, &mut cx, "home");
    action(&mut p, &mut cx, "exit");
    assert!(p.exit_requested);
    assert!(!cx
        .take_commands()
        .iter()
        .any(|c| matches!(c, Command::Exit)));
    action(&mut p, &mut cx, "exit-unsaved");
    assert!(cx
        .take_commands()
        .iter()
        .any(|c| matches!(c, Command::Exit)));
}
#[test]
fn failed_storage_cannot_block_sleep_forever_after_acknowledgement() {
    let (mut p, mut cx) = app();
    p.start(&mut cx, 0);
    p.on_save(&mut cx, KEY, StoreResult::Denied(StoreError::NoRoom));
    assert!(!p.can_suspend());
    action(&mut p, &mut cx, "allow-sleep");
    assert!(p.can_suspend());
    assert!(p.dirty);
    assert!(p.storage_notice.is_some());
    action(&mut p, &mut cx, "save-progress");
    assert!(!p.can_suspend());
    p.on_save(&mut cx, KEY, StoreResult::Saved { key: KEY.into() });
    assert!(p.can_suspend());
    assert!(!p.storage_waiver);
}
#[test]
fn restored_completed_screen_has_completion_feedback() {
    let (mut p, mut cx) = app();
    p.start(&mut cx, 0);
    while !p.session.as_ref().unwrap().complete() {
        let s = p.session.as_ref().unwrap();
        let uci = s.puzzle.solution[s.cursor].clone();
        p.move_piece(&mut cx, &uci);
    }
    let bytes = serde_json::to_vec(&p.progress).unwrap();
    let mut restored = Pocket::new(pack());
    restored.on_load(
        &mut cx,
        KEY,
        StoreResult::Loaded {
            key: KEY.into(),
            value: Some(bytes),
        },
    );
    let _ = cx.take_commands();
    action(&mut restored, &mut cx, "continue");
    let rendered = format!("{:?}", cx.take_commands());
    assert!(rendered.contains("Solved independently."));
    assert!(!rendered.contains("Tap a piece, then its destination."));
}
#[test]
fn solution_uses_readable_san_including_checkmate_and_castling() {
    assert_eq!(
        custom("7k/5Q2/6K1/8/8/8/8/8 w - - 0 1", &["f7g7"]).solution_text(),
        "Qg7#"
    );
    assert_eq!(
        custom("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1", &["e1g1"]).solution_text(),
        "O-O"
    );
}
#[test]
fn pending_promotion_ignores_page_buttons_and_cancel_preserves_position() {
    let (mut p, mut cx) = app();
    let s = custom("7k/P7/8/8/8/8/8/7K w - - 0 1", &["a7a8n"]);
    p.progress.begin(&s);
    p.session = Some(s);
    p.page = Page::Study;
    action(&mut p, &mut cx, "square-a7");
    action(&mut p, &mut cx, "square-a8");
    p.on_page_turn(&mut cx, true);
    assert_eq!(p.progress.current, 0);
    assert!(p.promotion.is_some());
    action(&mut p, &mut cx, "cancel");
    assert_eq!(p.session.as_ref().unwrap().cursor, 0);
}
#[test]
fn resume_redraws_once_without_writing_and_ordinary_idle_has_no_commands() {
    let (mut p, mut cx) = app();
    p.on_resume(&mut cx);
    assert!(cx
        .take_commands()
        .iter()
        .all(|c| matches!(c, Command::SetScreen(_))));
    assert!(cx.take_commands().is_empty());
}
#[test]
fn back_cancels_navigation_confirmation() {
    let (mut p, mut cx) = app();
    p.start(&mut cx, 0);
    action(&mut p, &mut cx, "hint");
    action(&mut p, &mut cx, "next");
    assert!(p.leave.is_some());
    action(&mut p, &mut cx, "back");
    assert!(p.leave.is_none());
    assert_eq!(p.session.as_ref().unwrap().hints, 1);
}
#[test]
fn unbounded_import_stream_is_stopped_at_the_pack_limit() {
    assert!(pack::read_input(std::io::repeat(b'x')).is_err());
    assert_eq!(
        pack::read_input(std::io::Cursor::new(b"small")).unwrap(),
        b"small"
    );
}
#[test]
fn v01_progress_fields_and_assistance_survive_the_upgrade() {
    let pack = pack();
    let id = pack.puzzle(0).id;
    let mut records = serde_json::Map::new();
    records.insert(id.clone(),serde_json::json!({"attempts":3,"incorrect_attempts":2,
        "hints_used":1,"revealed":true,"completed":false,"clean_solves":0,"completion_timestamp":null}));
    let old = serde_json::json!({"schema_version":1,"collection_id":"lichess-tactics","revision":9,
        "current":0,"flipped":true,"records":records,
        "session":{"puzzle_id":id,"cursor":0,"wrong":1,"hints":1,"revealed":false}});
    let restored = Progress::decode(&serde_json::to_vec(&old).unwrap(), &pack).unwrap();
    assert!(restored.flipped);
    assert!(!restored.coordinates);
    assert_eq!(restored.revision, 9);
    assert_eq!(restored.records[&id].attempts, 3);
    assert!(restored.records[&id].revealed);
    let session = restored.restore(&pack).unwrap().unwrap();
    assert_eq!(session.wrong, 1);
    assert_eq!(session.hints, 1);
    assert!(!session.clean());
}
#[test]
fn readonly_exit_does_not_offer_an_impossible_save_retry() {
    let (mut p, mut cx) = app();
    p.on_load(
        &mut cx,
        KEY,
        StoreResult::Loaded {
            key: KEY.into(),
            value: Some(b"corrupt".to_vec()),
        },
    );
    p.start(&mut cx, 0);
    action(&mut p, &mut cx, "home");
    action(&mut p, &mut cx, "exit");
    assert!(p.exit_requested);
    let _ = cx.take_commands();
    assert!(!format!("{:?}", p.screen()).contains("Retry save"));
    action(&mut p, &mut cx, "save-progress");
    assert!(p.storage_notice.is_some());
    action(&mut p, &mut cx, "exit-unsaved");
    assert!(cx
        .take_commands()
        .iter()
        .any(|c| matches!(c, Command::Exit)));
}

#[test]
fn contributor_clara_completion_and_storage_dialog_fit_at_170_percent() {
    use kobo_ui::{DiagnosticSeverity, DisplayMetrics, TextScale, CLARA_BW_METRICS};
    kobo_text::install(CLARA_BW_METRICS).unwrap();
    let (mut app, mut context) = app();
    app.start(&mut context, 0);
    action(&mut app, &mut context, "reveal");
    for text_scale in [TextScale::Default, TextScale::Largest] {
        let metrics = DisplayMetrics {
            text_scale,
            ..CLARA_BW_METRICS
        };
        let errors: Vec<_> = app
            .screen()
            .validate(&metrics)
            .into_iter()
            .filter(|issue| issue.severity == DiagnosticSeverity::Error)
            .collect();
        assert!(errors.is_empty(), "revealed {text_scale:?}: {errors:?}");
    }
    app.on_save(
        &mut context,
        KEY,
        StoreResult::Denied(StoreError::Unwritable),
    );
    assert!(app.storage_open);
    for text_scale in [TextScale::Default, TextScale::Largest] {
        let metrics = DisplayMetrics {
            text_scale,
            ..CLARA_BW_METRICS
        };
        let errors: Vec<_> = app
            .screen()
            .validate(&metrics)
            .into_iter()
            .filter(|issue| issue.severity == DiagnosticSeverity::Error)
            .collect();
        assert!(errors.is_empty(), "storage {text_scale:?}: {errors:?}");
    }
    action(&mut app, &mut context, "storage-dismiss");
    assert!(app.storage_notice.is_some());
    assert!(!app.storage_open);
}

fn indexed_rows(rows: &str) -> String {
    format!(
        "{{\"source\":\"{}\",\"license\":\"CC0-1.0\",\"input_sha256\":\"{}\",\"selection\":\"test\"}}\n{rows}",
        index::SOURCE,
        "0".repeat(64)
    )
}

const INDEX_ROWS: &str = concat!(
    "[\"zz003\",\"1k1r4/pp3pp1/2p1p3/4b3/P3n1P1/8/KPP2PN1/3rBR1R b - - 2 31\",\"b8c7 e1a5 b7b6 f1d1\",1500,[\"fork\"]]\n",
    "[\"zz001\",\"1k1r4/pp3pp1/2p1p3/4b3/P3n1P1/8/KPP2PN1/3rBR1R b - - 2 31\",\"b8c7 e1a5 b7b6 f1d1\",998,[\"mate\"]]\n",
    "[\"zz002\",\"1k1r4/pp3pp1/2p1p3/4b3/P3n1P1/8/KPP2PN1/3rBR1R b - - 2 31\",\"b8c7 e1a5 b7b6 f1d1\",1100,[\"mate\"]]\n",
);

#[test]
fn indexed_import_is_deterministic_and_checks_source_and_index() {
    let build = |rows: &str| index::build(indexed_rows(rows).as_bytes());
    let output = build(INDEX_ROWS).unwrap();
    let reversed = INDEX_ROWS.lines().rev().collect::<Vec<_>>().join("\n");
    assert_eq!(output, build(&reversed).unwrap());
    let index = index::Index::parse(&output).unwrap();
    assert_eq!(index.id(0), "zz001");
    assert!(index.contains("zz002"));
    assert!(!index.contains("zz004"));
    let puzzle = index.puzzle(0);
    assert_eq!(puzzle.side_to_move, "white");
    assert_eq!(puzzle.difficulty, Some(998));
    assert_eq!(puzzle.tags, ["mate"]);
    assert_eq!(puzzle.solution, ["e1a5", "b7b6", "f1d1"]);
    assert!(rules::position(&puzzle.fen)
        .unwrap()
        .board()
        .piece_at(Square::C7)
        .is_some());
    for rows in [
        INDEX_ROWS.replace("b8c7", "b8h8"),
        INDEX_ROWS.replace("f1d1", "f1a8"),
        INDEX_ROWS.replace("zz003", "zz001"),
        INDEX_ROWS.replace("1500", "1501"),
        INDEX_ROWS.replace("998", "79"),
    ] {
        assert!(build(&rows).is_err());
    }
    let mut corrupt = output;
    let metadata = u32::from_le_bytes(corrupt[8..12].try_into().unwrap()) as usize;
    corrupt[16 + metadata + 11] ^= 1;
    assert!(index::Index::parse(&corrupt).is_err());
    assert!(index::Index::parse(&corrupt[..12]).is_err());
}

#[test]
fn starter_browse_applies_both_filters_and_keeps_active_attempt() {
    let bytes = index::build(indexed_rows(INDEX_ROWS).as_bytes())
        .unwrap()
        .into_boxed_slice();
    let mut app = Pocket::new(bundled_pack(Box::leak(bytes)).unwrap());
    let mut cx = Context::default();
    app.ready = true;
    action(&mut app, &mut cx, "collections");
    action(&mut app, &mut cx, "browse-starter");
    action(&mut app, &mut cx, "rating-filter");
    action(&mut app, &mut cx, "theme-filter");
    action(&mut app, &mut cx, "pick-0");
    assert_eq!(app.session.as_ref().unwrap().puzzle.id, "zz001");
    action(&mut app, &mut cx, "hint");
    app.on_page_turn(&mut cx, true);
    assert!(app.leave.is_some());
    action(&mut app, &mut cx, "cancel");
    assert_eq!(app.session.as_ref().unwrap().hints, 1);
    app.on_page_turn(&mut cx, true);
    action(&mut app, &mut cx, "confirm-leave");
    assert_eq!(app.session.as_ref().unwrap().puzzle.id, "zz002");
    app.on_page_turn(&mut cx, true);
    assert_eq!(app.session.as_ref().unwrap().puzzle.id, "zz001");
    action(&mut app, &mut cx, "reveal");
    action(&mut app, &mut cx, "open-line");
    assert!(app.page == Page::Line);
    action(&mut app, &mut cx, "back");
    assert!(app.page == Page::Study);
}

#[test]
fn preview_history_limit_fits_sdk_store_even_with_largest_counters() {
    let (app, _) = app();
    let mut progress = Progress::new(&app.pack);
    progress.records = (0..progress::PREVIEW_RECORD_LIMIT)
        .map(|i| {
            (
                format!("{i:05}"),
                progress::Record {
                    attempts: u32::MAX,
                    incorrect_attempts: u32::MAX,
                    hints_used: u32::MAX,
                    revealed: false,
                    completed: false,
                    clean_solves: u32::MAX,
                    completion_timestamp: Some(u64::MAX),
                },
            )
        })
        .collect();
    progress.current = index::MAX_PUZZLES + 19;
    progress.revision = u64::MAX;
    progress.session = Some(progress::SavedSession {
        puzzle_id: "00000".into(),
        cursor: 63,
        wrong: u32::MAX,
        hints: 1,
        revealed: false,
    });
    assert!(serde_json::to_vec(&progress).unwrap().len() < kobo_sdk::MAX_STORE_VALUE);
}
