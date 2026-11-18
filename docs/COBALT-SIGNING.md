# Cobalt signing investigation — refreshed 18 November 2026

Cobalt Beta still resolves to
`1cc5b6412c4229b85e9960dd956c3d26c4ffd66e` and main to
`5ac30a46efce23d7fa0d1cb12a499c9b57b816b0`, verified via remote refs.
The current Beta contributor, install, development and release instructions
describe host simulation and protected Store delivery. The Beta device install
entry point uses the compiled-in release key and a signed channel catalog.
I found no documented, supported non-Store private installation route for this
standalone app. Local fixture signing remains useful for package validation.

## Requirements and evidence

| Question | Verified answer |
| --- | --- |
| Official application format? | A pathless `COBALTAP` bundle: bounded canonical manifest, detached Ed25519 signature and one executable. Manifest includes identity, version, minimum Cobalt, capabilities, SHA-256 and byte length. Package and catalog have their own integrity checks. |
| Is a developer key sufficient? | No. `app-bundle`/`app-verify` can round-trip a development fixture. Stock runtime `public_key()` chooses the compiled-in release public key, not a caller-selected key. |
| Is a trusted package alone sufficient? | No. Installation reads the trusted signed channel catalog, finds the app entry, checks package length/hash/signature and matches its manifest against that entry before atomic activation. |
| Is owner key enrollment or unsigned sideloading exposed? | No such stock app-store route was found. The explicit `*_using` key/transport interfaces are host fixture APIs; device entry points use the release key. Simulator test keys are not device trust configuration. |
| Can this development package be installed privately without Store publication? | No supported command was found for this standalone app. Do not change keys, forge a catalog, replace a built-in app or copy an ELF into managed slots. Maintainers would need to document/approve a non-Store personal-testing route. |
| Does source development exist? | Yes: `kobo dev` runs SDK apps on the host, `setup --source` builds the platform workspace, and owner-attended `present` runs known installed apps. Those do not automatically register this independent crate. `present` resolves an allowlist before starting a bounded panel session. |
| Who signs official third-party apps? | Cobalt's contributor route targets `beta`. Human maintainers review purpose, licensing, capabilities and setup. After merge, an isolated publish job receives `COBALT_APP_SIGNING_SEED`, creates packages plus the Beta catalog, and publishes immutable assets before the catalog pointer. Contributors never receive the seed. |
| Does that satisfy the owner's no-publication constraint? | No. An upstream contribution merge triggers Beta Store publication. It requires a separate owner decision; it was not submitted here. Stable promotion is another explicit maintainer action. |
| Is switching back to Stable an app downgrade? | No. Runtime tests and install logic prevent silent downgrade of an installed higher version. Do not use a channel toggle as a rollback promise. |

Sources:

* [Contributor route, licensing and automated Beta/Stable signing](https://github.com/BandarLabs/Cobalt/blob/1cc5b6412c4229b85e9960dd956c3d26c4ffd66e/docs/CONTRIBUTING_APPS.md)
* [Bundle format and signature verification](https://github.com/BandarLabs/Cobalt/blob/1cc5b6412c4229b85e9960dd956c3d26c4ffd66e/crates/kobo-app-store/src/bundle.rs)
* [Compiled-in release key](https://github.com/BandarLabs/Cobalt/blob/1cc5b6412c4229b85e9960dd956c3d26c4ffd66e/crates/kobo-app-store/src/lib.rs)
* [Runtime catalog, install, launch verification and transactions](https://github.com/BandarLabs/Cobalt/blob/1cc5b6412c4229b85e9960dd956c3d26c4ffd66e/crates/kobod/src/app_store.rs)
* [CLI local bundle commands and release key check](https://github.com/BandarLabs/Cobalt/blob/1cc5b6412c4229b85e9960dd956c3d26c4ffd66e/crates/kobo-cli/src/main.rs)
* [Publish workflow and secret isolation](https://github.com/BandarLabs/Cobalt/blob/1cc5b6412c4229b85e9960dd956c3d26c4ffd66e/.github/workflows/apps.yml)
* [Bounded personal panel sessions and app allowlist](https://github.com/BandarLabs/Cobalt/blob/1cc5b6412c4229b85e9960dd956c3d26c4ffd66e/crates/kobo-cli/src/panel.rs)

## Future hardware test

The supported delivery route identified in the reviewed documentation requires
maintainer review, a Beta merge and protected signing/publication. The owner
must explicitly approve upstream submission, publication and the attended device
test. A future private route would need explicit maintainer documentation that
preserves the stock verification policy. See the
[submission packet](COBALT-SUBMISSION.md) and [hardware gates](HARDWARE-TEST.md).
