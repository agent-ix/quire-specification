---
id: SR-13654
title: 'Code review of QSpec PR #315: universe provision vectors and test (STD-196)'
type: SpecReview
analysis: code-review
scope: agent-ix/quire-specification@d870b40de; tests/backend_provider_v1.rs tc_290_universe_provisions_are_canonical_arrays_of_typed_keys; proposals/backend-provider-v1/vectors.json universe_provision_vectors; ticket STD-196
review_set: subset
date: '2026-10-21'
relationships:
- target: ix://agent-ix/quire-specification/FR-361
  type: reviews
---

## Summary

Ticket: STD-196. PR quire-specification#315, head d870b40de15daccf948556127fa2c1717f19030c.

Mutations of `universe_provision_vectors`: adding `object_type` to a key, changing a key's `type` to `reference`, corrupting a digest, and making the two vectors identical each fail the test (good). Deleting `universe` from both keys of `counters-c-d` and recomputing `jcs` and `digest` passes the test.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| tc_290_universe_provisions_are_canonical_arrays_of_typed_keys | tests/backend_provider_v1.rs | examined |
| universe_provision_vectors | proposals/backend-provider-v1/vectors.json | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The test checks that each vector's `jcs` and `digest` match its own `keys`, that `type` is `key` and that no `object_type` is present. It does not check the key shape FR-361 defines: exactly `type`, `universe` (64 lowercase hex) and `identity` (lowercase hex). A vector whose keys lack `universe`, with `jcs`/`digest` recomputed, passes. Fix: assert the member set and the two hex patterns for every key (or validate keys against a small published key schema). | tests/backend_provider_v1.rs:763-786 |
| FND-002 | low | vectors.json ends without a newline after this change, and tests/backend_provider_v1.rs ends with an extra blank line. Fix: restore the newline and drop the blank line. | proposals/backend-provider-v1/vectors.json:3493 |

## Verdict

FND-001 should be fixed with the provision encoding; FND-002 is cosmetic.
