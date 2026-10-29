# Observation range correspondence

Definition identity: `quire.observation.range/v1`.
**The definition role is accepted by FS01 and these semantics are accepted by
FS04. Executable qualification remains separate.** Local rule and dependency
links resolve within this checkout. Runtime references select each definition by
its immutable identity.

This definition maps a selected finite window to one selected native
temporal clock. It does not redefine declared population membership, temporal
meaning, progress authority, closure, or runtime admission. It preserves the
window's half-open coverage and E's selected clock meaning; a caller selects exactly one
row below rather than converting among clock families.

## Required inputs

Use the domain declarations selected by the [observation
contract](../observation-contract.md) and exactly one clock meaning from
[FR-090](../../../spec/functional/temporal/FR-090-select-temporal-profile-and-clock.md).
These are supplied assessment inputs; this definition does not duplicate their
selection tables.

A range selection retains the domain package identity, the population
declaration key, window identity, selected temporal profile identity, clock
authority, ordered record-set identity, and the exact coverage row. Missing,
substituted, mutually inconsistent, or multiply selected values refuse the
dependent assessment. A member-object identity, a timestamp, a record arrival
order, or an identity of another kind is not a substitute.

## Selected correspondence

| Selected clock family | Window coverage | Native range correspondence | Record and endpoint rule |
| --- | --- | --- | --- |
| `event-position` | One declared sequence authority and integer `[startInclusive, endExclusive)` positions. | Native semantic positions are exactly those integers in the same half-open range. No elapsed time or UTC timestamp is produced. | Every covered position retains its independent `(sequenceAuthority, position)` identity and its explicit record identity/valuation. Start is included once; end is excluded and belongs only to an explicitly selected adjacent range. |
| `fixed-sample` | Exact epoch, positive rational period, declared unit, and integer `[startInclusive, endExclusive)` sample indexes. | Native sample index `i` is the exact rational value `epoch + i × period` in the declared unit, for every selected index. | A sample index is distinct from a record/member identity. Each required index has an explicit selected sample record/valuation or remains incomplete; no interpolation, binary-float conversion, resampling, or event-count substitution is permitted. Start is included and end is excluded. |
| `timestamp` | RFC 3339 UTC `[startInclusive, endExclusive)` instants. | A native timestamp tick `t` is covered exactly when `startInclusive <= t < endExclusive` under the selected timestamp unit. | A record retains both record identity and timestamp-tick/anchor identity. A native inclusive deadline at `t = endExclusive` is not covered and cannot be relabelled as closure; it requires an explicitly selected coverage range that contains `t`. |

Ordered observation records may contain multiple records for one member object; each
record and each position/anchor remains distinct even when their member object
identity is equal. Conversely, object membership alone supplies no record,
position, valuation, causal order, or clock coverage. Adjacent ranges share no
covered endpoint. A record at a timestamp or position 29 can supply decisive
support independently of unrelated missing coverage; it neither supplies the
excluded endpoint nor proves surrounding execution closure.

## Admission and completeness boundary

The range is an assessment input, not a static prerequisite for parsing or
type-checking a future template. At assessment, the assessment binder
must validate the selected window, this exact range definition and the
selected clock profile before passing admitted records to E/B. A missing record,
sample, coverage document, required valuation, progress assertion, or closure
premise remains incomplete with its exact identity. A wrong family, endpoint,
authority, sequence, epoch, period, unit, record/position association, or
declaration key refuses. Neither outcome is false, an empty population, a
manufactured member, or an ambient default.
