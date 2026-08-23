# The Figeometrica Manifesto

*[Versi Indonesia](MANIFESTO.id.md) · English version*

*Figures are geometric — and once you see it, text analysis changes shape.*

---

## 1 · The problem

Twenty-four centuries ago, Aristotle systematized persuasion. Roman
rhetoricians catalogued the ornaments of speech; Renaissance schoolbooks
drilled schoolboys on hundreds of them. We inherited roughly 456 named
figures of speech — **the oldest taxonomy of text analysis in existence**,
refined continuously from antiquity through the Renaissance.

And every single one of them is defined in prose.

Prose cannot be executed. Ask a room full of NLP researchers how many texts
in a corpus contain anaphora, and nobody can answer without reading every
text. Ask which passages escalate toward a climax, and you get opinions.
Modern computational methods skipped this layer entirely: Rhetorical
Structure Theory consciously discarded surface form in favor of semantic
relations; stylometry reduced style to function-word statistics; large
language models can *imitate* a style but cannot *audit* one — ask why a
passage feels rhythmic and you receive vibes.

The result: humanity's oldest and most refined theory of how texts are
shaped sits unused by machines.

---

## 2 · The insight

Read the definitions closely and they leak algorithms.

Anaphora: *"repetition of the same word at the beginning of successive
clauses."* That is not prose wearing a definition's clothes — that is an
operation: insert the same token at the **initial anchor** of consecutive
units, repeated at least twice. Antimetabole: invert a phrase — permutation.
Tmesis: cut a word open and insert another inside it — addition at the
grapheme grain. Chiasmus: reverse two conceptual roles across a turn —
permutation over meanings instead of words.

The Romans already knew this. Their four *operae* — **adjectio** (addition),
**detractio** (deletion), **immutatio** (substitution), **transmutatio**
(permutation) — plus repetition are the complete operator set. Every figure
in the catalog is a parameterization of these operations: an anchor point, a
grain, a repeat count, sometimes a slot template.

So we state the thesis plainly:

> A figure definition is an uncompiled algorithm. And a definition that
> *cannot* be written as such an operation is not a "non-geometric figure" —
> it is a badly written definition.

This makes the entire 456-figure catalog falsifiable, for the first time in
its long history.

---

## 3 · The move

Compile them.

Every definition is rewritten into canonical form. The classical thesis —

> figure = OPERATION × ANCHOR × GRAIN × REPETITION

— compiles, under [CONTRACT.md](CONTRACT.md), into an executable signature:

```
FigureSignature = domain    ∈ {textual | conceptual | entity | argumentative}
                × unit      ∈ {grapheme … concept}
                × scope     ∈ {phonological-form … discourse}
                × anchor    ∈ {initial/final/medial-segment, insertion-point,
                               whole-unit, cross-boundary,
                               person | non-person | non-human | character}
                × operation ∈ {adjectio | detractio | immutatio |
                               transmutatio | repetitio}   ← frozen term set
                × payload   ∈ {segment, letter, syllable, person, …}
                × locus     ∈ {initial, medial, terminal, response,
                               distributed, clustered, every,
                               cross_unit, alternating}
                [+ result, + constraints]
```

`tmesis` ("abso-bloody-lutely") becomes:

```json
{
  "domain_id": "textual",
  "unit_id": "word",
  "anchor_id": "insertion-point",
  "operation": "adjectio"
}
```

The slot vocabularies are not hardcoded anywhere — they live in a versioned
knowledge manifest ([`data/knowledge/vN`](data/knowledge/README.md)). The
highest version is canonical; old versions stay immutable as the experiment
log. When a discovery doesn't fit the old slots, you propose `vN+1` with the
story of what demanded it — *no silent promotion*, even for vocabulary.

Once compiled, everything changes:

- **Detection is deterministic.** The matcher never calls a model. Given a
  text, it either finds the pattern or does not — with byte-exact evidence
  spans.
- **The catalog becomes queryable.** "Which figures close a discourse?" →
  filter by final anchor. "What can escalate?" → gradatio. Before analyzing
  any document.
- **Contributions are machine-checked.** Every entry ships with positive and
  negative example sentences; CI runs the matcher against them. A
  contributor cannot submit a spec that contradicts their own examples.
- **Negative evidence becomes real.** "No chiasmus in this paragraph" stops
  being an impression and becomes a checkable claim.

---

## 4 · Why it matters

**For NLP and computational humanities:** this is the missing bridge between
classical stylistics and computation. Retrieval by rhetorical function —
"find texts that build momentum," "find passages that concede before
refuting" — instead of retrieval by keywords. Style analysis with provenance
instead of vibes.

**For writers and teachers:** figures stop being trivia to memorize and
become moves to see, name, and practice. A student's speech can be checked:
does it open with parallel structure? Does it escalate? Where does it close?
Style becomes teachable because it becomes visible.

**For AI systems:** hybrid pipelines where geometry is the deterministic
evidence layer and language models do what they are good at — interpretation
— on top of evidence they cannot fake. Every finding auditable down to its
byte offsets.

**For the humanities at large:** a demonstration that theories become
cumulative and testable when compiled. Not by reducing them to numbers, but
by taking their structural claims seriously enough to execute them.

---

## 5 · The bigger frame

Rhetoric is the pilot, not the boundary.

The framework — theory base as versioned data, canonical compilation format,
deterministic engines, machine-validated crowd contributions — applies to
any humanistic theory whose claims have structure. Fallacies come next:
Aristotle's *apparent enthymemes*, arguments that look valid and are not,
waiting for the same treatment. Then prosody, argument schemes, narrative
moves.

The machine does not replace the rhetorician. It gives their oldest
observations executable bodies — so that what was discovered by hand over
twenty-four centuries can finally be verified at scale.

---

*447 figures await. Pick one, compile it, let the machine check your work.*
