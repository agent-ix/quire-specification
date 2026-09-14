---
id: TC-002
title: "Refuse role and reference identity substitutions"
type: TC
relationships:
  - target: "ix://agent-ix/quire-specification/FR-017"
    type: verifies
---
# TC-002: Refuse role and reference identity substitutions

## Description

Accepted portable contract and negative controls for FR-017-AC-2–5. The
negative controls isolate structural validity from intended-role matching.
TM-001 remains Planned; this artifact does not claim executed portfolio coverage.

## Test Procedure

Start with TC-001's six valid references. For every requested role, substitute
generated-artifact plus each of the other five new kinds, leaving every other
field unchanged. Then mutate unknown kind, extra member, missing member,
envelope version and artifact version individually. For exact property, subject
(a source ArtifactRef), binding, dependency-closure and run-artifact references,
mutate authority, identity, revision namespace/value, digest, wire identity/version
one at a time. Check the original adopted schema/fixtures against e897f81 and
submit each added role under the previous closed schema.

## Expected Results

Valid nearby kinds still refuse intended-role matching with identity-mismatch.
Malformed shapes refuse invalid-wire and unsupported selections refuse their
version/kind interpretation. Every exact-reference mutation fails equality;
changing a digest without changing bytes additionally fails byte verification.
The original adopted packet is unchanged and its schema rejects the added kinds.
No failed selection returns a partially accepted reference.
