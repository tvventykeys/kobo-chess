# Build artifacts and first device test

For v0.2, read [the exact signing investigation](COBALT-SIGNING.md) and
[the conditional hardware, uninstall and rollback guide](HARDWARE-TEST.md) first.
There is no authorized device operation in this repository task.

No Kobo was connected, probed or modified. KOReader, Focus Reader, Nickel,
startup hooks, firmware and installed Cobalt were untouched. This application is
an independent native Cobalt process; it is not a KOReader plugin or web wrapper.

Build using `bash scripts/verify-pocket.sh`. The ARM executable is:

```
/workspace/build/cobalt/armv7-unknown-linux-musleabihf/release/polgar-pocket
```

It is a statically linked ARMv7 hard-float musl EABI5 executable, with no ELF
interpreter or dependency on Kobo system shared libraries. Host simulator builds
are separate. The public puzzle pack is embedded, so no data download is needed.

Generate and verify a **verification-only** Cobalt bundle:

```bash
bash scripts/package-pocket.sh /workspace/verification/pocket-bundle
```

This writes a `.cobaltapp` file using Cobalt's actual `COBALTAP` pathless format,
release-shaped manifest (including binary SHA-256 and size), local development
public key and a private development signing seed. Keep that seed local and
out of commits/deliveries. `kobo app-verify` checks the signature, manifest and
exact binary. It does not install anything or publish to a Store.

**Installation limitation:** the pinned Cobalt CLI packages contributed apps via
its upstream workspace/catalog, and stock runtimes trust Cobalt's release key.
There is no supported arbitrary standalone-app installation/local publish mode
in this revision. A locally signed development bundle is not a Store-trusted,
ready-to-install release. Renaming it to `KoboRoot.tgz`, copying the ELF into
system folders or disabling signature verification is not an installation path.

This project supplies the built executable, source and structurally verified
bundle. Completing owner-controlled hardware installation requires a supported
Cobalt contributor/trusted-release path (or a documented future standalone
development route). That platform step was not performed, and no release key
was requested. The upstream repository was not edited to register this app.
Do not run the existing platform-wide `package` command and assume it includes
this separate crate. No Cobalt Store publication is authorized by this work.

Before a first physical test, review the app/SDK version and trusted installation
route with the owner. Then verify portrait orientation, square hit targets,
brown/paper contrast, coordinate mode, selection/capture/check outlines,
promotion dialog, both page buttons, accidental-navigation guard, normal and
170% text, progress after app restart, real suspend/resume, storage failure,
ghosting and refresh timing. Confirm the reader can return to its existing
launcher/readers. Physical battery life and display behavior must be measured on
the device; simulator success does not establish either.
