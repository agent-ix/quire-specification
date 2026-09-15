---
id: FR-153
title: "Query explicit closed model environments"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-043
    type: references
  - target: ix://agent-ix/quire-specification/AD-006
    type: references
---

# FR-153: Query explicit closed model environments

## Description

When evaluating lookup or all-instances, the evaluator SHALL query only the
explicit typed environment and population closure bound to the request.

## Inputs

Qualified model/type identity, environment snapshot, membership/closure
authority and query bounds.

## Outputs

A typed value or bounded collection, unavailable/incomplete result, or refusal.

## Behavior

Lookup uses a qualified key and declares absent-key behavior. `allInstances`
returns the complete selected population only when closure authority is present;
otherwise it is incomplete. Neither operation consults a process-global registry
or invents objects from samples.

## Environment key and closure

An environment key is the tuple `(producer_namespace, producer_revision,
model_identity, declaration_identity, value_slot)`. Absence behavior is selected
per query as `undefined`, `empty` or `refused`; it cannot be inferred from the
requested result type. Duplicate equal entries collapse only when their complete
keys and value digests match; conflicting duplicates refuse the environment.

A population closure binds one qualified type, correspondence digest, authority
identity and revision to a finite, deterministically ordered set of object
identities. `allInstances(T)` includes direct instances and instances of every
closed subtype of `T`, once by object identity. Without proof that both subtype
and object populations are closed at that revision, the query returns
`incomplete` and no partial collection.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-153-AC-1 | Closed all-instances returns every and only member of the selected typed population in deterministic exposure order. | Test (TC-198) |
| FR-153-AC-2 | Unknown closure returns incomplete and a missing qualified key follows its declared absence rule. | Test (TC-198) |
| FR-153-AC-3 | Foreign authority/type keys, duplicate conflicting entries or exceeded bounds refuse without ambient fallback. | Test (TC-198) |
| FR-153-AC-4 | Each declared absence behavior produces its distinct undefined, empty or refused result. | Test (TC-198) |
| FR-153-AC-5 | All-instances includes closed subtype populations exactly once and emits no partial population when closure is unknown. | Test (TC-198) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
