# The golden render record (`sh2golden`)

The behaviour gates compare what a transpiled program *prints*, so a lowering
can change every render it touches and still pass — and "which `sh2perl` made
this render?" has no answer once the worktree has moved on. This is the record
that answers both: the rendered text of the corpus, per backend, stamped with
the exact renderer identity that produced it.

It is **not needed to compile or run anything**. No gate, build or CI job
depends on it: a gate must never need a second clone.

## Shape: a standalone repo, not a submodule

- A submodule exists so a consumer can check out a pinned commit and *build
  against it*. Nothing builds against rendered text; the workflow is
  regenerate → `git diff` → review → commit. A gitlink would only add a
  pointer bump per render — and this superproject already has three
  `.gitmodules`-less gitlinks (`s2p.c`, `s2p.rust`, `s2p_go`) that make
  `git submodule status` fail outright.
- `sh2perl` must never reference this record (PLAN §1.1), and `examples/`
  lives inside the `sh2perl` submodule, so nesting is ruled out as well.
- Where it lives: **`github.com/gmatht/sh2examples`** (published 2026-11-13),
  cloned as a sibling; `$SH2_GOLDEN_REPO` points at it, defaulting to the
  ignored `.golden-inbox/` in the workspace for ad-hoc runs.
- **The sets themselves are tracked files.** A round adds
  `sets/<utc-stamp>-<sha7>-<backend>/` and its `INDEX.tsv` row in one commit,
  so the exported text is browsable, diffable and cloneable next to the index
  that names it. This replaced the release-asset design
  (`sets-<round>.tar.gz`): nothing consumed the tarballs, they were opaque to
  `git diff`, and every round that could not upload left an `INDEX.tsv` row
  naming a set nobody could retrieve.
- **What that costs, measured.** A full-corpus rust set is 3.4 MB across 415
  files (~8 KB average, largest single render 17 KB); a full 16-backend round
  is ~5–10 MB of working tree. Git is content-addressed, so an unchanged
  render costs nothing however many round directories name it — a quiet round
  adds only the blobs that actually changed. What grows with every round kept
  is the **checkout**, so prune deliberately (`git rm -r sets/<old>` in a
  normal commit; history keeps them) instead of letting the nightly job grow
  the tree forever. The per-file 1 MiB cap in `golden-render.sh` (larger
  renders become `oversize`) is what keeps any single blob far from GitHub's
  100 MB limit — the failure `docs/CLEAN_GIT.md` records at 4.7 GB → 19 MB.
- If a comparison later needs a fixed point, pin it by commit SHA in CI — the
  `sh2runtime` pattern (AGENTS.md:14-15, PLAN §1.2).

## What a set is

One directory per render round: `sets/<utc-stamp>-<sh2perl-sha7>-<backend>/`

| file | meaning |
|---|---|
| `render.json` | the stamp (below) |
| `MANIFEST.tsv` | `name <TAB> sha256 <TAB> bytes <TAB> status` |
| `renders/<example>.<ext>` | status `rendered` |
| `renders/<example>.<ext>.refused` | status `refused`; the first diagnostic line |
| `renders/<example>.<ext>.error` | status `error`; the first stderr line |
| `patches/tracked.patch` | only when the renderer's worktree had tracked changes |
| `determinism.diff` | only when two renders of the same input disagreed |
| `INDEX.tsv` (record root) | append-once row per set: backend, sh2perl sha, corpus sha, counts — answers "which version" without opening a set |

`render.json` records, because the version string cannot (`CARGO_PKG_VERSION`
is a hard-coded `0.1.0` in both crates):

- `renderer` — `kind` (`shir_render` or `cli`), `path`, `sha256` of the
  binary, its mtime;
- the workspace's `sha`/`branch`/`dirty`, and the same for the backend
  worktree (`sh2perl_worktree_*`), plus `ahead`/`behind` against `main` —
  worktrees are routinely hundreds of commits behind, and each carries its own
  `examples/` snapshot;
