---
id: FR-323
title: "Native runtime request and result envelope"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/interface_005
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-180
    type: references
---

# FR-323: Native runtime request and result envelope

## Description

When a checked package is executed or replayed, the producer SHALL use the immutable NativeRuntimeEnvelope contract below rather than an untyped map, display string or implementation-local structure.

## Properties

- `contract_version` — exactly `quire.native-runtime/v1`.
- `request_id` — identity over package, selection, inputs, limits and options.
- `package` — exact FR-322 checked-package reference; a non-replay request
  carries the reference alone, and a replay request adds the `sources` and
  `dependencies` members below, as
  `{package_id, contract_version, sources, dependencies}`. Definition and
  domain-package bytes are reached through `byte_provision` by the digests
  the recompiled sources name.
  `sources` is the proved package's own FR-322 lock `sources`.
  `dependencies` holds one entry per entry of the proved package's
  `dependency_selections`, in the same order, each
  `{identity, package_id, sources}`: `identity` and `package_id` are that
  selection's, and `sources` is the selected
  dependency package's own FR-322 lock `sources`. The producer copies these
  from the proved package and its admitted dependency packages and invents
  none. The executor supplies each dependency, recompiled from its entry's
  `sources`, under the entry's `identity` to every import of that identity
  (FR-307). It refuses with the first of these, each rule
  applied over all entries, in entry order, before the next rule, and
  yields no result:
  1. before any recompile, entries not in strictly ascending UTF-8 byte
     order of `identity`, a repeated identity included, refuse
     `invalid_package`/`invalid-value` at `/package/dependencies`;
  2. building the dependency input refuses an entry whose source has the
     authority and identity of the proved package's source or of another
     entry's source as `invalid_package`/`conflicting-definition`, naming
     both; then the recompile's own refusals, FR-307's among them: an import
     whose identity no entry supplies refuses
     `missing_import`/`missing-selection`;
  3. an entry that no import of the recompiled package reaches refuses
     `invalid_package`/`invalid-value` at `/package/dependencies`;
  4. an entry whose `package_id` differs from the `package_id` its sources
     recompile to refuses `stale_dependency`/`content-mismatch` (QSL's
     `DependencyIdentityMismatch`), naming the entry's `identity`, its
     `package_id` and the recompiled one. An import carries no digest, so
     this is the rule that catches an edited dependency the recompile
     admits, such as an edit to a declaration the proved package does not
     call or to the body of a called function with its signature unchanged:
     the entry's `package_id` is the one the proved package's lock pinned.
     An edit that removes or retypes an export the proved package uses
     makes the recompile itself refuse at rule 2 (`missing_declaration` or
     `ill_typed`) before this rule runs;
  5. a recompiled proved `package_id` other than the request's refuses
     `stale_dependency`/`content-mismatch` (QSL's `PackageIdMismatch`),
     naming the request's `package_id` and the recompiled one. An edited dependency also
     changes the proved package's `package_id`, so rules 3 and 4 run first
     and name the stale dependency.
- `byte_provision` — present on a replay request only: the digest-addressed
  bytes of every source, definition and domain-package input the replay
  recompiles, each keyed by its declared digest domain and digest (QSL
  ADR-013 QC-1). The executor obtains every recompilation input from this
  provision and never from a path, environment variable or search location.
  Before any recompile, an input the `package` reference names that the
  provision does not carry refuses `missing_import`/`missing-selection`,
  naming the requested digest record (domain and digest) and the member of
  the `package` reference that names it; an entry under a raw-byte digest
  domain such as `quire.source.bytes/v1` whose bytes do not match its
  declared digest refuses `stale_dependency`/`byte-digest-mismatch`, naming
  the declared digest and the digest of the supplied bytes; and an entry
  under `sha256-jcs` whose document canonicalises to another digest refuses
  `stale_dependency`/`content-mismatch`, naming the selected and the
  recomputed digest.
- `selection` — ordered clause/function identifiers.
- `state_environment` — typed anchors, values, object populations, frames and invocation captures.
- `limits` — the `quire.value.accounting/v1` limit objects: `ScalarLimitsV1`,
  plus `PopulationAdmissionLimitsV1` when the request binds a population.
  Work maps to `work_units`, output to `result_units`, and allocation to the
  high-water size counters. A zero limit admits zero units. The supplied-value
  amendment below adds `limits.supplied` without adding fields to either
  existing accounting object or changing its identity, units or defaults.
- `stage_limits` — present on a replay request: one entry per stage limit
  of the proving run, keyed by its FR-461 setting name (for example
  `s1.tokens` or `s3.nodes`) with its configured value. The executor runs
  each recompile stage under exactly these values, and a stage limit the
  request does not name under its published default (FR-460). An entry
  whose name is not a stage-limit setting name refuses before any stage
  runs.
- `results` — one ordered typed disposition per selected item.
  Supplied conversion/admission limit, input, cancellation, storage and
  capacity stops carry the typed `supplied` context below, never an
  evaluated-call limit record. An existing pre-call `InternalFault` instead
  retains the separate native fault route specified below.
- `replay` — present only on a replay request, and absent otherwise. It
  carries: the originating counterexample identity and expected originating
  verdict; the `ReplaySource` (`Witness` or `Input`, FR-197); the selected
  function's `QualifiedName`; arguments keyed by parameter node id; the
  semantic profile selections; the failing node's occurrence key
  (`node_id`, `role`, `ordinal`, FR-322 `source_map` key); the proof bounds
  and declared domains carried from the obligation identity; the trace
  position; and the `backend` member (QSL ADR-013 O-19). No replay member is
  inferred from the package or from the non-replay properties above (QC-8).

## Supplied-value phase amendment

This is a PROPOSED amendment of this closed bootstrap envelope, not a claim
of a released reader or qualified implementation. Its
`contract_version` remains exactly `quire.native-runtime/v1`; no compatibility
reader, private extension or alternate version is introduced. The existing
closed validation, byte-provision/dependency precedence and replay-only
members remain unchanged. The authoritative producer/consumer reader must
implement and qualify this amended shape before claiming it is supported.

### Request ceilings

For a request that supplies function arguments, the effective typed `limits`
SHALL additionally carry `supplied: { admission_work_units,
conversion_work_units, numeric_magnitude_bits }`. All three members SHALL be exact unsigned 64-bit
ceilings, with no other members in this object. Admission bounds A; conversion
bounds H when witness/input conversion occurs. A direct call over already
constructed values does not consume H merely because its ceiling is carried.
Requests with no supplied-value phase retain their existing limit objects
and may omit `supplied`; a supplied-value request lacking an effective
applicable ceiling SHALL NOT be admitted with an unbounded or evaluator-derived
fallback. Zero admits zero logical units.

QSL FR-277 owns the A/H units and defaults; QSL FR-098 and FR-255 carry the
same names. The members are `supplied.admission_work_units` (A) and
`supplied.conversion_work_units` (H), each with the published default
16777216 units. Caller API omission SHALL be resolved from each field's own
published default into this complete effective wire object before
consumption and request identity calculation. Existing ScalarLimitsV1 and
population omission/default rules SHALL NOT change. A default is never
derived from the evaluation allowance, the population admission ceiling or
any stage setting.

`stage_limits` SHALL continue to carry only FR-460/461 recompile stage settings.
Neither supplied field is a stage setting; placing it there SHALL refuse
under the existing unknown-stage-setting rule before stages run. Changing
either effective supplied ceiling SHALL change request identity, not package
identity, with package/input/selection/options otherwise fixed. Holding
both effective ceilings and those other inputs fixed SHALL retain request
identity. Replay/non-replay structural separation is unchanged.

A and H SHALL each retain one cumulative budget across all supplied arguments
and nested helpers, using the original caller Cancel. One actual event has
ONE initiating owner: conversion-triggered QSV work spends H; direct/final
membership spends A, with final replay membership retaining the admission
budget already spent. Neither budget resets on entry/cache/helper/argument
boundaries, seeds the other, nor debits the evaluator Meter. QSL FR-098's
converted-node C and semantic occurrence limits remain per argument with
their original units and zero call-meter debit. Population admission is
independent and SHALL NOT be renamed or repurposed as A.

### Numeric magnitude bound

The `limits.supplied` object carries a third member,
`numeric_magnitude_bits` (B): an exact unsigned 64-bit count of bits, the
caller-owned finite magnitude bound for supplied Integer and Rational
admission, with the published default 4096 bits. It is the only carrier of B.
B is not `integer_bits` of `ScalarLimitsV1` (an evaluation counter that keeps
its own meaning and its `u64::MAX` default), is not a QSL family check
constant, and is not declared per type or per argument. The effective wire
object is complete: `admission_work_units`, `conversion_work_units` and
`numeric_magnitude_bits` are its only members and all three are required on
the wire.

1. **Default.** B has the published default 4096, like A and H each has its
   own. Caller API omission SHALL resolve to 4096 in the complete effective
   wire object before consumption and request identity calculation. Nothing
   else derives B: not `integer_bits`, A, H, the evaluation allowance, the
   population ceiling, a stage setting, a declared type, or `u64::MAX`. A
   stated B, including 0, overrides the default exactly.
2. **Identity.** B is part of the effective `limits.supplied` object. A
   request that omits B and a request that states 4096 have the SAME request
   identity; a request stating any other B (4095, 4097) has a different
   request identity, not a different package identity. Two requests that state
   the same B with the other inputs fixed have the same identity.
3. **Malformed B.** On the wire, a missing `numeric_magnitude_bits`, or one
   that is not an exact JSON unsigned integer in u64 range (negative,
   fractional, above the u64 range, `null`, a string or any other type), or
   that appears in `stage_limits`, is a typed input refusal before
   consumption, as an incomplete or malformed `limits.supplied` object is.
4. **Omitted B.** An omitted B means the published default 4096; there is no
   separate unsupported outcome in this envelope. (A QSV-level caller that
   passes NO B to QSV supplied admission still receives QSV's typed
   `UnsupportedHelper { family: NumericDomain }`; that is QSV's own contract,
   outside this envelope.)
5. **Coverage, value-in-domain.** Every Integer or
   Rational operand of one value-in-domain comparison is checked against B
   after the initiating event is charged and before that comparison runs, in
   this order, the first operand over B deciding. Integer: `value`, `lower`,
   `upper` (3 operands). Rational (a numerator interval plus a denominator
   interval, QSV `RationalDomain`): `value-numerator`, `value-denominator`,
   `numerator-lower`, `numerator-upper`, `denominator-lower`,
   `denominator-upper` (6 operands). An operand's size is the bit length of
   its absolute value, with zero counted as ONE bit (QSpec
   `proposals/quire-v1/definitions/value-accounting.md`, `bits(x)`, "zero is
   one"; quire-exact FR-380 Inputs: zero has magnitude length one). Every value
   therefore has bit length at least 1: B = 0 admits no operand and B >= 1
   admits zero. An operand is within B when its bit length is at most B; at
   the default, a value of 4096 bits is admitted and a value of 4097 bits is
   refused. B applies in whichever phase initiates the comparison, and a
   declared endpoint is checked against B like a supplied value. The bounded kernel helper
   exists under quire-exact FR-380; the QSV caller is pending.
6. **Coverage, type-link comparison (NORM, NOT YET IMPLEMENTATION-QUALIFIED).**
   QSV FR-109 "Nested retained type": type-link comparisons carry input-sized
   numeric data. The `type-comparison` event (one compared pair of chain
   links) is charged first; then, before the endpoint equality runs, every
   numeric endpoint of the pair is checked against B by the same bit-length
   rule (zero is one bit). For an Integer type-to-type comparison there are 4
   operands: `type-link-expected-lower`, `type-link-expected-upper`,
   `type-link-actual-lower`, `type-link-actual-upper`. For a Rational pair
   there are 8: each of those four names with a `-numerator` and
   `-denominator` suffix on the matching interval, checked in the order
   expected, then actual, numerator interval before denominator interval,
   lower before upper. For a Decimal pair there are 4: `type-link-expected-lower-coefficient`,
   `type-link-expected-upper-coefficient`, `type-link-actual-lower-coefficient`,
   `type-link-actual-upper-coefficient`, each the coefficient of the named
   Decimal endpoint, checked in the order expected, then actual, lower before
   upper (a Decimal endpoint's scale is not a bounded operand). The first over B stops ON the charged `type-comparison`
   event with the same `limit` cause below, `event: type-comparison`. This
   part depends on the IR type-link follow-up: the bounded cancellable
   type-to-type equality helper and its QSV caller are not built, so it is
   not yet implementation-qualified and no implementation is claimed.
7. **Over B.** An operand above B yields the `limit` cause below with
   `field: supplied.numeric_magnitude_bits`, `limit_kind:
   numeric_magnitude_bits`, `limit` = B, and an `operand` member naming the
   operand from the lists above (type-link operands carry the `type-link-`
   prefix). The row's definitions hold unchanged. B is a per-operand
   magnitude bound with no cumulative counter, so its owning counter's
   successful prefix is the context member `consumed` = 0, `next_charge` = the operand's bit
   length (the denied amount) and `counter` = consumed + next_charge = the
   same bit length. The stopping event is reported in `event`
   (`value-visit` or `type-comparison`) and the initiating owner in `phase`.
   The disposition is that of an A/H limit denial (`incomplete`; replay
   `inconclusive`/`NoValue`). The numeric comparison is not performed, so no
   operand is compared, truncated or widened and no unbounded continuation
   exists.
8. **Budgets and order.** The size checks charge no A or H unit of their own
   and the evaluator Meter nothing. The initiating event is charged first
   (QSV FR-109: poll and request one unit when ValueVisit, or the type
   comparison, is requested; only after a successful charge perform it), then
   the qualified helper runs under that one logical event (quire-exact FR-380
   Behavior 6 / AC-6). For the value path the comparison is at most two
   Integer comparisons (Integer) or four (Rational) of at most B bits. An
   over-B operand stops on that charged event: the A (or H) budget spend of
   the initiating phase INCLUDES that one event, and no separate preflight
   seam exists and the helper never runs before the event's charge. Neither
   budget resets or is refunded.

Qualification: a B check discharges the "actual finite operand bounds" premise
of QSL FR-277 AC-6 and QSV FR-109 AC-7 for Integer and Rational value-in-domain
membership once its QSV caller and reviewed typed mapping exist, and for type-link comparison once the type-link follow-up
above exists. It does not qualify Decimal, temporary storage, cancellation
response or typed mapping.

Rationale: B lives beside A and H because `limits.supplied` is the closed
caller-owned pre-call carrier that never derives from evaluation limits;
`integer_bits` is an evaluator counter whose default is `u64::MAX`, so reusing
it would inherit that default or let evaluation limits bound admission. B has
a published default because QSL FR-277 requires every enforced bound to be a
caller limits field with a published default; omission therefore normalizes
into request identity as for A and H. An over-B operand reuses the catalogued
`limit` cause with a new field name and an `operand` locator instead of a new
cause arm or code, and reports the stopping event and magnitude in the fields
the row already defines. Conversion-time numeric leaf materialization is not
bounded by B here.

### Pre-call result context

For an observed supplied-phase limit, input, cancellation, storage or capacity
stop, the affected ordered result SHALL carry exactly this closed context
in `supplied`. Existing `InternalFault` stops are outside this five-arm
context and SHALL use the separate native fault route below:

`{ phase, parameter, event, locus, consumed, cause }`.

- `phase` SHALL be `admission` or `conversion`, the actual initiating owner.
- `parameter` SHALL be the zero-based declared parameter position, or `null`
  only when genuinely unavailable. Replay arguments remain keyed by parameter
  node id; this context does not change those request keys.
- `event` SHALL be the actual owning logical event: one of `value-visit`,
  `type-comparison`, `child-scheduling`, `descriptor-query`,
  `witness-preflight`, `witness-materialization`, `converted-node` or
  `value-occurrence`. The first four use QSV FR-109's definitions/order;
  type comparison is exactly ONE compared PAIR OF chain LINKS including the
  initial pair, with no extra entry. The other events/counters use QSL
  FR-098, not new semantic ChargePoints. Cancellation outside a scheduled
  event has `event: null`, not a fictional step.
- `locus` SHALL preserve the owning producer's actual typed input/value/source
  locus, or `null` when unavailable; no fabricated empty path/span.
- `consumed` SHALL be the exact u64 successful prefix of the owning counter:
  cumulative for A/H, per argument for C/occ. It never includes a denied step.
- `cause` SHALL use one of the closed forms below. These are envelope
  distinctions, not new kernel/native catalog code spellings.

| Cause form | Required meaning and payload |
| --- | --- |
| `{ kind: limit, limit_kind, field, limit, next_charge, counter }` | Actual pre-call bound denial. `field` names the actual owning caller field; `limit_kind` is its counter, `limit` its configured u64 ceiling, `next_charge` the exact nonnegative denied amount. For A/H it is one. For `supplied.numeric_magnitude_bits` the cause also carries `operand` and no other limit cause does; the supplied-context member `consumed` (not a cause field) is 0 and `next_charge` and `counter` are the operand's bit length (see Numeric magnitude bound rule 7). `counter` is the checked attempted count, or the explicit tagged value `{overflow: true}` when counter representation overflows. No narrowing/wrapping; the denied A/H event is unperformed/unspent (an over-B numeric operand instead stops on its charged initiating event, see Numeric magnitude bound rule 8). |
| `{ kind: input, native }` | Actual invalid kind/domain/member input with original QSL InputRefusal/QSV membership cause and provenance, never a fabricated limit or failed predicate. |
| `{ kind: cancelled, native }` | Original caller Cancel cause and provenance, including Requested vs Deadline where observed; never a fresh token or work exhaustion. |
| `{ kind: storage, native, requested, unit }` | Actual allocator denial at fallible reservation before mutation. Preserve the measured request exactly and its native `bytes` or `additional-elements` unit; do not convert the amount into work units. |
| `{ kind: capacity, native }` | Actual checked representation-size/capacity failure, distinct from allocator denial and cumulative counter overflow; preserve its original operation and measured quantities where available. |

`native` SHALL be the original CLOSED typed cause/provenance from the selected
owning producer contract: its interface/stage, diagnostic catalog/code/cause
when applicable, and actual payload. It is not an untyped map, an English
message or a newly guessed kernel variant. QSL-675 FR-090/096 and QSV FR-109
own those payloads; this envelope embeds their reviewed typed form without
renaming native fields or reclassifying a declaration EnvironmentFailure/Stop
as supplied membership. An unavailable native variant/serialization mapping
blocks qualification of that dependent cause arm; it is not a new runtime
unsupported or unavailable-mapping error. This amendment supplies envelope
placement, not a private export or an invented source API.

A limit has disposition `incomplete` (replay: `inconclusive`/`NoValue`
with the phase-marked pre-call limit record); invalid supplied input retains its
existing typed refusal; actual cancellation/storage/capacity retain the
owning operation's existing typed failure classification, with this phase
context. No new blanket `failed` classification is selected for those causes.
A pre-call stop SHALL yield no partial admitted value/replay agreement and
report zero semantic evaluation consumption, with no evaluated FunctionCall
or fabricated charge point. Replay preserves QSL's inconclusive/NoValue
settlement for a pre-call limit and the distinct existing invalid/cancel/
storage/capacity boundaries. Input byte/dependency/recompile refusals retain
their existing precedence and are not wrapped as supplied-phase work.

`supplied` SHALL be absent on evaluated semantic outcomes. An evaluated
accounting `incomplete` retains EXACTLY `limit_kind, limit, consumed,
next_charge, charge_point` under `quire.value.accounting/v1`; this amendment
adds nothing to that record. A pre-call limit uses the separate envelope
context above and SHALL NOT instantiate that kernel record. No reader may
fill its mandatory charge_point with FunctionCall, omit phase provenance or
count A/H/C as evaluator work. Result identity includes the complete actual
typed disposition and supplied context when present.

### Existing pre-call fault route

An existing `ReplayRefusal::Fault(InternalFault)` detected during conversion
SHALL remain a native internal-failure result, not a sixth `supplied.cause`
arm. Its result SHALL omit `supplied` and preserve the original fault
diagnostic, stage, invariant identifier and typed provenance. Replay SHALL
retain its existing `TerminalValue::Failed` classification and unavailable
replay basis, not an ordinary InputRefusal/Inconclusive settlement. The
absence of `supplied` does not imply evaluation occurred: the original
detecting operation and stage SHALL retain its pre-call origin.

This explicitly preserves the converter's QSL-owned `replay` faults naming
`converted-payload`, `converted-slots`, `converted-elements` or
`one-converted-argument`. These retain their own identifiers, not a received
kernel CheckedInvariantCause, an input error, capacity denial or an evaluated
invariant. The receiving mapping SHALL NOT synthesize a kernel payload or
replace the detecting stage/invariant. A received typed kernel fault, when
actually present, retains its distinct original typed provenance under
QSL FR-096; no blanket conversion to a QSL-owned fault is permitted.

Fault propagation SHALL add no meter event or fault-specific helper debit,
perform no later conversion/evaluation/result retention, and return no
partial value or replay agreement. Earlier actual successful helper spend
SHALL NOT be reset/refunded; semantic evaluation consumption for a
pre-evaluation conversion fault remains zero. A genuine evaluated fault
retains its actual prior semantic charges instead. No mandatory evaluated
charge_point or FunctionCall is fabricated for the pre-call fault.
The original native typed fault serialization remains its owning reviewed
contract, not an opaque map or newly invented cause/field here. Qualification
of that binding is separate from preserving the fault route: inability to
serialize it cannot justify coercion into one of the five supplied arms.

### Qualification prerequisites

Every supported supplied path SHALL satisfy QSL FR-277 AC-6 / QSV FR-109
AC-7: authoritative helper operation, actual finite input/operand bounds,
finite work AND peak temporary storage, original-Cancel response boundary,
fallible reservation where needed and reviewed native typed mapping. Logical
event counts alone do not prove these premises. Cold/warm caches retain
identical counts/first denial for identical input/environment/nonzero incoming
budget. Checked representation overflow and reservation denial are distinct.
After removing a storage denial, retry SHALL retain the already-successful
budget prefix and original Cancel, with no reset/refund. Its fresh comparison
attempt SHALL use identical input/environment and the SAME initial nonzero
already-spent prefix. Independently enumerated retry/fresh logical sequences
SHALL agree in cumulative spend and first-denial record at one-less, and
membership result/spend at exact ceiling; membership equality alone is not
the oracle. No denied reservation/event is spent and no partial state survives.

Missing proof is an implementation qualification BLOCKER, not any fabricated
invalid/limit/storage/unsupported result. Decimal requires reviewed IR-718
SPEC capability implemented through IR-719 and actual typed integration;
the tiny [0,100], coefficient 1 at scale 0, min/max scale 2 allocation
falsifier remains required. Positive Decimal replay remains mandatory once
enabled. Future union requires the reviewed public declaration/member/helper
baseline after IR-713. The unchanged QSL-503 full 10,000-level fixture, its
29,998/29,999 occurrence and 39,996/39,997 evaluator controls remain with
QSL TC-830/831/906, not replaced or qualified by this envelope proposal.

## Outcome-to-verdict map

QSpec fixes the replay outcome-to-verdict map once per QSL ADR-013 O-16
category (FND-009; QC-8); no request instance selects or overrides a
category's verdict.

- When a replay outcome is settled, the producer SHALL apply the verdict
  fixed for the outcome's QSL ADR-013 O-16 category.
- The producer SHALL settle no agreement for an `Undefined` outcome or for
  a refused `FamilyOutcome` (its `Refused` case, QSL ADR-011 #209 FND-008
  S6a stage), regardless of the `replay` property's expected originating
  verdict.

## Identity and validation

Request identity includes every semantic input and limit. Result identity additionally includes the canonical per-item results.
A document whose `contract_version` is not `quire.native-runtime/v1`, missing required properties, duplicate keys,
out-of-domain values and non-canonical encodings refuse before consumption.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-323-AC-1 | Every selected item yields exactly one value, false/violation, refusal, undefined, incomplete or failed disposition; `undefined` is its own disposition carrying its reason and locus. Evaluator invariant failure remains `runtime_invariant`/`established-invariant-broken`; an actual constructor/input-boundary fault retains QSL-675's reviewed original fault provenance and is not relabelled a failed evaluation. Supplied input, cancellation, storage and capacity stops retain their owning typed classifications, not a newly invented invariant fault. | Analysis |
| FR-323-AC-2 | Replay uses the same evaluator and inputs rather than a generated-oracle shortcut. | Analysis |
| FR-323-AC-3 | Missing anchors, exhausted limits and false predicates remain distinct. | Analysis |
| FR-323-AC-4 | The outcome-to-verdict map assigns one verdict per QSL ADR-013 O-16 category, fixed independently of any request; an `Undefined` outcome and a refused `FamilyOutcome` never settle agreement. | Test (TC-273) |
| FR-323-AC-5 | A replay request's `byte_provision` supplies every source, definition and domain-package input the replay recompiles, each keyed by its declared digest domain and digest; the `package` reference's `RawSourceRef` digests are exactly the source and definition digests the executor recomputes and compares, and the executor reads no recompilation input by path, environment variable or search location. Before any recompile, a named input the provision does not carry refuses `missing_import`/`missing-selection` naming the requested digest record and the `package` member that names it; a raw-byte entry whose bytes do not match its digest refuses `stale_dependency`/`byte-digest-mismatch` naming the declared and the actual digest; and a `sha256-jcs` entry whose document canonicalises to another digest refuses `stale_dependency`/`content-mismatch` naming the selected and the recomputed digest. | Test (TC-277) |
| FR-323-AC-6 | A replay request's `replay` property carries the `ReplaySource`, the selected function's `QualifiedName`, arguments keyed by parameter node id, the semantic profile selections, the failing node's occurrence key, the proof bounds and declared domains, the trace position and the `backend` member; a non-replay request carries no `replay` property. | Test (TC-278) |
| FR-323-AC-7 | A replay request's `package` reference names the proved package's own `sources` and one `dependencies` entry per `dependency_selections` entry, in its order, each carrying that selection's `identity` and `package_id` and the dependency's own lock `sources`. The executor refuses by the first rule of its order: swapped or repeated entries `invalid_package`/`invalid-value` at `/package/dependencies` before any recompile; an entry whose source shares the authority and identity of another source `invalid_package`/`conflicting-definition` naming both; a removed entry `missing_import`/`missing-selection`; an entry no import reaches `invalid_package`/`invalid-value`; an entry whose sources are edited, by an edit the recompile admits, so that they recompile to a `package_id` other than the entry's, and an entry whose `package_id` alone is changed, each `stale_dependency`/`content-mismatch` naming its `identity`, its `package_id` and the recompiled one, before the proved package's `package_id` is compared. An edit removing an export the proved package calls instead refuses in the recompile, `missing_declaration`/`missing-name`. None yields a result. | Test (TC-277) |
| FR-323-AC-8 | A replay request's `stage_limits` entry raises its named stage limit for the replay recompile, a stage limit the request does not name runs at its published default, and an entry naming no stage-limit setting refuses before any stage runs. | Test (TC-426) |
| FR-323-AC-9 | A replay whose sources recompile to a proved `package_id` other than the request's `package.package_id`, after replay rules 1 to 4 pass, refuses `stale_dependency`/`content-mismatch` (QSL's `PackageIdMismatch`) naming the request's `package_id` and the recompiled one, and yields no result. | Test (TC-277) |
| FR-323-AC-10 | Applicable replay/non-replay requests carry complete effective u64 `limits.supplied` A/H ceilings and B (AC-15), independent of unchanged ScalarLimitsV1, PopulationAdmissionLimitsV1 and per-argument C/occ. Zero permits no event; either changed ceiling changes request identity. Supplied fields in `stage_limits` refuse before stages run. No absent ceiling borrows evaluator/population/stage work; a ceiling omitted at the caller API resolves to its own published default 16777216, while a wire `limits.supplied` object that is incomplete or malformed refuses before consumption. | Test (TC-278) |
| FR-323-AC-11 | A supported fixture's independently enumerated A/H event sequence denies at zero/one-less and completes at exact ceiling when other bounds fit. Its actual pre-call limit result preserves supplied phase, parameter/locus, actual event, configured ceiling, successful prefix, denied amount and checked attempted count/overflow, with zero evaluation consumption and no kernel charge_point/FunctionCall. Its evaluated counterpart retains the unchanged accounting incomplete record without supplied context. | Test (TC-278) |
| FR-323-AC-12 | Actual supplied invalid input, original caller cancellation, fallible storage denial and checked representation capacity failure preserve their distinct reviewed native typed cause/provenance; storage reports the real request in native bytes/additional-elements, not a work limit. Paired identical input/environment/nonzero-budget cold/warm runs preserve count and first denial; nested/multiple arguments neither reset nor double debit. After removal of storage denial, retry retains the successful prefix/original Cancel and matches a fresh attempt with identical input/environment and the same initial nonzero spend: independently expected cumulative spend/result at exact and first-denial record at one-less, without reset/refund or partial state. Malformed/unknown/mixed-origin context refuses interpretation, never a successful or guessed native cause. | Test (TC-278) |
| FR-323-AC-13 | Qualification of each supplied helper/cause arm identifies actual input/work/peak-storage/Cancel/fallible-route premises and reviewed typed mapping. Omitting any premise blocks qualification without creating a runtime unsupported/error result. Decimal remains a mandatory positive path once IR-718/IR-719 capability and typed integration enable it; its tiny allocation falsifier and future union baseline remain explicit prerequisites, not proof from numerical hypotheses. | Analysis (TC-278 RE-21) |
| FR-323-AC-14 | An existing typed replay conversion InternalFault supplied at its owning seam retains Failed/internal-failure classification, actual replay stage/invariant/provenance and unavailable basis, with supplied context absent, no ordinary InputRefusal/Inconclusive coercion, no synthetic kernel payload and zero semantic consumption/no added mapping charge. Compare with an ordinary input refusal and a genuine evaluated fault retaining its earlier charges. A five-arm-only serializer that drops/coerces the pre-call fault fails the control; original successful helper prefix is not reset/refunded. | Test (TC-278) |
| FR-323-AC-15 | A request's `limits.supplied.numeric_magnitude_bits` B (published default 4096; omitted resolves to it, so omitted and stated 4096 have the same request identity and 4095 or 4097 differ) admits a supplied Integer and Rational whose value and every value-in-domain operand (3 Integer, 6 Rational) have bit length at most B, with zero counted as one bit, so B = 0 admits nothing; at the default a 4096-bit operand admits and a 4097-bit operand refuses. An operand at B+1 bits refuses by the `limit` cause with field `supplied.numeric_magnitude_bits`, `limit` B, `consumed` 0, `next_charge` and `counter` its bit length, the named `operand` and `event: value-visit`, stopping on the charged initiating event (the A or H spend includes that one event) before any comparison, with no further unit charged and zero evaluation consumption. A malformed B, a B missing from the wire object, a B above the u64 range or a B in `stage_limits` is a typed input refusal before consumption. | Test (TC-278) |
| FR-323-AC-16 | NORM, not yet implementation-qualified (depends on the IR type-link follow-up; the bounded cancellable type-to-type equality helper and its QSV caller are not built): for a nested retained type link comparison, the `type-comparison` event is charged first and then every numeric endpoint (4 Integer, 8 Rational, 4 Decimal) is checked against B by the AC-15 bit-length rule; the first endpoint over B refuses by the same `limit` cause with `event: type-comparison` and a `type-link-` `operand`, before the endpoint equality runs, and an endpoint at exactly B admits. | Test (TC-278) |

## Dependencies

- [interface_005](./interface_005-native-runtime.md) owns the operation boundary.
- FR-180 owns the behavior that produces or consumes this value.
- [QSL FR-098](ix://agent-ix/quire-spec-language/FR-098) and
  [FR-277](ix://agent-ix/quire-spec-language/FR-277) own QSL-681 conversion
  nodes/occurrences, A/H units/defaults and consumer settlement.
- [QSV FR-109](ix://agent-ix/quire-semantic-value/FR-109) owns shared supplied
  logical events, initiating owner, cumulative Cancel and helper proof gates.
- QSL-675 [FR-090](ix://agent-ix/quire-spec-language/FR-090) /
  [FR-096](ix://agent-ix/quire-spec-language/FR-096) own native fault/context
  mapping. Their reviewed serialized native payload is an integration gate.
- QSL [FR-277](ix://agent-ix/quire-spec-language/FR-277) AC-6 and QSV
  FR-109 AC-7 own the operand-bound qualification premise that B discharges
  for Integer and Rational membership.
- [Accounting definition](../../../proposals/quire-v1/definitions/value-accounting.md)
  owns the unchanged scalar/population objects and closed evaluated incomplete.
- [IR-718 SPEC](https://linear.app/agent-ix/issue/IR-718/spec-quire-exact-bounded-fallible-decimal-membership-for-supplied) /
  [IR-719 implementation](https://linear.app/agent-ix/issue/IR-719/quire-exactqsv-implement-bounded-decimal-membership-and-supplied)
  own Decimal capability qualification; QSL-503 owns full-fixture integration.
- QSL FR-255 / ADR-014 owners retain setting-registry, default and B-2
  phase-projection amendments; this envelope does not approve those gates.

## Status

Accepted complete-V1 boundary value contract. Implementation and qualification
remain tracked separately.
The QSL-681 supplied-phase amendment and AC-10..16 are PROPOSED,
independently unreviewed and UNQUALIFIED. No implementation, typed native
mapping, helper proof or acceptance execution is claimed.
