# Knowledge v1 — initial state

**Created**: 2026-08-23 · **Source**: snapshot of the SARVA vault's
knowledge tables after the CONTRACT.md v1 implementation (Phases 1–5).

## Contents

| Vocabulary | Count | Notes |
|---|---|---|
| domains | 4 | textual, conceptual, entity, argumentative |
| units | 12 | grapheme…concept (includes entity units for personification figures) |
| scopes | 5 | phonological-form, orthographic-form, token-stream, representation, discourse |
| anchors | 10 | segment positions (initial/final/medial), insertion-point, whole-unit, cross-boundary, + entity anchors (non-person, person, non-human, character) |
| payloads | 7 | segment, letter, syllable, person, human-attribute, characterological-attribute, preemptive-response |
| loci | 9 | initial, medial, terminal, response, distributed, clustered, every, cross_unit, alternating |
| bindings | 9 | all status `valid`; other combinations = UNKNOWN (legal but untested) |

## Why this state became v1

This is the point where geometrization moved from SARVA's private vault
to figeometrica's public ledger. The numbers above are not theoretical
design — each one was born from real figures among the 455 definitions
that were extracted, migrated, and put on trial:

- entity anchors (`non-person`, etc.) — demanded by prosopopoeia;
- `preemptive-response` — demanded by procatalepsis;
- locus `alternating` — demanded by abecedarian;
- 9 valid bindings = exactly the combinations used by signature-bearing
  figures.

## What v1 does NOT contain (by design)

There are no just-in-case slots. Every slot here has a figure that
ordered it. Tempting empty vocabulary (e.g. a generic scope like `other`)
was deliberately rejected per the *scope is not a junk drawer* principle
(CONTRACT §4).
