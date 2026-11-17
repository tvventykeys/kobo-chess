# First physical test: Kobo Libra Colour with existing readers

**Status: preparation only. Do not install this build yet.** No Kobo has been
connected, probed, changed or tested in this task. A source/ARM build and a
verification-only bundle exist; trusted installation is externally blocked.
This is a conditional owner-operated guide, not permission to execute it now.

## 1. Prerequisites and stop gates

1. Review [COBALT-SIGNING.md](COBALT-SIGNING.md). Obtain an officially supported
   installation route for this app and an authenticated package/catalog. The
   development fixture key is not trusted by stock Cobalt. **Stop here while
   that route is unresolved.** No ELF copying, key replacement or signature bypass.
2. Record the model (N428/code 390), firmware, Cobalt version if already installed,
   and how NickelMenu, KOReader, Zen OS and Focus Reader currently launch/exit.
   Do not post a full serial number, credentials, private books or databases.
3. The inspected upstream matrix lists Libra Colour 4.45.23697 and 4.46.23836,
   kernel 4.9.77; colour and suspend/resume proof has limits even upstream.
   Other firmware needs its documented compatibility decision; firmware 5.x is
   unsupported by this Cobalt/NickelMenu route. Do not upgrade/downgrade firmware
   to satisfy this test. Our simulator covers `libra-colour-390`, not your device.
4. Determine whether Zen OS changes the launcher, boot flow, stock Nickel process,
   power handling or display ownership. No compatibility evidence for that stack
   was found in Cobalt's inspected docs. If the normal return-to-Nickel path is
   absent or unknown, **stop for a documented compatible handoff**. Do not rename
   processes, kill services, disable Zen OS or replace startup hooks to force a test.
5. If Cobalt is absent, platform installation is a separate owner-reviewed change.
   An app test must not silently install Cobalt, reinstall NickelMenu, enable SSH
   or change Wi-Fi/power settings. If it is present, do not replace the platform
   just to test this app. Check the minimum runtime before considering an update.
6. Charge the device comfortably (recommend at least 50%), keep a known data USB
   cable and a computer available, and learn the existing safe reboot procedure
   from the reader's manual. Schedule a short attended test with no concurrent
   reader or firmware update and no pending `.kobo/KoboRoot.tgz` installation.

## 2. Private backups before any authorized installation

1. Exit each reader normally; return to its known launcher, not a suspended app.
2. Connect in the reader's normal USB storage mode. With no app/database writing,
   copy the visible storage including hidden `.adds` and `.kobo` directories to
   a dated **private local backup**. Preserve the complete NickelMenu folder and
   KOReader/Focus Reader/Zen OS folders/configuration at their actual locations;
   do not assume every installation uses the same paths. Keep any existing
   Cobalt app/state/data/store folders and the `.adds/cobalt-launch.sh` bootstrap.
3. Record the current `.adds/nm` configuration and Cobalt-only entry, if any.
   Keep the owner-side installer version and prior authenticated package metadata.
   Do not upload this backup to Git or this public app repository.
4. Check that the backup is readable, has plausible sizes, and includes the
   intended hidden folders. Keep it unchanged until every existing reader has
   been retested. This storage backup is not a full firmware/partition image.
5. Eject normally. Do not leave USB storage mounted while launching an app.

## 3. Conditional installation procedure

There is currently **no executable standalone installation step for this build**.
The exact stock path, once externally approved and legitimately available, is:

1. Verify the app identity `polgar-pocket`, expected version, zero capabilities,
   minimum runtime, authenticated catalog and package SHA-256/size. A successful
   `app-verify` under a supplied development key does not satisfy this gate.
2. If the owner separately approves a maintained Cobalt catalog route, launch the
   existing Cobalt entry from NickelMenu using its normal reader handoff. Open
   **App Store**, find Polgár Trainer Pocket in the approved channel, review its
   details and choose Install. If the app is absent, stop; do not switch the whole
   platform to Beta or hand-edit a catalog as a substitute.
3. Wait for the runtime's successful install acknowledgement and launcher tile.
   Catalog signature, package signature/hash and app identity must all pass.
   If this path becomes a maintainer-documented non-Store personal test route,
   use only that exact reviewed procedure; this document does not invent one.
4. Record the actual installed version and package/catalog identifiers. Install
   only the application, without changing NickelMenu entries or existing readers.

The documented public contribution route requires a maintainer-reviewed upstream
PR and signed Beta publication, which the owner has forbidden for this task.
The prepared contribution tree has not been submitted. Any future publication
requires a separate decision. `kobo setup`, `setup --source`, `deploy`, and the
platform-wide `package` command are not app-only installation commands here.

