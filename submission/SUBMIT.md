# Owner-reviewed upstream submission steps

This packet is **prepared, not submitted**. No fork, upstream PR, release,
Store publication, device install, signing-key change or automatic merge is
performed. The application version is 0.2.0; the monorepo Cargo crate inherits
Cobalt's workspace version, as its contributor contract requires.

1. Review NAMING-LICENSE-PRIVACY.md and the actual screenshots/CAPTURES.json.
   Decide whether to retain the requested name or authorize the recommended
   **Pocket Chess Studies** alternative. No name clearance is asserted. A display
   rename can keep the internal `polgar-pocket` app ID/progress namespace.
2. Review the complete patch `cobalt-beta.patch` and its pinned upstream base in
   upstream.json. It adds one app, workspace/lock registration, an app-specific
   root attribution notice, the active protocol lockfile review hash (only the
   new workspace package, no existing dependency change), website screenshot/category entries, authentic
   assets and the official generator's pages/index/sitemap/README outputs.
   Existing Lichess source, signing workflow/key, catalog and other apps are
   unchanged. The test preview is signed only with upstream's public fixture key.
3. Recheck `BandarLabs/Cobalt`'s current `beta` ref and contributor guide. This
   packet pins the reviewed commit; it cannot promise that future Beta has not
   changed. If it moved, rebase the local contribution and rerun official checks,
   including registry/page freshness and current published Beta provenance.
4. A complete local branch is prepared outside the pinned SDK at
   `/workspace/work/cobalt-beta-submission`, `feat/polgar-pocket-beta`. It has no
   enabled push destination. Apply the patch to a separate clean clone of its
   pinned base, or use `scripts/prepare-cobalt-beta.py CLEAN_CHECKOUT` from the
   application repository, then run the documented contributor command. Neither
   preparation method creates a remote fork or contacts maintainers.
5. **Approval gate:** obtain explicit permission before creating a fork, pushing
   this branch outside the owner's app repository or opening the upstream PR.
   First check whether an appropriate owner fork already exists. No fork is
   necessary for local preparation and none was created in this task.
6. After that separate approval, push the reviewed branch to the approved fork
   and open the application PR against **BandarLabs/Cobalt:beta**, using
   UPSTREAM-PR.md and its real final contributor-check output. The template
   covers purpose, code/content licensing, zero capabilities, no accounts,
   Clara BW simulator captures and test results. Do not target upstream main.
7. Maintainers must accept purpose, naming, licensing, setup/capabilities and
   demo evidence, then merge through their protected review/CI process. That
   merge triggers **Beta Store publication**. Approval to open a PR is not a
   claim that publication or physical installation has already happened.
8. Protected automation rebuilds ARM binaries, signs each package and the channel
   catalogs with its actual release key, uploads immutable assets and finally
   moves the Beta catalog pointer. Contributors receive no release seed. Our
   deterministic fixture package/catalog is not stock-runtime installable.
9. Stable promotion is a separate maintainer action, not part of this packet.
   A documented non-Store personal key-enrollment/sideload path was not found in
   this revision. Do not replace trust roots, built-in binaries or catalogs.
10. Only after authenticated official availability and a further owner-approved
    hardware step should the reader installation guide be used. Zen OS/reader
    handoff, real colour, touch, page keys, suspend/resume and battery still need
    physical acceptance. Preserve KOReader, NickelMenu, Zen OS and Focus Reader.

No step here authorizes a device command. A successful local build, public-key
fixture verification or generated website install button does not mean the app
has been published, accepted or installed.
