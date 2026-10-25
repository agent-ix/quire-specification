---
id: TC-968
title: "Route state-graph items by advertised provider form"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-331
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-290
    type: verifies
---

# TC-968: Route state-graph items by advertised provider form

## Description

Check that a backend provider declares the state-graph forms it decides and
that negotiation matches each state-graph item to a provider advertising its
exact form.

## Test Procedure

Validate manifest descriptors against the FR-331 schema, then submit one
request item per state-graph form and inspect its candidate set and negotiated
disposition. Repeat with absent, partial, empty, duplicate, unknown and
non-temporal `state_graph_forms` values. For the cross-kind vectors, keep the
manifest and request fixed except for the claim kind.

## Vectors

| Vector | Input | Expected result |
| --- | --- | --- |
| SGF-01 | Temporal descriptor advertises `state_graph_forms` [`possible`, `always-possible`, `unique-path`] | Descriptor admitted; each corresponding state-graph item has that descriptor as a candidate |
| SGF-02 | Temporal descriptor omits `state_graph_forms` | Descriptor admitted and matches no state-graph item |
| SGF-03 | Temporal descriptor advertises only [`possible`] | It matches `possible`; `always-possible` and `unique-path` have empty candidate sets and settle `unsupported`, `unsupported-requested-capability` |
| SGF-04 | A request names a temporal descriptor that omits the requested state-graph form | Empty candidate set and `unsupported`, `unsupported-requested-capability`, naming the form |
| SGF-05 | Temporal descriptor has `state_graph_forms` `[]`, [`possible`, `possible`], or [`eventually-possible`] | Registration refused before consumption with `invalid_capability`/`invalid-state-graph-forms`, keyed by backend identity |
| SGF-06 | Descriptor without `temporal-satisfaction` advertises any `state_graph_forms` value | Registration refused before consumption with `invalid_capability`/`invalid-state-graph-forms`, keyed by backend identity |
| SGF-07 | Same backend manifest with temporal-formula, hyperproperty, and step-relation items | Their candidate sets are unchanged by `state_graph_forms` |

## Expected Results

State-graph negotiation requires exact advertised-form membership. An absent
member means the backend decides none of the state-graph forms, and the member
does not affect negotiation of other temporal, hyperproperty or step-relation
items.
