---
id: TC-270
title: "SMT/native runtime differential parity corpus"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-355
    type: verifies
---

# TC-270: SMT/native runtime differential parity corpus

## Description

Evaluate the shared differential corpus through the registered SMT backend
and through native `runtime::execute`, and compare verdicts member by member.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-355. Execute its
positive case, each declared boundary, and each explicit refusal/non-conclusive
mutation through the real local Rust boundary; compare the complete typed
result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-355; every corpus member is
evaluated by both the SMT backend and the native runtime under the same
package, domain and claim identity; matching verdicts are recorded as
agreement; a decode, domain or verdict mismatch is recorded as an explicit
parity failure naming the member and both verdicts, never silently resolved
toward either side; the aggregate parity report accounts for every corpus
member exactly once; each mutation returns its exact typed disposition with
source/provenance and preserves independent sibling results.
