---
id: TC-278
title: "Replay envelope members"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-323
    type: verifies
---

# TC-278: Replay envelope members

## Description

Show that an FR-323 replay request's `replay` property carries the
`ReplaySource`, the replay target (a function's `QualifiedName` or a frame operation), for a function request arguments keyed by
parameter node id, the semantic profile selections, for a function request the failing node's
occurrence key, the proof bounds and declared domains, the trace position
and the `backend` member, and that a non-replay request carries no `replay`
property, per QSL ADR-013 O-25, O-26 and QC-8.
Extend the same owning envelope case for the proposed supplied-phase
request/result amendment, binding FR-323-AC-10..16 without a new TC id.

## Preconditions

- FR-197's `ReplaySource` structural separation (`Witness`/`Input`) exists.
- FR-322 owns the `source_map` occurrence key (`node_id`, `role`, `ordinal`).
- QSL ADR-011 E9: the `replay` facade resolves the selected function by
  `QualifiedName`, or the selected frame operation by `object` and `operation`,
  against the recompiled package's declarations.
- Supplied vectors require the reviewed real QSV FR-109 / QSL FR-277
  bounded path and actual QSL-675 typed native mapping before execution is
  qualified. Default vectors use the published A/H default 16777216 units each
  (QSL FR-277). Decimal waits for IR-718/IR-719 and typed
  integration, while its positive requirement remains mandatory once enabled.

## Test Procedure

Build an FR-323 replay request whose `replay` property carries every named
member, and the mutations that omit one member or supply a non-replay
request. Execute each vector through the real local Rust boundary and read
back the admitted request or refusal.

For the extension below, use the same real typed envelope reader and
runtime result producer, not a schema-only producer or generated-oracle
shortcut. Independently enumerate a qualified supplied fixture's logical
events and A/H totals; do not read the counter under test for the expected
sequence. Hold unrelated pre-call and evaluator limits sufficient when
isolating a boundary. Preserve existing RE-01..11 and byte/dependency
precedence from TC-277; no change to original replay member custody.

For RE-19, independently enumerate the successful events before the injected
real reservation denial. Let incoming spend be s > 0 and the retained
successful prefix at denial be S = s + P, where P is that independently
expected performed work; the denied reservation/event is not spent.
After removing denial, start the retry and its fresh comparison with
identical input, admitted environment, original Cancel and initial spend S.
Do not reset/refund to zero or s. Independently enumerate the actual retry
sequence's N further logical events and run each partner at ceilings
S+N-1 and S+N (within u64), all other bounds sufficient. At one-less compare
the complete actual first-denial record and expected successful S+N-1;
at exact compare membership and successful S+N. Both prefix preservation
and additional spend must be observed, not inferred from equal Booleans.

## Vectors

| Vector | Mutation | Expected result | Criteria |
| --- | --- | --- | --- |
| RE-01 | A replay request whose `replay` property carries the `ReplaySource`, `function`, parameter-node-id-keyed arguments, semantic profile selections, occurrence key, proof bounds and declared domains, trace position and `backend` member | Admitted; every named member is present | FR-323-AC-6 |
| RE-02 | The same request with its `function` member removed | Refused before consumption | FR-323-AC-6 |
| RE-03 | The same request with an argument keyed by ordinal position instead of parameter node id | Refused before consumption | FR-323-AC-6 |
| RE-04 | The same request with its occurrence key replaced by a bare node id | Refused before consumption | FR-323-AC-6 |
| RE-05 | The same request with its `backend` member removed | Refused before consumption | FR-323-AC-6 |
| RE-06 | The same request with its `ReplaySource` member removed | Refused before consumption | FR-323-AC-6 |
| RE-07 | The same request with its semantic profile selections member removed | Refused before consumption | FR-323-AC-6 |
| RE-08 | The same request with its proof bounds member removed | Refused before consumption | FR-323-AC-6 |
| RE-09 | The same request with its declared domains member removed | Refused before consumption | FR-323-AC-6 |
| RE-10 | The same request with its trace position member removed | Refused before consumption | FR-323-AC-6 |
| RE-11 | A non-replay `NativeRuntimeEnvelope` request | Carries no `replay` property | FR-323-AC-6 |
| RE-12 | Equivalent applicable replay/non-replay requests with explicit complete `limits.supplied` A/H; hold package/input/selection/options fixed and vary A alone then H alone; also hold both fixed | Either changed effective ceiling changes request identity, not package identity; both fixed with other inputs fixed retains request identity; existing scalar/population fields and per-argument C/occ units unchanged | FR-323-AC-10 |
| RE-13 | Move either supplied ceiling into replay `stage_limits`; omit a ceiling from the wire object (incomplete wire object; caller-API omission is RE-14), add an unknown member or use an out-of-u64 value | Misplaced stage setting refuses before stages; incomplete/malformed typed supplied object refuses before consumption; no evaluator/stage/population/default-borrowing fallback | FR-323-AC-10 |
| RE-14 | Request with no supplied phase, and caller-API request omitting both ceilings, and a request stating `supplied.admission_work_units` 16777216 and `supplied.conversion_work_units` 16777216 explicitly | No-phase existing request shape remains usable without supplied object; both supplied requests normalize to 16777216 each (and `numeric_magnitude_bits` to 4096) and have the SAME request identity, so an implementation whose identity changes when a value was defaulted fails | FR-323-AC-10 |
| RE-15 | Qualified supplied fixture at zero, actual N-1 and N in each A/H counter with all other bounds permitting progress | Zero (also a zero ceiling, FR-323-AC-10) denies first event; one-less denies actual last event unperformed/unspent with correct phase/parameter/event/locus, limit, successful prefix, next amount and checked counter; exact permits that phase; no evaluation consumed by pre-call stop | FR-323-AC-11 |
| RE-16 | Checked pre-call counter overflow; then actual evaluated accounting denial after pre-call bounds permit entry | Pre-call overflow is explicit without wrapping; denied event is not performed or spent and successful prefix is unchanged; evaluated counterpart retains exactly the original accounting limit record/real charge_point, no supplied context; no fabricated FunctionCall | FR-323-AC-11 |
| RE-17 | Identical supplied inputs/environment and same nonzero incoming budget, multiple arguments/nested helper, paired cold/warm caches | Same logical count and complete first-denial record; original caller budget/Cancel persists, one initiating owner, no cache/entry/argument reset or double debit | FR-323-AC-12 |
| RE-18 | Actual wrong-kind/domain input and original Requested/Deadline cancellation at a deterministic supported helper boundary | Original native typed cause/provenance and actual phase/parameter/locus retained; neither becomes limit/failed predicate; no further event/partial admitted value or evaluation debit | FR-323-AC-12 |
| RE-19 | Deny actual fallible worklist reservation; separately trigger checked representation-size overflow; remove allocation denial and compare retry/fresh attempts with identical input/environment/original Cancel and SAME initial nonzero retained successful spend S; pair independently enumerated S+N-1/S+N ceilings | Storage preserves actual measured bytes/additional-elements; capacity distinct from allocation/counter overflow; original native operation/cause retained. No reset/refund of S or partial state; one-less retains expected S+N-1 spend and identical complete first-denial record, exact yields same membership and expected S+N spend; result equality alone cannot pass | FR-323-AC-12 |
| RE-20 | Mutate supplied context with fake charge_point/FunctionCall, wrong phase/event/prefix/next metadata, unknown cause, fabricated native selection or mixed evaluated/pre-call origins | Strict interpretation refuses malformed context; no omission or reinterpretation produces successful admission or a guessed native cause | FR-323-AC-11, FR-323-AC-12 |
| RE-21 | Analyze each helper/cause qualification record, omitting one input/work/peak-storage/Cancel/fallible-route/typed-mapping premise at a time; include tiny Decimal [0,100], 1@0, min/max scale 2 and unpublished union negative controls | Missing premise blocks implementation qualification, not a runtime unsupported/invalid/limit/storage error; IR-718/IR-719 plus reviewed typed integration enable mandatory Decimal positive replay, future union needs its owning public baseline | FR-323-AC-13 |
| RE-22 | Supply the existing typed ReplayRefusal::Fault/InternalFault at the conversion stop seam for replay stage and each existing converted-payload/converted-slots/converted-elements/one-converted-argument invariant; pair with ordinary input refusal and genuine evaluated fault | Native Failed/internal-failure result with original stage/invariant/provenance and unavailable replay basis, supplied absent, no fake kernel payload/FunctionCall/charge_point; pre-call semantic spend zero and no mapping/helper debit or refund of earlier prefix; evaluated control retains its actual prior charges, ordinary input control retains refusal/Inconclusive and its supplied input context. Five-arm-only drop/coercion or evaluated-fault relabelling fails | FR-323-AC-14 |
| RE-23 | Integer-typed supplied fixture, request states `limits.supplied.numeric_magnitude_bits` B, value and interval endpoints at bit length exactly B (B = 64 with value 2^64-1; B = 1 with value 0, since zero has bit length ONE); hold A/H and all other bounds sufficient. Run the SAME fixture (same types and values) with B = 4096 stated | Admitted; membership result is the independently expected one; A and H spend equal the independently enumerated event totals of this same fixture and are identical for B = 64, B = 1 and B = 4096 (B adds no spend) | FR-323-AC-15 |
| RE-24 | Same fixture with the supplied value at bit length B+1 (2^64 at B = 64), then separately the lower endpoint, then the upper endpoint at B+1 with the value within B; and B = 0 with value 0 | Each refuses by the `limit` cause with field `supplied.numeric_magnitude_bits`, `limit` B, `consumed` 0, `next_charge` and `counter` B+1 and `event` `value-visit`, `operand` `value`, `lower`, `upper` respectively, in that checking order when several exceed; B = 0 with value 0 refuses with `operand` `value`, `next_charge` and `counter` 1 (B = 0 admits nothing); the stop is on the charged initiating event, so the A (or H) spend of the phase equals the independently enumerated prior spend PLUS that one event, the numeric comparison is unperformed, nothing further is charged, zero evaluation consumption, replay `inconclusive`/`NoValue` | FR-323-AC-15 |
| RE-25 | Rational fixture, B stated: each of the six operands (`value-numerator`, `value-denominator`, `numerator-lower`, `numerator-upper`, `denominator-lower`, `denominator-upper`) in turn at exactly B bits, then in turn at B+1 bits with the other five within B | At exactly B all six admit; at B+1 each refuses by the same `limit` cause naming that `operand`, `consumed` 0, `next_charge` and `counter` the B+1 bit length, `event` `value-visit`, stopping on the charged initiating event (phase spend includes that one event) before any comparison; a negative numerator is measured by absolute value; zero has bit length ONE, so a zero operand with B = 0 refuses with `next_charge` and `counter` 1 and with B >= 1 admits | FR-323-AC-15 |
| RE-26 | Equivalent requests with the same package/input/selection/options and explicit A/H: B omitted at the caller API; B stated 4096; B stated 4096 again; B stated 4095; B stated 4097 | Omitted, stated 4096 and restated 4096 have the SAME request identity and the effective wire object carries 4096 (an implementation whose identity changes when the default was applied fails); 4095 and 4097 each differ from it and from each other; package identity is the same in every case | FR-323-AC-15 |
| RE-27 | B omitted at the caller API with an Integer interval type, a Rational domain type, and a type needing neither; compare with B stated 4096 | Omitted B behaves exactly as B = 4096 for all three types: a 4096-bit value admits and a 4097-bit value refuses by the AC-15 `limit` cause with `limit` 4096; the type needing no numeric comparison is unaffected. No unsupported outcome exists in the envelope | FR-323-AC-15 |
| RE-28 | Malformed B: -1, 1.5, 18446744073709551616 (above the u64 range), `null`, "64", true, and a missing `numeric_magnitude_bits` in an otherwise complete wire object; and B moved into replay `stage_limits` | Each is a typed input refusal before consumption (the misplaced B under the unknown-stage-setting rule before stages run); no fallback to a default or `integer_bits`; A, H and `integer_bits` settings are unchanged by a stated B | FR-323-AC-15 |
| RE-29 | NORM. Nested type links `Option<Int[0,9]>` and `Sequence<Int[0,9]>`: with the expected and actual endpoint set widened in turn, each of the four Integer endpoints (`type-link-expected-lower`, `type-link-expected-upper`, `type-link-actual-lower`, `type-link-actual-upper`) at exactly B bits, then in turn at B+1 bits with the other three within B; a Rational pair with each of the eight endpoints (the four names with `-numerator`/`-denominator`) likewise, and a Decimal pair with each of the four coefficients (`type-link-expected-lower-coefficient`, `type-link-expected-upper-coefficient`, `type-link-actual-lower-coefficient`, `type-link-actual-upper-coefficient`) likewise | At exactly B all admit; at B+1 each refuses by the `limit` cause with that `type-link-` `operand`, `limit` B, `consumed` 0, `next_charge` and `counter` the B+1 bit length and `event` `type-comparison`, stopping on the charged `type-comparison` event (phase spend includes that one event) before the endpoint equality runs; the first over-B endpoint in the stated order decides; the type-link helper and its QSV caller exist, so the vector is runnable | FR-323-AC-16 |
| RE-30 | A frame counterexample's replay request carrying `frame_operation` `{object, operation}`, no `function`, no `arguments` and no occurrence key | Admitted; the replay target resolves by `object` and `operation` | FR-323-AC-18 |
| RE-31 | RE-30's request with `frame_operation` removed | `invalid_runtime_input`/`missing-member` at `/replay` | FR-323-AC-19 |
| RE-32 | RE-30's request with a `function` member added | `invalid_runtime_input`/`invalid-value` at `/replay/frame_operation` | FR-323-AC-19 |
| RE-33 | RE-32's request with a byte-provision entry removed | `invalid_runtime_input`/`invalid-value` at `/replay/frame_operation`, not the provision refusal | FR-323-AC-19 |
| RE-34 | RE-30's request with a stale proved `package_id`, and `operation` naming an undeclared operation | `stale_dependency`/`content-mismatch` (rule 5), not `missing-name` | FR-323-AC-20 |
| RE-35 | RE-30's request with a current `package_id` and: `operation` undeclared; `operation` naming an operation no clause or attempt names (no frame node is emitted for it, QSL FR-105, ADR-017); `object` naming a record value type; `object` naming a subtype that only inherits `operation`; `operation` exposed by two supertypes | `missing_declaration`/`missing-name`; `missing_declaration`/`missing-name`; `invalid_model_binding`/`malformed-declaration`; `invalid_model_binding`/`malformed-declaration`; `ambiguous_declaration`/`ambiguous-name`, each naming the target | FR-323-AC-20 |
| RE-36 | RE-30's request with `arguments` added; then with an occurrence key added | `invalid_runtime_input`/`unknown-member` at `/replay/arguments`; at `/replay/occurrence_key` | FR-323-AC-19 |

## Expected Results

RE-01 finds every QC-8 replay member present on a well-formed replay
request. RE-02 through RE-10 find each of the nine named `replay` members
refused before consumption when it is missing or malformed, in turn. RE-11
finds the `replay` property entirely absent from a non-replay request.

RE-30 is admitted; RE-31, RE-32 and RE-36 refuse the member shape with their stated code and locus, RE-33 shows the member refusal before the byte-provision rule, RE-34 the `package_id` rule before the resolution, and RE-35 each resolution outcome of FR-323-AC-20.

RE-12..20 and RE-23..29 require the complete typed request/result forms and distinct
pre-call provenance of FR-323, with unchanged canonical contract_version,
closed evaluated accounting record, byte provision/dependency precedence
and zero semantic consumption for pre-call stops. Actual input/cancel/
storage/capacity causes remain native typed causes, not messages or invented
catalog variants. RE-21 is Analysis evidence bound to FR-323-AC-13, not a
claimed runnable unsupported-result trace. Capability/mapping
prerequisites are not discharged by authored shapes.
The unchanged QSL-503 full Tree and evaluator parity controls remain owned
by QSL TC-830/831/906 and cannot be replaced by this envelope case.
RE-22 verifies the existing native fault route OUTSIDE the five supplied
causes: it is not a new kernel cause, new API or a claim that ordinary inputs
can reach an invariant break. Compare original typed accessors/payloads, not
display strings. Fault transport qualification still needs the actual native
binding; a missing binding blocks qualification, not a fabricated outcome.

## Status

The supplied-phase extension RE-12..29 is PROPOSED, PLANNED/UNRUN and
independently unreviewed. No new fixture implementation, native helper,
reader qualification or test pass is claimed.
