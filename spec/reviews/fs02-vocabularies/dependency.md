---
id: SR-524
title: "Dependency review of FS02 vocabulary closure"
type: SpecReview
analysis: dependency
scope: "quire-specification #29 authority and consumer dependency graph"
review_set: subset
review_date: "2026-09-13"
---
# Dependency review of FS02 vocabulary closure

## Summary

PASS. The authority graph is acyclic and separates this vocabulary enablement
from later consumer implementation; the unresolved role-instance key is exposed
as an explicit construction refusal owned by #28 rather than guessed here.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No dependency cycle, hidden implementation claim or TL-lane reassignment remains in the scoped packet. | FR-212; FR-285; FR-286; FR-061; FR-062 |

## Classification and graph

| Requirement | Class | Direct consumers or effect |
| --- | --- | --- |
| FR-201, FR-212 | Existing enablement | Digest-domain authority and role-instance identity authority. |
| FR-242, FR-243, FR-271, FR-273, FR-283, FR-285, FR-286 | Enablement | Closed values, shapes, namespaces and identity preimages consumed by result/diagnostic behavior. |
| FR-047, FR-061, FR-115 | Feature contracts | Produce or retain values under the selected authorities. |
| FR-062 | Feature adapter | Consumes a strict FR-061 source and derives the E02 target without loss. |
| FR-249 | Derived object | Records the FR-062 projection identity and refusal boundary. |

The logical order is:

1. FR-201 supplies the digest domain and FR-212 supplies role-instance keys.
2. FR-273/FR-271 type producer diagnostics; FR-242/FR-243/FR-285/FR-286 type
   canonical result values; FR-283 types projection refusals.
3. FR-047 consumes diagnostic authorities; FR-061 and FR-115 consume result
   authorities.
4. FR-062/FR-249 consume a strict accepted source and target context.

No `depends_on` cycle is introduced. FR-285 intentionally cannot construct or
reorder role-instance collections until FR-212's key/order is accepted by #28;
it retains supplied facts and refuses display-derived ordering. That is the
ticket's allowed explicit deferred disposition and makes #28 visible rather
than making #29 depend on an invented key.

The change is confined to quire-specification. It consumes no TL implementation,
changes no TL-* file, and leaves Agent B's entire TL lane untouched. Protocol
and projection implementation remain downstream work; planned matrices do not
claim those consumers exist.