- `corpus` — directory, the sha256 of every input actually read (so a render
  made from an edited example is detected), and the `HEAD:examples` tree hash;
- `counts` and `unstable`.

A worktree with tracked changes is the normal case mid-fix: the patch beside
the stamp makes the render reproducible from `sh2perl@<sha> + tracked.patch`
after the worktree moves on. (`dirty` counts tracked changes only — untracked
scratch, pid files and caches do not change what the compiler emits, and
counting them would mark every worktree dirty forever.)

## Tools

```
harness/golden-render.sh --backend rust              # one set
harness/golden-render.sh --all --jobs 8              # every backend with a renderer
harness/golden-render.sh --backend c --limit 20      # smoke run
harness/golden-diff.sh <setA> <setB>                 # summary (rc 1 if any differ)
harness/golden-diff.sh <setA> <setB> --list|--stat   # per-example detail
harness/golden-diff.sh <setA> --grep 'sh2\.bigint'   # which examples contain it
harness/golden-diff.sh --latest rust                 # the newest set for a backend
```

`golden-diff.sh` refuses to pretend: it prints both worktrees' distance from
`main` and warns when the two sides are not like-for-like or were rendered
from different `sh2perl` commits, and it flags an `unstable` set.

## Renderer choice

Per backend, `backends/<lang>/target/debug/shir_render --target=<lang>` when
that bin exists (what the gates use), else `otranspilerl-cli <example>
--target <lang>`; the stamp records which, because the two are not guaranteed
to agree and several worktrees still lack `shir_render`.

## Determinism and hygiene

- Every render is done twice; non-identical outputs are a transpiler bug (an
  embedded tmp path, a timestamp) and mark the set `unstable`.
- Re-rendering with an unchanged binary must leave the record clean. That
  clean tree is the acceptance test.
- Per-file cap 1 MiB (larger renders become `oversize`, not committed) and
  `golden-render.sh` never stages: text only, one commit per backend per
  round, and the record gets its own pre-commit guard.

## Publishing a round (gmatht/sh2examples)

```
harness/golden-render.sh --backend rust --jobs 8     # writes a set under the record root
cd $SH2_GOLDEN_REPO                                  # the clone of gmatht/sh2examples
git add -A sets INDEX.tsv
git commit -m "sets: $(date -u +%F) round"
git push origin main
```

`INDEX.tsv` is the correlation table (backend, sh2perl sha, corpus sha,
counts); `sets/` is the corpus itself. A set directory and its `INDEX.tsv` row
land in the **same** commit — a row without its files is a broken reference,
which is what the old release-asset design produced whenever the upload failed.

## The nightly round (cron, 03:14)

`harness/golden-nightly.sh` is installed as

```
14 3 * * * HOME=/home/john /home/john/src/sh2loop/harness/golden-nightly.sh >>/home/john/src/sh2loop/loop-golden.log 2>&1
```

(install idempotently with `harness/golden-nightly.sh --install-cron`; every
path in the line is derived from the script itself, so it is machine state,
not in git — re-run it if the tree moves, the same rule as
`reap-orphan-examples.sh`.)

The round needs nothing but `git`: it **commits the sets and `INDEX.tsv` and
pushes**, which is the whole publication. There is no tarball and no release
asset, so no `gh` and no second upload that can fail after the index landed.

It takes `.golden-nightly.lock` (no two rounds, and no round racing a
hand-run `golden-render.sh`), fast-forwards its clone of `gmatht/sh2examples`
(refusing to publish over local commits someone made by hand), renders every
backend `--jobs 2` (this box wedges on IO when several compiles overlap), and
commits the round's set directories with their `INDEX.tsv` row.

Trial runs: `harness/golden-nightly.sh --dry-run --backends fish --limit 3`
(renders, publishes nothing, and restores the record clone — nothing staged
and no set left behind, so a trial cannot feed a row for an unpublished set
into the next real round).
