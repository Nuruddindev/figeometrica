*[Versi Indonesia](CONTRIBUTING.id.md) · English version*

# Contributing to Figeometrica

One rule: **every claim must carry evidence.** Everything in this repo is
machine-checkable — including your claims. The gate runs on every PR in
CI, and the local version is identical.

## Two ways to contribute

You do **not** need to know programming or Git. Most of our contributors
are students and scholars of linguistics, rhetoric, and philosophy.

### Way A — entirely in your browser (recommended for your first figure)

1. **Create a free [GitHub](https://github.com) account**, then open
   [the repository](https://github.com/Nuruddindev/figeomatrica) and press
   the **Fork** button (top right). *Fork* = your own copy of the project,
   under your account. Nothing you do can break the original.
2. **Claim a figure** — open an issue with the
   ["Geometrize a figure"](../../issues/new?template=geometrize-figure.md)
   template so nobody double-claims, or just pick any file under
   `data/figures/` whose contract blocks are missing (e.g. `epizeuxis.json`).
3. **Edit the file in the browser**: navigate to
   `data/figures/<name>.json`, click the **pencil icon** (top right of the
   file view), and fill in the blocks described below. GitHub shows you a
   preview; JSON must stay valid (watch commas and quotes).
4. **Press "Commit changes"** and choose *"create a new branch … propose
   changes"*. Committing = saving a snapshot. It lives only in YOUR fork.
5. **GitHub now offers "Propose changes" / "Open pull request"** — click it,
   describe briefly what you did, submit. This request (*pull request*, PR)
   says: "here is my work, please consider it."
6. **A robot checks your work within ~2 minutes.** Green checkmark =
   your evidence held up; you wait for human review. Red X = open the
   failed check, read the exact reason, edit your file again and commit to
   the same branch — the check reruns automatically. Nobody scolds you;
   the machine simply refuses unevidenced claims, always with an
   explanation.
7. **A maintainer reviews** (= our Judgment Desk). Merge = ratification:
   your name enters the entry's `attribution`, permanently.

### Way B — on your own computer

For those comfortable with Git: fork → `git clone` your fork → branch →
edit → commit → push to your fork → open the PR. Nothing is ever sent
anywhere automatically; work stays local until you push, and even then it
only reaches your own fork — never this repository without a PR.

Local verification before pushing:

```bash
cargo test --workspace
cargo run -q -p figeometrica-rhetorica --bin sidang -- --ci
cargo run -q -p figeometrica-rhetorica --bin validate
```

If all three are green, CI will be green too.

---

## What to put in the figure file

```json
{
  "id": 91,
  "name": "apocope",
  "definition": "Cutting off final letter/syllable",
  "geometry":   { "...": "legacy geometry block (optional)" },
  "signature": {
    "domain_id": "textual",
    "unit_id": "word",
    "scope_id": null,
    "anchor_id": "final-segment",
    "operation": "detractio",
    "payload_id": null,
    "locus_id": null,
    "result": null,
    "constraints": {}
  },
  "epistemic": {
    "status": "WITNESS_TESTED"
  },
  "examples": {
    "positive": [["Photograph", "photo"]],
    "negative": [["The veterinarian examined the dog.",
                  "The vet examined the dog carefully and completely."]]
  }
}
```

### Rules for the `signature` block (CONTRACT.md §2, §12)

- Every slot must come from the canonical knowledge manifest:
  `data/knowledge/vN/manifest.json` — highest N wins.
- Slots that don't apply get `null`, never invented values.
- `scope` is not a junk drawer: if unsure, leave it `null`.

### Rules for the `epistemic` block (the ladder)

| Your claim | What CI checks |
|---|---|
| `EXTRACTED` | signature present & slots valid |
| `STRUCTURALLY_VALID` | + bindings not INVALID |
| `WITNESS_TESTED` | + witness protocol re-run passes |
| `USER_ACCEPTED` / `CANONICAL` | + maintainer merge |

Claiming a status without evidence fails CI with a message explaining
why. That's a feature, not a bug: *NO SILENT PROMOTION*.

### Prose definitions

Write definitions in your own words. Text copied from copyrighted
sources will not be accepted.

### Language policy (CONTRACT §12.3)

- `definition` and all `note` fields: **English only** — the theory layer
  is monoglot so it can be judged anywhere.
- Indonesian glosses are welcome as parallel fields with the `_id` suffix
  (e.g. `definition_id`) — they never replace the canonical English field.
- Example text: **any language welcome** — figures are
  language-independent patterns, so an Indonesian or Arabic instance is
  evidence for the thesis, not noise. Entries with non-English examples
  declare them: `"example_languages": ["en", "id"]`.
- Never mix languages inside one example sequence — each sequence is one
  discourse.

## Missing a vocabulary slot? Propose a knowledge version

Never force an unrelated slot into place. Instead:

1. Copy the current highest version: `cp -r data/knowledge/v2 data/knowledge/v3`
2. Add your slot/binding in `v3/manifest.json`
3. Write `v3/README.md`: which slot, which figure demands it, and the
   experiment/example that revealed the need
4. Reference that folder in your PR

Old versions are never edited — they are the experiment log. Details in
[`data/knowledge/README.md`](data/knowledge/README.md).

## Review is our Judgment Desk

PRs that pass CI are reviewed by the maintainer. Merge is the act of
ratification: that's where a figure rises to `USER_ACCEPTED`, and only
this path reaches `CANONICAL`. The review history lives forever in the
PR thread — that thread *is* our ledger.

## License, attribution & co-authorship

- Contributions are licensed **MIT** from the moment the PR opens
  (inbound = outbound).
- Your name goes into the entry's `attribution` field + CONTRIBUTORS.md.
- Contributors with **≥ 10 accepted entries** or serving as validators
  join the dataset paper's public co-author list. Final criteria are
  announced before the paper is written and are not retroactive.
