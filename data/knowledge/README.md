# Knowledge Vocabulary — Versions & Manifests

The contract vocabulary (domains, units, scopes, anchors, payloads, loci,
bindings) evolves every time an experiment discovers a phenomenon that
doesn't fit the old slots. So that this evolution stays **recorded and
auditable**, each state of the vocabulary is preserved as one versioned
folder:

```
knowledge/
  v1/manifest.json   ← initial state (SARVA snapshot after Phases 1–5)
  v1/README.md          why v1 exists, what it contains
  v2/...                ← born when a new discovery demands it
```

## Versioning rules

1. **Highest version = canonical.** The `sidang` bin always validates
   against the highest version number. There is no mutable `LATEST`
   pointer — git history is enough.
2. **Old versions are immutable.** Once released, a version folder is
   never edited. New discoveries mean a new version, never a silent
   correction. This is *NO SILENT PROMOTION* applied at the vocabulary
   level.
3. **Version bumps require a story.** `vN+1/README.md` must explain which
   slots are new, which figures demanded them, and which experiment
   revealed the need. A slot without a story doesn't get in.
4. **Slots carry status.** `known` = tested across several figures;
   `candidate` = used by one or two, awaiting confirmation. CI only
   requires that `candidate` slots carry a justification in the manifest.

## How to bump a version

```bash
cp -r data/knowledge/v1 data/knowledge/v2
# edit v2/manifest.json: add/modify slots + status
# write v2/README.md: the story of the discovery
git commit -m "knowledge v2: <new slot> for <figure/phenomenon>"
```
