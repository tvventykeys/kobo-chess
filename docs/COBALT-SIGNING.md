# Cobalt signing investigation — 17 November 2026

**Current blocker:** this standalone app has no supported personal install path
in the inspected stock Cobalt revision. A development signature is valid only
under its own test key; it grants no installation trust. No device-ready package
is claimed or supplied. No Store publication or maintainer contact was performed.

The pinned SDK and public upstream `main` both resolve to
`5ac30a46efce23d7fa0d1cb12a499c9b57b816b0` (0.3.37), verified with remote refs.
The following evidence is from actual source, not an assumption about signing.
All links pin that exact revision.

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

* [Contributor route, licensing and automated Beta/Stable signing](https://github.com/BandarLabs/Cobalt/blob/5ac30a46efce23d7fa0d1cb12a499c9b57b816b0/docs/CONTRIBUTING_APPS.md)
* [Bundle format and signature verification](https://github.com/BandarLabs/Cobalt/blob/5ac30a46efce23d7fa0d1cb12a499c9b57b816b0/crates/kobo-app-store/src/bundle.rs)
* [Compiled-in release key](https://github.com/BandarLabs/Cobalt/blob/5ac30a46efce23d7fa0d1cb12a499c9b57b816b0/crates/kobo-app-store/src/lib.rs)
* [Runtime catalog, install, launch verification and transactions](https://github.com/BandarLabs/Cobalt/blob/5ac30a46efce23d7fa0d1cb12a499c9b57b816b0/crates/kobod/src/app_store.rs)
* [CLI local bundle commands and release key check](https://github.com/BandarLabs/Cobalt/blob/5ac30a46efce23d7fa0d1cb12a499c9b57b816b0/crates/kobo-cli/src/main.rs)
* [Publish workflow and secret isolation](https://github.com/BandarLabs/Cobalt/blob/5ac30a46efce23d7fa0d1cb12a499c9b57b816b0/.github/workflows/apps.yml)
* [Bounded personal panel sessions and app allowlist](https://github.com/BandarLabs/Cobalt/blob/5ac30a46efce23d7fa0d1cb12a499c9b57b816b0/crates/kobo-cli/src/panel.rs)

## Prepared without release credentials

The independent app, manifest, tests and ARM binary are ready for review. Running
`scripts/package-pocket.sh` produces a **verification-only** `.cobaltapp`, with
its own local fixture key and exact binary digest; this is not a release package.
The packaging evidence includes a negative test: the fixture must be refused
when verified under Cobalt's actual release public key. No trust bypass is used.

`scripts/prepare-cobalt-contribution.py OUTPUT` creates a review tree outside both
repositories, adapting only the package manifest to Cobalt's documented workspace
conventions. The tree contains public code/puzzles and licenses, with no private
book fixtures or signing seed. Its README lists remaining workspace registration,
generated registry/pages, contributor check, review and release steps. It has not
been compiled as an upstream member, submitted, signed or published.

Questions prepared for a future owner-authorized maintainer discussion:

1. Is there a supported, owner-attended **non-Store** third-party test route for
   an independent app at this revision? What exact commands and trust policy
   apply without replacing installed apps or changing the release trust root?
2. If not, is maintainer-approved Beta publication the only supported route?
   The owner must approve that publication separately before any upstream PR.
3. Which runtime/firmware and reader handoff are supported alongside KOReader,
   NickelMenu, Zen OS and Focus Reader? Zen OS coexistence is not documented in
   the inspected support matrix, so the ordinary Nickel return path is unproven.

No signing credential, unreviewed key enrollment, private catalog or pretend
installable release is needed to continue host development. The first physical
test remains gated on those external answers and owner review of the guide.