## 4. Launch, exercise and exit

After every gate above is satisfied, keep the first session to about five minutes:

1. Close KOReader, Focus Reader and Zen OS reader views normally and reach the
   confirmed compatible launch state. Do not run competing display owners.
2. Launch existing Cobalt from its existing NickelMenu entry, then the Polgár
   Pocket tile. Confirm portrait screen, collection name and no network request.
3. Open the collection and tap Continue. Check large brown/paper squares, both
   piece colours, side to move, square selection, legal dots and capture outlines.
   Tap a wrong legal move: the position must remain available with feedback.
4. Hint must identify only the source. Retry must restore the start without
   erasing history. Reveal must show the line without adding a solve. Solve an
   entire line and check the completion message immediately below the board.
5. Try upper/back and lower/forward buttons (the upstream 4.45 mapping is 193/194).
   A partly played/hinted attempt must ask before switching; Cancel preserves it.
   Home/Continue should preserve that attempt without a discard question.
6. Exercise a promotion and choose/cancel explicitly. Check board flip and Coords.
   At normal and 170% system text, all ordinary controls must remain reachable
   with no clipping, overlap or scrolling. Photograph only non-private screens.
7. Go Home, then **Exit to launcher** (Home Back does the same). The app waits for
   pending save acknowledgements. A failed save offers retry or explicit exit
   without saving; do not mistake that loss warning for a successful save.
   Return through Cobalt's normal Back/Reader exit, not by killing processes.
8. Reopen and confirm both solved and partly played attempts survive. Exit again,
   launch each existing reader normally, and verify its settings/books/progress
   and NickelMenu entries. Stop if any previous behavior changed.
9. Only after normal exit/restart is reliable, separately test the sleep cover,
   power suspend/resume and a longer offline study session. Do not infer battery
   life from a five-minute test or from the host's idle CPU observations.

## 5. Failure recovery and app-only uninstall

* **Blank/stuck app:** use Cobalt's runtime-owned Back to leave the app if it still
  responds. If save fails, explicitly choose Exit without saving only if willing
  to lose the unsaved changes. If the runtime is unresponsive, use the reader's
  documented ordinary reboot. Do not invent button-hold durations, factory reset
  or remove files while the device is writing. Reboot return to your particular
  Zen OS/Nickel stack is unverified; have its existing recovery method ready.
* **App refuses to start or fails signature checks:** leave it stopped, preserve
  the package identity and private logs for review, and use the existing reader.
  Do not disable verification or replace another app. A Cobalt `present`/`stop`
  session is an owner-attended device operation; it is not a signing workaround.
* **Storage warning:** retry after resolving available space. “Allow sleep without
  saving” acknowledges that pending changes may be lost on suspend/restart; it
  does not erase or falsely acknowledge saved history. Prefer a successful retry
  before exiting. Corrupted progress is preserved and study becomes read-only;
  back it up before an owner-directed repair. Do not automatically delete it.
* **Uninstall just this app:** in the already working Cobalt App Store, select
  Polgár Trainer Pocket and its Uninstall action. The runtime removes only the
  app installation; its implementation deliberately preserves separate app state.
  Keep that state/backup until the test is accepted. No reader/menu configuration
  should be removed for an app-only uninstall.
* **Rollback:** stop the app and uninstall it; return to the existing reader stack.
  The app's previous slot is an interrupted-transaction recovery mechanism, not a
  guaranteed user version selector. Switching Beta to Stable does not downgrade.
  Reinstall an earlier version only through an authenticated maintainer-supported
  rollback procedure. Do not rename `.prev` folders or overwrite a live binary.
* **Whole-platform removal is outside this app test.** Do not run `kobo setup
  --undo`: it can turn SSH off and change Cobalt's menu/bootstrap/plugin state.
  Although Cobalt preserves shared NickelMenu configurations and moves its data
  to recovery directories, that is not proof of Zen OS/Focus Reader compatibility.
  If full removal is needed, review the exact installer undo plan and your backups
  separately. Never delete all `.adds`, `.kobo`, `.adds/nm` or another reader folder.

## 6. Record acceptance, without overclaiming

Record model/firmware/runtime, approved package digest, launch/exit result, both
page keys, normal/170% text, touch corners, promotion, progress after restart,
true sleep/resume, any visible ghosting and each existing reader's post-test
status. Mark every unrun item untested. A failure ends the test until understood.
No unattended deployment, boot changes, firmware changes, factory reset, Wi-Fi
hold or background engine is part of this procedure.
