# Observation range correspondence definition candidate

Selection identity: `quire.observation.range/v1`; revision: `1-draft.1`.
**Candidate only: not adopted, serialized, canonicalized, or implemented.**

This definition maps a producer-owned finite window to one selected native
temporal clock. It does not redefine producer membership, temporal meaning,
progress authority, closure, or runtime admission. It preserves the producer's
half-open coverage and E's selected clock meaning; a caller selects exactly one
row below rather than converting among clock families.

## Required immutable dependencies

| Dependency | Identity | Revision/version | Exact bytes or source revision |
| --- | --- | --- | --- |
| Producer interface | FCD Producer interface | `1.2.0` at `cb740adc80fdc44aaf2c6cce0553344228c18784` | `sha256:05258d455c78d6b72d1fa4c063078ac5277d4ebca627cfe965ae7fee5c31a570` |
| Event-position profile | `quire.temporal.event-position.false-extension/v1` | `1-draft.3` | `sha256:125d5f2231318279ae6737d2fa228702c7230ed23aab70c9c46b3ff28e711b95` |
| Fixed-sample profile | `quire.temporal.fixed-sample.false-extension/v1` | `1-draft.3` | `sha256:4b6c8e6346222508f3bb2b07b183725003df63d169ba72b90cd635644c7afe36` |
| Timestamped profile | `quire.temporal.timestamped-event.finite-window/v1` | `1-draft.3` | `sha256:13add5a82ac54977dba6fa104693349c849c7a08369dbfc0fccafea471012ef4` |

A range selection retains the producer interface/document identity and digest,
window identity and digest, selected temporal profile identity/revision/digest,
clock authority, ordered record-set identity/digest, and the exact coverage row.
Missing, substituted, stale, mutually inconsistent, or multiply selected values
refuse the dependent assessment. A member-object identity, a timestamp, a
record arrival order, or a digest from another domain is not a substitute.

## Selected correspondence

| Selected clock family | Producer coverage | Native range correspondence | Record and endpoint rule |
| --- | --- | --- | --- |
| `event-position` | One declared sequence authority and integer `[startInclusive, endExclusive)` positions. | Native semantic positions are exactly those integers in the same half-open range. No elapsed time or UTC timestamp is produced. | Every covered position retains its independent `(sequenceAuthority, position)` identity and its explicit record identity/valuation. Start is included once; end is excluded and belongs only to an explicitly selected adjacent range. |
| `fixed-sample` | Exact epoch, positive rational period, declared unit, and integer `[startInclusive, endExclusive)` sample indexes. | Native sample index `i` is the exact rational value `epoch + i × period` in the declared unit, for every selected index. | A sample index is distinct from a record/member identity. Each required index has an explicit selected sample record/valuation or remains incomplete; no interpolation, binary-float conversion, resampling, or event-count substitution is permitted. Start is included and end is excluded. |
| `timestamp` | RFC 3339 UTC `[startInclusive, endExclusive)` instants. | A native timestamp tick `t` is covered exactly when `startInclusive <= t < endExclusive` under the selected timestamp unit. | A record retains both record identity and timestamp-tick/anchor identity. A native inclusive deadline at `t = endExclusive` is not covered and cannot be relabelled as closure; it requires an explicitly selected coverage range that contains `t`. |

Ordered producer records may contain multiple records for one member object; each
record and each position/anchor remains distinct even when their member object
identity is equal. Conversely, object membership alone supplies no record,
position, valuation, causal order, or clock coverage. Adjacent ranges share no
covered endpoint. A record at a timestamp or position 29 can supply decisive
support independently of unrelated missing coverage; it neither supplies the
excluded endpoint nor proves surrounding execution closure.

## Admission and completeness boundary

The range is an assessment input, not a static prerequisite for parsing or
type-checking a future template. At assessment, A's linked-package admission
must validate the selected producer window, this exact range definition and the
selected clock profile before passing admitted records to E/B. A missing record,
sample, coverage document, required valuation, progress assertion, or closure
premise remains incomplete with its exact identity. A wrong family, endpoint,
authority, sequence, epoch, period, unit, record/position association, or
producer digest refuses. Neither outcome is false, an empty population, a
manufactured member, or an ambient default.

## Adoption boundary

A must index this candidate's exact digest before an observation binding or
progress definition selects it. Earlier observation definitions retain their
old producer pins and do not acquire this correspondence.
