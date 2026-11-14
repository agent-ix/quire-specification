---
id: TC-047
title: "Classify producer diagnostics from typed causes"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-047
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-271
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-273
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-272
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-272-AC-29
    type: verifies
---
## Description

Planned controls for [FR-047](../functional/foundation/FR-047-emit-typed-located-causes.md).
No executable test or observed coverage is claimed.

## Test Procedure

Trigger each declared failure through its real producer boundary and compare the structured outcomes before/after changing only presentation wording. Audit emitted catalog membership and independently vary similar refusal causes. Use repeated declaration names and multibyte source to verify original locus retention; exercise missing source context without inventing a span.

Select the exact [native diagnostic candidate](../../proposals/quire-v1/definitions/native-diagnostics.md).
Cover every required cause variant at
the actual boundary that can emit it, after that producer adopts a supported
response wire. Compare these paired failures independently:

| Pair/control | Required structured distinction |
| --- | --- |
| Missing definition versus a cyclic definition graph | Missing selection or identified cycle edges; no common message-only fallback. |
| Request retaining a node identity taken from another package versus one retaining its subject's own node identity | `invalid_runtime_input`/`invalid-value` naming the retained and the carried identity versus acceptance; no revision comparison decides it. |
| Valid observation binding with no required record versus supplied record from another invocation | Incomplete missing input versus refused wrong invocation, with separate expected/actual handles. |
| Unknown source tokens versus recognized integer division under a refusing profile | Syntax failure versus profile refusal, preserving the actual stage and construct. |
| Failure to prove a rational divisor nonzero versus a valid predicate evaluating false | Definedness refusal versus ordinary logical violation, without error-code substitution. |
| Missing package member versus duplicate member versus wrong value kind | Distinct typed structural cause with the actual field/index path, independent of Serde's wording. |
| Wrong extraction mode versus conflicting selected package versus wrong binding count | The three retained codes and their distinct typed payloads; deleting one guard must fail that case. |
| A refinement using a feature the producer has not built (`history-row`, `protocol-abstract-side`, `refinement-internal-list`, `nested-reference-row`) versus a declaration form the selected profile prohibits | `unsupported_construct`/`not-yet-implemented` retaining the feature name, declaration and profile identity, versus `declaration-form` retaining the prohibited form; one feature name against another compares unequal (FR-272-AC-20). |
| Sequence declaration without a declared maximum versus a valid model exceeding a caller work budget | Invalid source/model admission versus resource incompleteness; no invented bound or shared generic limit cause. |
| A built backend with a translatable entry on a host that denies executable memory versus an unbuilt backend, an untranslatable entry and a caller budget too small to run | `environment_refused`/`environment-refused` retaining `backend` and `executable-memory`, versus `unimplemented_capability` (`backend-not-supplied`, `untranslatable-entry`) and `resource_exhausted`; raising the budget leaves the first unchanged and a host that grants executable memory runs the entry instead. The test makes the host deny by supplying a denying executable-memory allocator at the backend's allocation seam (or running under an OS policy that refuses executable mappings), not by mocking the diagnostic. |

Supply each valid diagnostic to its strict consumer, then remove a required cause
field, substitute another cause kind while keeping the same message, change the
catalog identity, add an unknown required variant, and erase an originating stage
through a wrapper. Each mutant refuses this interpretation. Localize messages
and change display paths separately: code, typed cause and semantic subject stay
equal. A changed response catalog is retained in response provenance without
silently relinking the checked subject.

For every supported producer interface, enumerate every member of its closed
stage vocabulary through the real producer boundary. Then mutate the interface
identity, add an unknown stage member, reuse a valid label under another
interface and replace the originating pair with a wrapper display stage. Each
mutation refuses or preserves the original opaque pair as required; no global
six- or eleven-member fallback is consulted.

Supply a code/message response lacking a required discriminant. No cause is
guessed from its English text. Combine
a known invalid defect with a missing-input defect, then cap diagnostic detail
to zero: both independent dispositions remain represented and overall success
does not appear from the empty detail vector. Existing code spelling checks do
not discharge these typed-cause or consumer controls.

## Expected Results

Machine consumers discriminate failures using stable typed data only. Classification and identity survive presentation changes. Unavailable context is explicit, not guessed; no uncatalogued literal or message-matching path is accepted as the contract.
Every stage admits only under its exact selected producer interface. Unknown or
cross-interface pairs retain their original bytes and provenance and refuse
without a global-list, display-stage or first-wins fallback. Projection-refusal
codes remain in the separate FR-283 catalog even when a spelling resembles a
native producer code.
