*[Versi Indonesia](CONTRIBUTING.id.md) · English version*

# Contributing to Figeometrica

One rule: **every claim must carry evidence.** Everything in this repo is
machine-checkable — including your claims. The gate runs on every pull request (PR) in
continuous integration (CI), and the local version is identical.

## Adding or fixing a figure = one JSON file

Open `data/figures/<name>.json`. The structure:

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

## Verify locally before pushing

```bash
cargo test --workspace
cargo run -q -p figeometrica-rhetorica --bin sidang -- --ci
cargo run -q -p figeometrica-rhetorica --bin validate
```

If all three are green, CI will be green too.

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
