---
id: TC-195
title: "Model normalization provenance"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-150
    type: verifies
---

# TC-195: Model normalization provenance

## Description

Normalize inherited/redefined declarations and mutate identity, digest and rule-order inputs.

## Test Procedure

Use the subject and inputs named by FR-150. Execute its positive case, each declared boundary, and each explicit refusal/non-conclusive mutation through the real local Rust boundary; compare the complete typed result and artifact accounting with the criterion.

Select `quire.model.complete/v1` through
[`complete-model-lock.json`](../../proposals/quire-v1/definitions/complete-model-lock.json),
which also selects the `quire.value.complete/v1` lock. Every fixture is a domain
package with identity `test/orders`. A node written `X` has IR
node identity `ix://test/orders/X`, and its declaration key is
`{package: test/orders, node: ix://test/orders/X, digest_domain: sha256-jcs}`.
Object type `A` is declared by the artifact with id `A` and title `Alpha`, `B`
by id `B` titled `Beta`, `C` by id `C` titled `Gamma` and `D` by id `D` titled
`Delta`; each type definition's construct meaning is
`quire.meaning.model.object-type/v1`, and each field's span is its Properties
row. Rows N04 and N05 admit package bytes: their ModelSelection digest is the
SHA-256 of those JCS bytes, and the bytes are supplied under that digest. Every
other row enters at the normalization boundary with a fixed placeholder
ModelSelection, not the digest of real package bytes: F1 selects the SHA-256 of
the UTF-8 bytes `n01`, F2 of `n02`, and F1 republished under a second digest of `n01v2`. The
published vectors use these placeholder selections.
Effective identities are written as the first eight hex digits of the full
digests in
[`model-effective-declaration-vectors.json`](../../proposals/checked-package-v2/model-effective-declaration-vectors.json).
Unless a row states `ModelNormalizationLimitsV1`, run it under limits large
enough that no charge is denied.

Fixtures:

- **F1**: object types `A` and `B`, with `B.supertypes = [A]`; field `A/x` of
  `A`, typed `A`, presence `optional`, multiplicity
  `{lower: 0, upper: 1, ordered: false, unique: true}`. Three IR nodes.
- **F2**: F1's nodes plus object types `C` with `supertypes = [A]` and `D`
  with `supertypes = [B, C]`. Five IR nodes.
- **Source S1**: `language "ix:native" edition "1-draft";`, one profile with
  alias `V`, `model M = "test/orders" digest "<F2 digest>";`.

