# Native finite state core definition

Profile identity: `quire.state.core/v1`; revision: `1-draft.3`.
Language: `ix:native` / `1-draft`. **Proposed, not adopted or implemented.**

This successor changes the required edition selection to its stage/correspondence
correction. State forms and the selected state-rule bytes remain unchanged from
`1-draft.2`; previous candidates retain their own dependency closure.

## Required definition

| Identity | Revision | Definition artifact | Exact byte digest |
| --- | --- | --- | --- |
| `ix:native` / `1-draft` | `1-draft.2` | [Edition](edition-1-draft-2.md) | `sha256:3070ddb8bef9ffb0eea70d984dc663ba48f8410d9558c958bd2b3e088932575b` |

## Admitted forms and meaning

This profile admits invariant, operation precondition and postcondition
declarations. Shared expressions admit Boolean, contextually typed bounded
integer, explicit rational, text and enum literals; resolved value/parameter
reads; acyclic record paths; grouping, `let`, `if`; `not`, `and`,
`or`, `implies`; admitted scalar equality/order; numeric negate, `+`, `-`,
`*` and rational `/`; `present`, guarded `value`, `size`, `contains`,
`forall`, `exists`; and invocation-bound `pre(expr)` in postconditions.

All source constructs and retained model declarations must satisfy the selected
state rules, including unused declarations and unselected branches. Range and
nonzero proofs apply to potentially evaluated operations under sound native
control facts. Scalar types, units, bounds, presence and source declarations
come from exact supplied model contracts. No default numeric maximum or field
is derived from example data. Sequences are finite, ordered and duplicate
preserving with authored maxima at most 10,000 per wrapper.

Whole-record/sequence/option equality, reference/object identity and navigation,
`reaches`, named predicate declarations/calls, `filter`, `map`, `count`
and `sum` require other definitions and are refused by this profile.
No sequence indexing or slicing production is selected by the shared grammar.
Integer slash/div/rem/mod, float, implicit conversion, silent widening,
recursive value containment, ambient lookup, reflection and I/O are refused.
Rational division uses the same named dimensionless rational type, exact
nonzero proof and normalized result bounds; it never admits integer division.

Temporal/protocol declarations require their own profiles. A state-core success
does not claim full composed-v1 capability. Historical `state-finite/0-draft`
and the first-profile amendment retain their separate identities; this new
edition/profile does not relabel their sources or results.

## Selected state rules

The state contract defines semantics for admitted forms; descriptions of query
or graph extensions do not grant those forms here. Its earlier first-profile
comparison is compatibility context; the exact admitted-form catalog above and
selected composed rules define this candidate.

Selected at repository revision `033fcda3cc9f964d018b466faa12cd14c8a9878a`; digests cover exact file bytes.

| Rule file | Exact byte digest |
| --- | --- |
| [state-contract.md](https://github.com/agent-ix/quire-specification/blob/033fcda3cc9f964d018b466faa12cd14c8a9878a/proposals/quire-v1/state-contract.md) | `sha256:c81d7f9f7463f879ce13be754859b20d78228c3b8f206f073352fc1386b7645e` |
| [FR-039-normalize-exact-rational-literals.md](https://github.com/agent-ix/quire-specification/blob/033fcda3cc9f964d018b466faa12cd14c8a9878a/spec/functional/FR-039-normalize-exact-rational-literals.md) | `sha256:6775bc27f3bf0f9a0bf2afc685126584fb8e2ecdfe69bfa8630057249673c98e` |
| [FR-044-check-composed-numeric-definedness.md](https://github.com/agent-ix/quire-specification/blob/033fcda3cc9f964d018b466faa12cd14c8a9878a/spec/functional/FR-044-check-composed-numeric-definedness.md) | `sha256:ec97683039bae198463d729db4754ca707c28331698bc5c5dd64a6ec1aa35ab4` |
| [FR-045-preserve-control-and-presence-facts.md](https://github.com/agent-ix/quire-specification/blob/033fcda3cc9f964d018b466faa12cd14c8a9878a/spec/functional/FR-045-preserve-control-and-presence-facts.md) | `sha256:c71031ec7f2d778685707d3c3cc28fa7e67221b94285e28746eb1ab58ed01530` |
| [FR-046-validate-state-invocation-inputs.md](https://github.com/agent-ix/quire-specification/blob/033fcda3cc9f964d018b466faa12cd14c8a9878a/spec/functional/FR-046-validate-state-invocation-inputs.md) | `sha256:d97c9e05fd9dddcf7188dd1a539356c2db95b4c657ec714a094f084d8791e61a` |
