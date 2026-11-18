# Prepared Cobalt Beta submission, 0.2.0

This directory is a review packet. **Nothing has been submitted to Cobalt or
published to its Store, and no Kobo has been modified.**

- [Submission steps and approval boundaries](SUBMIT.md)
- [Code/data/art privacy and naming review](NAMING-LICENSE-PRIVACY.md)
- [Filled upstream PR template, not submitted](UPSTREAM-PR.md)
- [Exact upstream base and manifest policy](upstream.json)
- [Complete binary Git patch](cobalt-beta.patch)
- [Proposed app README](README-upstream.md), [smoke route](drive.kobo),
  [root attribution](upstream-third-party.md), [icon specification](icon.md)
- [Authentic screenshot inventory](CAPTURES.json), [PNGs](screenshots/),
  [capture sidecars](capture-metadata/)
- [Exact validation report](../docs/SUBMISSION-VALIDATION.md) and
  [recorded check results/logs](checks/RESULTS.json)

The complete local branch is `feat/polgar-pocket-beta` at
`ba5ad80d0e2f2151ec29e610f81867a4b3bb763a` in the separate Cobalt clone.
The binary patch applies to `1cc5b6412c4229b85e9960dd956c3d26c4ffd66e`:

```bash
git apply --check /path/to/cobalt-beta.patch
git apply /path/to/cobalt-beta.patch
```

Alternatively, from the app repository with its toolchain environment loaded,
run `python3 scripts/prepare-cobalt-beta.py /path/to/clean/pinned/Cobalt`.
Both methods recreate the same 47 reviewed files. Never apply them to the
read-only pinned SDK. Review current upstream before a future submission.

The local preview `.cobalt-app` is deliberately **not included** in this source
packet: it uses the public fixture key, is not trusted by stock Cobalt and must
not be presented as device-ready. The official dry-run recreates it locally.
No release seed, personal history, private book pack or device backup is included.