| Vector | Input | Expected result/disposition |
| --- | --- | --- |
| N01 | Normalize F1; then under `ModelNormalizationLimitsV1 { declaration_records: 3, derivation_facts: 5, effective_declarations: 4, dispatch_candidates: 0, hashed_bytes: 5030, work_units: 19 }`; then with `work_units: 18`; then with `hashed_bytes: 5029`; then with `derivation_facts: 4` and every other counter as in the exact run | effective types `A` = `23664417` (one qualify fact) and `B` = `ca6ff932` (qualify fact, then inherit fact with inputs `[A]`), effective members `(A, A/x)` = `3729a6fa` and `(B, A/x)` = `42cf1c2c` (one inherit fact with inputs `[A, A/x]`), effective view `ce039e96` and object universe `dae94514` with root types `[23664417]`; charges, in order, are three `normalize.record` (`A`, `A/x`, `B`); five `normalize.fact` (phase 2 qualify `A`, `B`, `(A, A/x)`; phase 3 `normalize.cycle-check` with `L = 1` then inherit `B`; inherit `(B, A/x)`); four `normalize.declaration`, each followed by its `normalize.hash` (JCS lengths 352, 517, 477 and 560); then `normalize.hash` of the universe (335) and of the view (2789): nineteen work units and 5030 hashed bytes; the exact run completes identically; `incomplete { limit_kind: work_units, limit: 18, consumed: 18, next_charge: 1, charge_point: normalize.hash }` at the view hash; `incomplete { limit_kind: hashed_bytes, limit: 5029, consumed: 2241, next_charge: 2789, charge_point: normalize.hash }` at the view hash; `incomplete { limit_kind: derivation_facts, limit: 4, consumed: 4, next_charge: 5, charge_point: normalize.fact }` at the fact for `(B, A/x)`; neither incomplete run exposes an effective view, and the checker reports it as `resource_exhausted` with cause `insufficient-next-charge` at stage `model-normalization` |
| N02 | Normalize F2 | eight effective declarations: types `A` = `23664417`, `B` = `ca6ff932`, `C` = `b23df80c`, `D` = `9fb83dd4` (qualify plus four inherit facts with inputs `[B]`, `[B, A]`, `[C]`, `[C, A]` in that order); members `(A, A/x)` = `3729a6fa`, `(B, A/x)` = `42cf1c2c`, `(C, A/x)` = `ea039070` and exactly one `(D, A/x)` = `36d8ed69` retaining both paths `[B, A, A/x]` and `[C, A, A/x]`; view `e10820a6`; universe `c45b5094`; five records, fifteen facts (five qualify, ten inherit), six `normalize.cycle-check` charges before the six phase 3 type facts (`L` = 1 for `B`, 1 for `C`, then 1, 2, 1 and 2 for `D`'s four paths: eight work units), eight declarations and ten hashes (declarations 5137 bytes, universe 335, view 6640): forty-six work units and 12112 hashed bytes. `A` and `(A, A/x)` equal their F1 identities because an effective declaration identity binds no ModelSelection, while the views and universes differ |
| N03 | F2 plus fields `B/y` and `C/y`, both typed `A` with multiplicity `{0,1,false,true}`; then S1 plus `invariant I using V on M::D at current { present(self.y) }` | normalization admits twelve effective declarations, N02's eight plus `(B, B/y)`, `(C, C/y)` and the distinct, equal-shaped `(D, B/y)` and `(D, C/y)`; checking the invariant is `refused { code: ambiguous_declaration, cause: ambiguous-name }` at `self.y` listing both effective members and loci |
| N04 | F1's JCS bytes stating identity `acme/other`, supplied under their SHA-256, under a ModelSelection naming that digest and identity `test/orders`; then F1 with a variant type `V1`, from artifact `V1` whose construct meaning is `quire.meaning.model.variant-type/v1`, referenced as `M::V1` in a record field of S1 | `refused { code: invalid_model_binding, cause: wrong-model-selection }` with both identities and no effective view; normalization admits `V1` as an effective type, and checking admits `M::V1` in the record field (FR-150, FR-208-AC-15) |
| N05 | F1's JCS bytes under a ModelSelection whose digest is their SHA-256; then with the selection's digest domain replaced by `quire-native-bytes-1`, digest bytes unchanged; then with `A/x`'s multiplicity changed and the selection unchanged | admitted with three declarations; `refused { code: stale_dependency, cause: digest-domain-mismatch }` with expected `sha256-jcs` and actual `quire-native-bytes-1`; `refused { code: stale_dependency, cause: content-mismatch }` naming the selected digest and the recomputed digest of the changed bytes; neither refusal builds a declaration |
| N06 | F2 plus fields `B/x2` and `C/x3` of type `A` and multiplicity `{0,1,false,true}`, with `B/x2.redefines = A/x` and `C/x3.redefines = A/x`; then additionally field `D/x4` of the same type and multiplicity with `D/x4.redefines = A/x` | `refused { code: invalid_model_binding, cause: derivation-conflict }` at `D` with paths `[B, B/x2, A/x]` and `[C, C/x3, A/x]`, and no effective view; admitted: `D` is a proper descendant of `B` and `C`, so only the most-derived owner's redefining member is exposed. `(D, D/x4)` is exposed with its qualify fact and one redefine fact with inputs `[D/x4, A/x]`; `(D, A/x)` is retained but hidden, with redefine facts `[B, B/x2, A/x]`, `[C, C/x3, A/x]` and `[D/x4, A/x]` in that order; `(D, B/x2)` and `(D, C/x3)` keep their inherit facts and their redefine facts (`[B, B/x2, A/x]` and `[C, C/x3, A/x]` respectively) and are hidden |
| N07 | F2 with its IR nodes supplied in reverse order | the same eight identities and view `e10820a6` as N02 |
| N08 | F1 with `A/x.presence` set to `maybe` and `B.supertypes` set to `[Z]` | after three `normalize.record` charges, two refusals in node order and no effective view: `refused { code: invalid_model_binding, cause: malformed-declaration }` naming `ix://test/orders/A/x`, member path `presence` and its `origin`; then `refused { code: missing_declaration, cause: missing-name }` naming `ix://test/orders/B`, absent node `ix://test/orders/Z` and its `origin` |
| N09 | F1 under its placeholder ModelSelection; then the N01 identities compared with the F1 selection digest and with the I04 node key of the `model_correspondence` node; then F1's nodes under the placeholder selection `n01v2` | ModelSelection `{identity: test/orders, digest_domain: sha256-jcs, digest: <SHA-256 of n01>}`, exactly the `model_selection` of the `n01-view` vector; no effective, view or universe identity equals the F1 selection digest or the I04 node key, because their domains differ; the `n01v2` run yields declarations byte-equal to `n01-view`'s, view `c478fa11` and universe `0b675aaf`, because only its ModelSelection differs |
| N10 | Each mutation in `invalid_mutations` of the vectors file applied to its base preimage | `absent-node` refuses as a stale key; `cross-domain-declaration-key` and `owner-as-declaration-key` are refused by the schema; `unsorted-derivation`, `duplicate-path` and `unsorted-view` are refused by the semantic check; none exposes its retained digest as a valid identity |
| N14 | F1 plus a variant type `V1`, from artifact `V1` whose construct meaning is `quire.meaning.model.variant-type/v1`, with cases `Gold` and `Silver` whose case nodes are `ix://test/orders/variant/V1-Gold` and `ix://test/orders/variant/V1-Silver`; then the same nodes with the `variants` list reversed; then also a second variant type `V2` with one case `Bronze` and `V1.supertypes = [V2]`; then `V1.supertypes = [A]`, then `V1.supertypes = [Z]` (absent), then `V1.supertypes = [V2]` with `V2.supertypes = [V1]` | `V1` is one effective type with owner `null` and original its declaration key, carrying exactly one qualify fact with inputs `[V1]` and no inherit fact; `Gold` and `Silver` are effective members owned by `V1`, each with original its case node's own key and exactly one qualify fact; `V1` has no object universe; the reversed run yields the same effective identities and the same effective view; with `V1.supertypes = [V2]` the `supertypes` list is ignored, so `V1`, `Gold` and `Silver` keep the identities of the first run, `V1` has no inherit fact, and `V2` and `Bronze` are effective declarations of their own; the lists naming an object type and an absent node are each `refused { code: unsupported_construct, cause: declaration-form }` at `supertypes`, and the cycle run is two such refusals, one at `V1` and one at `V2`, one per node on the cycle |
| N15 | F1 plus record value types `R` (construct meaning `quire.meaning.model.record-value-type/v1`) with field `R/f` typed `A`, optional, multiplicity `{0,1,false,true}`, and `R2` with `R2.supertypes = [R]` and no field of its own | `R` is an effective type with owner `null` and one qualify fact; `R2` has a qualify fact then an inherit fact with inputs `[R]`; `(R, R/f)` has one qualify fact; `(R2, R/f)` has one inherit fact with inputs `[R, R/f]` and no qualify fact, the derivation N01 specifies for `(B, A/x)`; neither record has an object universe |
| N16 | F1 plus event types `E` (construct meaning `quire.meaning.model.event-type/v1`) with field `E/at` of native type `Timestamp` and `occurrenceField = E/at`, and `E2` with `E2.supertypes = [E]` and `occurrenceField = E/at`; neither declares an identity field | `E` is an effective type with owner `null` and one qualify fact; `E2` has a qualify fact then an inherit fact with inputs `[E]`; `(E, E/at)` has one qualify fact; `(E2, E/at)` has one inherit fact with inputs `[E, E/at]` and no qualify fact, the derivation N01 specifies; neither event type has an object universe |
| N17 | F1 plus a state machine `S` (construct meaning `quire.meaning.model.state-machine/v1`) with operation `S/op`, states `s1` and `s2` and a transition `s1 -> s2` triggered by `S/op`; a state machine `S2` with `S2.supertypes = [S]`, its own operation `S2/op`, state `s3` and transition `s1 -> s3` triggered by `S2/op` (a state name reached from two owners, or two transitions with equal `from`, `to` and `trigger`, is a derivation conflict, so `S2` reuses no state name of `S` and its transition differs in `to` and `trigger`); a process `P` (construct meaning `quire.meaning.model.process/v1`) with `identityFields = [P/id]`, field `P/id` and one step of kind `command`; a persistence interface `PI` (construct meaning `quire.meaning.model.persistence-interface/v1`) with operation `PI/op` and `persists = [A]` | `S`, `P` and `PI` are effective types with owner `null` and one qualify fact each; `S2` has a qualify fact then an inherit fact with inputs `[S]`; `P` has an object universe of runs and `S`, `S2` and `PI` have none; `(S, S/op)` and `(PI, PI/op)` have one qualify fact each, `(S2, S/op)` has one inherit fact with inputs `[S, S/op]` and no qualify fact, the N01 derivation; `PI` has no inherit fact; states, transitions and steps are not effective members |
| N18 | F1 plus a `quire` invariant clause `A/inv` on `A` and a clause in another language on `A`; then also operation `A/op` of `A` with a `Pre:` and a `Post:` contract clause | `(A, A/inv)` has one qualify fact and `(B, A/inv)` one inherit fact with inputs `[A, A/inv]` and no qualify fact, as N01 specifies for fields; the other-language clause stays an original in the retained inventory with no qualify fact or effective entry; the contract clauses are not declarations: none is retained or qualified, and `A/op` is the only added declaration |

## Expected Results

Every positive and boundary result matches FR-150; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
