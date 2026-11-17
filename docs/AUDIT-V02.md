# v0.2 source audit and polish

Started from merged main `85be813`, reading actual application, SDK, simulator,
packaging, launcher and store code. The existing 41 tests were retained. Six
new regressions first reproduced failures against v0.1 (log:
`/workspace/verification/pocket-v02-audit-regressions.log`), then passed after
focused fixes. No chess engine or application architecture replacement was made.

## Correctness and lifecycle changes

* **Reload crash:** after read-only study, a successful retry of a missing store
  value could clear the session while leaving the Study screen selected. Render
  then panicked. Storage reload now resets navigation to Home and clears stale
  selection/feedback, preserving damaged stored bytes instead of overwriting.
* **Unacknowledged exit:** Home previously delegated Back to the runtime, so an
  exit could overtake a queued/coalesced snapshot. Home now offers Exit to launcher;
  Back follows the same path and waits for all successful save acknowledgements.
  A failed save requires retry, staying, or explicit exit without saving.
  Read-only exit does not offer an impossible save retry.
* **Permanent sleep refusal after disk failure:** save failure previously held
  the suspend gate indefinitely. The warning now offers explicit Allow sleep,
  acknowledging potential loss while keeping dirty state and existing bytes.
  Beginning an actual retry revokes the waiver until its acknowledgement.
* **Incorrect Home guard:** Home preserves the active session, so asking to discard
  it was misleading. Home/Continue now preserves it directly; only changing puzzle
  asks during an active attempt. Back cancels that confirmation or promotion.
* **Unplayable accepted castling alias:** shakmaty accepts king-to-rook UCI notation
  (`e1h1`) while the touch board generates standard king destinations (`e1g1`).
  Pack validation now requires the same canonical standard UCI the UI can play.
* **Inconsistent saved results:** corruption could fabricate completion without
  an attempt, or restore a finished session without matching aggregate history.
  Validation now checks these relationships and preserves invalid data read-only.
* **Host import allocation:** file reads are limited to
  4 MiB plus one sentinel byte before parsing, rather than reading an entire database first.
* **Repeated work:** identical screens are retained once and not resubmitted;
  repeat hints, ignored actions and unchanged page events cause no repaint or
  storage command. Resume explicitly redraws. The pack retains its already-built
  ID index, removing repeated whole-pack JSON decoding during progress validation.

Shakmaty remains the authority for FEN and legal moves, pins/check, castling,
en passant and promotion. All twenty original public lines still replay and
solve through touch. Wrong legal moves preserve the current position and count
as incorrect; illegal targets do not advance. Hints identify only a source,
reveal is never a solve, retries retain aggregate history, and completion is
recorded only after the supplied line is played. A fresh retry can later be a
clean attempt; earlier reveal/wrong history remains recorded. Alternate winning
lines and new engine analysis remain out of scope.

## Interface and assets

The large root-level brown/paper board, touch targets and orientation are retained.
Completed exercises identify the solver rather than misleadingly naming the
opponent's next turn. Restored completions show the right feedback. Reveal uses
standard notation, including checks, mates, castling and underpromotion. Hint
and Reveal visibly disable after completion. Error recovery uses a short action
row; a clipping failure at 170% text was reproduced and fixed before acceptance.
Normal and 170% screenshots are inspected for the ordinary and failure screens.

**Cburnett asset limitation:** Lichess's actual
[COPYING.md](https://github.com/lichess-org/lila/blob/master/COPYING.md), blob
`6bfc9405d70b4aacc0ea4e41ee4d99789080aadf`, identifies `public/piece/cburnett` as
Colin M. L. Burnett's GPLv2-or-later work. That is a possible compatible asset
source with attribution/license obligations; it is not a license inferred from
appearance. No Cburnett file was copied. This Cobalt SDK's interactive chess
board accepts a closed `Glyph` per square, not a custom SVG/piece image. Replacing
its renderer would require upstream support or an unsupported board/input
rewrite. v0.2 retains the licensed Cobalt vector pieces with distinct filled
black and outlined white shapes; it does **not** claim to ship Cburnett artwork.
Cobalt stays read-only. The SVG-source URL first tried for a per-set LICENSE did
not exist; the repository-wide copying declaration supplies the actual license.

## Battery and memory evidence

The app has zero network capabilities and requests no Wi-Fi hold, timer,
background job, thread, engine, animation or keep-awake. Its SDK input path blocks
between messages. Meaningful progress changes use one atomic snapshot; selection,
repeat hints and idle do not write. Equal-screen suppression avoids redundant
screen submissions while leaving the runtime's diff/targeted refresh policy in
control. No full-refresh command was added. The screen cache trades a bounded
small allocation for less repeated render work.

The recovery simulator records the application's own Linux process, separately
from the host CLI/renderer: CPU tick delta and paint count over two idle seconds,
thread count and resident memory. These are host observations, not ARM memory,
power, e-ink timing or battery-life measurements. The host simulator itself has
its own service loop, which is not an application timer. Actual suspend, device
CPU/power behavior, ghosting and long-session memory remain hardware tests.

The pack remains a small embedded selected subset. Raw pack bytes and offsets
stay resident; only the current puzzle is decoded for study. The versioned v1
progress key and field types are preserved for the v0.1 upgrade; no reset or
schema migration of real user history is performed.
