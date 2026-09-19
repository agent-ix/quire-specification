---
id: FR-251
title: "TL target evaluator profile"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-095
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-250
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-025
    type: references
---
# FR-251: TL target evaluator profile

## Description

When a native temporal clause is lowered, the reader SHALL treat the TL evaluator profiles listed here as lowering targets of a versioned correspondence rather than as authored Quire temporal profiles.

FR-200 records that `tl-*` syntax and evaluators "are consumed behind a
versioned native-Quire lowering boundary; they are not a second authored source
language". FR-095 states that "The generated TL artifact is internal derived
output and never editable source."

## Values

| Value | Description |
| --- | --- |
| `mltl.closed-trace/v1` | The TL target named by FR-095 for "Event-position false-extension, bounded future core, complete execution", supported "when every bridge premise below holds". FR-095 also records it as the target of two unsupported rows: "Finite-window future semantics" is "Unsupported because TL extends atoms false beyond closure". FR-091 pins the native lower-bound convention to it: the `until` convention "matches `mltl.closed-trace/v1` and shall not be replaced by a different unbounded-until convention under the same identity." |
| `mltl.online-prefix/v1` | The TL target named by FR-095 for "Event-position false-extension, bounded future core, open prefix", supported "when every bridge premise below holds". |

FR-095 additionally records "The same TL profiles" as the target for
fixed-sample false-extension bounded future core, "Supported only with one total
valuation per required sample and retained epoch/period/unit correspondence",
and "Current TL profiles" as the target of the two unsupported rows for
timestamped-event finite-window and for "Any bounded past operator". FR-095
also records "No TL target" for an infinite-trace unbounded property, resolving
to an absent target rather than to either labeled member; FR-161 analyzes that
request natively instead of lowering it. No requirement enumerates any TL
profile identity beyond the two labels above, so no further member is supplied
here.

These labels are **not** members of
[FR-250](FR-250-temporal-semantic-profile.md). A native clause never selects one
of them, and selecting one of them is never a substitute for the authored
profile selection required by FR-090.

## Identity and comparison

- **Identity kind** — *value*. A TL profile label has no identity of its own and
  is compared componentwise as an exact string.
- **Key** — the label itself, matched exactly, as it appears inside the bridge
  definition whose "identity, revision and digest naming the TL syntax and
  evaluator profiles it targets" FR-095 Inputs require. Equality is
  componentwise over exactly that and nothing else. Display name, source path,
  span, message text, locale and rendering never participate
  ([FR-009](../../functional/foundation/FR-009-preserve-object-identity.md),
  [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md)).
- **Key order** — lexicographic over the label, for collection comparison only.
  It carries no precedence, capability ranking or causal meaning, and it is not
  an order over traces, positions or instants.
- **Causal order** — none.
- **Absence** — a native request with no admitted TL target has an absent target
  profile, and absence compares only to absence. FR-095 requires "a typed
  unsupported/refused mapping naming every unmatched semantic dimension", and
  FR-095-AC-3 holds that "Timestamped-event and past-time requests return typed
  unsupported mappings at current TL versions without emitting a substitute
  formula." An absent target is never defaulted to a member.
- **Digest domain** — the bridge definition digest that carries this label
  retains the domain it was derived under
  ([FR-201](../foundation/FR-201-canonical-identity-domain.md)); an identity
  crossing domains is unequal regardless of byte equality.

## A TL profile label is not a correspondence

FR-095 states that the bridge "shall not treat a formula match without its
valuation, capture and clock mapping as semantic correspondence", that it "shall
keep TL proposition identifiers distinct from native predicate, signal and
capture identities", and that it "shall not use a backend, syntax or historical
result to repair an unknown mapping, substitute a profile or synthesize missing
correspondence." FR-095-AC-2 records the concrete rejection: "The one-position
`always[0,1] p` vector rejects a finite-window-to-`mltl.closed-trace/v1`
equivalence claim."

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-251-AC-1 | Each member's value equals the exact label string in the table. | Inspection |
| FR-251-AC-2 | No member of this vocabulary is accepted where an authored temporal profile identity is required. | Analysis |
| FR-251-AC-3 | A timestamped-event, bounded-past or infinite-trace request resolves to an absent target rather than to a member. | Analysis |
| FR-251-AC-4 | Equal TL formula bytes under different bridge definition identities compare unequal. | Analysis |

## Dependencies

- [FR-095](../../functional/temporal/FR-095-preserve-native-tl-correspondence.md)
  establishes the support table, the target labels and the unsupported rows.
- [FR-091](../../functional/temporal/FR-091-evaluate-bounded-future.md) pins the
  lower-bound convention shared with `mltl.closed-trace/v1`.
- [FR-250](FR-250-temporal-semantic-profile.md) holds the authored profile
  vocabulary this one is kept separate from.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the
  identity discipline.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage.
