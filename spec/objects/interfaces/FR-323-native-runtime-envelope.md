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
  A result stopped during supplied conversion/admission carries the typed
  `supplied` context below, never an evaluated-call limit record.
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
of a released reader, selected default or qualified implementation. Its
`contract_version` remains exactly `quire.native-runtime/v1`; no compatibility
reader, private extension or alternate version is introduced. The existing
closed validation, byte-provision/dependency precedence and replay-only
members remain unchanged. The authoritative producer/consumer reader must
implement and qualify this amended shape before claiming it is supported.

### Request ceilings

For a request that supplies function arguments, the effective typed `limits`
SHALL additionally carry `supplied: { admission_work_units,
conversion_work_units }`. Both members SHALL be exact unsigned 64-bit
ceilings, with no other members in this object. Admission bounds A; conversion
bounds H when witness/input conversion occurs. A direct call over already
constructed values does not consume H merely because its ceiling is carried.
Requests with no supplied-value phase retain their existing limit objects
and may omit `supplied`; a supplied-value request lacking an effective
applicable ceiling SHALL NOT be admitted with an unbounded or evaluator-derived
fallback. Zero admits zero logical units.

QSL FR-277 owns A/H fields, units and published default selection. Caller API
omission SHALL be resolved only from each field's own reviewed published
default into this complete effective wire object before consumption and
request identity calculation. Existing ScalarLimitsV1 and population
omission/default rules SHALL NOT change. This proposal selects NO A/H
default: 262,144/524,288 are unselected hypotheses. Until owning defaults
are approved, default-request qualification is blocked; explicit effective
ceilings do not qualify an unreviewed helper. No numeric fallback is inferred.

`stage_limits` SHALL continue to carry only FR-460/461 recompile stage settings.
Neither supplied field is a stage setting; placing it there SHALL refuse
under the existing unknown-stage-setting rule before stages run. Changing
either effective supplied ceiling changes request identity, not package
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

### Pre-call result context

For an observed supplied-phase stop, the affected ordered result SHALL carry
exactly this closed context in `supplied`:

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
| `{ kind: limit, limit_kind, field, limit, next_charge, counter }` | Actual pre-call bound denial. `field` names the actual owning caller field; `limit_kind` is its counter, `limit` its configured u64 ceiling, `next_charge` the exact nonnegative denied amount. For A/H it is one. `counter` is the checked attempted count, or the explicit tagged value `{overflow: true}` when counter representation overflows. No narrowing/wrapping; the denied event is unperformed/unspent. |
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

A limit has disposition `incomplete`; invalid supplied input retains its
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

### Qualification prerequisites

Every supported supplied path SHALL satisfy QSL FR-277 AC-6 / QSV FR-109
AC-7: authoritative helper operation, actual finite input/operand bounds,
finite work AND peak temporary storage, original-Cancel response boundary,
fallible reservation where needed and reviewed native typed mapping. Logical
event counts alone do not prove these premises. Cold/warm caches retain
identical counts/first denial for identical input/environment/nonzero incoming
budget. Checked representation overflow and reservation denial are distinct;
retry after removed denial preserves membership rules.

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
| FR-323-AC-10 | Applicable replay/non-replay requests carry complete effective u64 `limits.supplied` A/H ceilings, independent of unchanged ScalarLimitsV1, PopulationAdmissionLimitsV1 and per-argument C/occ. Zero permits no event; either changed ceiling changes request identity. Supplied fields in `stage_limits` refuse before stages run. No absent ceiling borrows evaluator/population/stage work or an unselected default; default-request qualification waits for the owning published defaults. | Test |
| FR-323-AC-11 | A supported fixture's independently enumerated A/H event sequence denies at zero/one-less and completes at exact ceiling when other bounds fit. Its actual pre-call limit result preserves supplied phase, parameter/locus, actual event, configured ceiling, successful prefix, denied amount and checked attempted count/overflow, with zero evaluation consumption and no kernel charge_point/FunctionCall. Its evaluated counterpart retains the unchanged accounting incomplete record without supplied context. | Test |
| FR-323-AC-12 | Actual supplied invalid input, original caller cancellation, fallible storage denial and checked representation capacity failure preserve their distinct reviewed native typed cause/provenance; storage reports the real request in native bytes/additional-elements, not a work limit. Paired identical input/environment/nonzero-budget cold/warm runs preserve count and first denial; nested/multiple arguments neither reset nor double debit; retry after removal of storage denial matches fresh membership. Malformed/unknown/mixed-origin context refuses interpretation, never a successful or guessed native cause. | Test |
| FR-323-AC-13 | Qualification of each supplied helper/cause arm identifies actual input/work/peak-storage/Cancel/fallible-route premises and reviewed typed mapping. Omitting any premise blocks qualification without creating a runtime unsupported/error result. Decimal remains a mandatory positive path once IR-718/IR-719 capability and typed integration enable it; its tiny allocation falsifier and future union baseline remain explicit prerequisites, not proof from numerical hypotheses. | Analysis |

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
The QSL-681 supplied-phase amendment and AC-10..13 are PROPOSED,
independently unreviewed and UNQUALIFIED. No implementation, new default,
typed native mapping, helper proof or acceptance execution is claimed.
