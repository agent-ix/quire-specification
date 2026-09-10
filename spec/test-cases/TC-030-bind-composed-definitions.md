---
id: TC-030
title: "Bind every clause to its exact semantic definitions"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-030
    type: verifies
---
## Description

Planned acceptance controls for [FR-030](../functional/FR-030-bind-composed-definitions.md).
No executable test or completed coverage is claimed by this artifact.

## Test Procedure

Use one three-family package with two inventoried source units, one exact model
closure and two distinct clauses declaring the same local clock-role name.
Establish its expected selections from the source and producer-owned definitions.
Apply each control independently:

| Control | Expected observation |
| --- | --- |
| Change a definition revision or digest; duplicate the editable clause authority | Typed dependent admission refusal with the failed selection and original source region. |
| Replace only the runtime trace | Equal static subject; different retained runtime input. |
| Change a selected semantic definition with matching new bytes | Distinct static subject, admitted only if the new closure is supported and valid. |
| Omit an inventoried source unit | Namespace cannot close; no guessed binding to a remaining declaration. |
| Put a shadowing source file beside the inventoried units without adding it to the inventory | No change to the declared package or name resolution. |
| Put the manifest digest, lock digest or producer fingerprint in the compiled-model import digest position | Refused import; no domain substitution. |
| Inspect both equal-named clock roles | Distinct owning clauses/source regions; exact types, anchors and scope dependencies retained. |
| Change only the compile resource ceiling, with both ceilings sufficient for admission | Static subject unchanged; the exact changed configuration remains in build provenance. |

Compare complete structured static components and references. Do not use an
invented package hash, a producer-generated expected result or diagnostic English
as the oracle. Concrete typed producer/consumer execution is specified by IT-010.

## Expected Results

The valid package retains exact bindings and separate roles. Each invalid
selection refuses its dependent admission with a typed cause and location.
Runtime/provenance changes preserve static identity where specified; semantic
changes do not. Refused dependencies never execute or claim complete admission.
