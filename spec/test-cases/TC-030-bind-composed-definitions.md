---
id: TC-030
title: "Bind every clause to its exact semantic definitions"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-130
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-030
    type: verifies
---
## Description

Accepted FS05 portable controls for FR-130 and the reference/evolution portion
of [FR-030](../functional/foundation/FR-030-bind-composed-definitions.md). No executable test or completed coverage is claimed.

## Test Procedure

Use one three-family package with two inventoried source units, one exact model
closure and two distinct clauses declaring the same local clock-role name.
Establish its expected selections from the source artifacts and selected definitions.
Apply each control independently:

| Control | Expected observation |
| --- | --- |
| Select a definition identity other than the selected one; duplicate the editable clause authority | Typed dependent admission refusal with the failed selection and original source region. |
| Replace only the runtime trace | Equal static subject; different retained runtime input. |
| Change a selected semantic definition with matching new bytes | Distinct static subject, admitted only if the new closure is supported and valid. |
| Omit an inventoried source unit | Namespace cannot close; no guessed binding to a remaining declaration. |
| Put a shadowing source file beside the inventoried units without adding it to the inventory | No change to the declared package or name resolution. |
| Put the lock digest, a package fingerprint or a raw-byte digest in the domain package's lock-row digest position | Refused import; no domain substitution. |
| Inspect both equal-named clock roles | Distinct owning clauses/source regions; exact types, anchors and scope dependencies retained. |
| Change only the compile resource ceiling, with both ceilings sufficient for admission | Static subject unchanged; the exact changed configuration remains in build provenance. |
| Link a valid three-family template without concrete workflow instances, runtime populations/windows, observation records or progress | Static linking succeeds and retains the exact typed requirements for later assessment. Removing a required static definition still refuses dependent linking. |
| Import the domain package lifted from the package's `entity` artifacts by identity alone and read the derived `sha256-jcs` digest from the lock row; resolve each clause's qualified domain type name | Each name resolves to one effective declaration keyed by domain package identity, IR node identity and `sha256-jcs`; declared kinds, relationships and closure match independently authored expectations. |
| Omit the imported domain package; name a type the domain package does not declare; substitute a declaration key from another domain package; put a `quire.source.bytes/v1` digest in the domain package's lock-row position and the domain package digest in a source's lock position | Each dependent binding refuses with a structured cause even when labels or field shapes match. |
| Re-serialize the domain package with a different key order but the same RFC 8785 canonical form; then change one declared field in its source artifact | The first keeps the `sha256-jcs` identity and static subject; the second changes the domain package identity and the static subject. |

Compare complete structured static components and references. Do not use an
invented package hash, a tool-generated expected result or diagnostic English
as the oracle. Concrete typed extraction, intake and binder execution is specified by IT-010.

For the [whole-package reference](../../proposals/quire-v1/definitions/package-reference.md),
use a package declaring a predicate and all three clause families. Its outer
selection is the package interpretation and its four exact required features;
the inner state/temporal/protocol selections remain separate. Substitute one
clause's profile for the outer selection, remove an inner selection, omit or add
an outer feature, and attach an unrelated IR/JCS canonical digest. Each mutant
refuses interpretation. Reordering the unique exact feature set preserves it.
Changing a valid inner temporal selection changes the static subject even though
the outer definition remains the same. A partial package cannot claim full
linking from one valid child reference. These cases require the future actual
payload reader; existing envelope shape tests do not discharge them.

Select the outer package interpretation as a native clause's `using` profile and
require a profile-role refusal. Supply a valid envelope naming an unknown package
payload wire and require unsupported-wire refusal before semantic admission.
The outer feature set cannot count as evidence for implemented family support.

A predicate-only package has `declaration.predicate` without `family.state`;
a state-only package has `family.state` without `family.temporal`, and so
does a package whose only state clause is a state constraint. Add the known
but absent temporal feature to that state-only package: semantic feature-set
validation must reject it even though it is not an unknown-feature spelling.

## Expected Results

The valid package retains exact bindings and separate roles. Each invalid
selection refuses its dependent admission with a typed cause and location.
Runtime/provenance changes preserve static identity where specified; semantic
changes do not. Refused dependencies never execute or claim complete admission.
