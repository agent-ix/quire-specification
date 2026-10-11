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

For N01–N10, use the v1 authority below and execute the existing cases at the
available v1 Rust boundary. N11–N13 are proposed v2 QSL adoption and
qualification procedures: execute them only against the atomically selected
v2 producer/strict-reader cohort described by FR-150. Their specification and
contract-audit bindings do not claim that this repository implements those
runtime procedures or that they have passed.

N01–N10 are historical `quire.model.complete/v1` cases, authorized only by
[`complete-model-lock.json`](../../proposals/quire-v1/definitions/complete-model-lock.json)
and the published v1 vector bytes. Their fixture selections, preimages, writer
bytes and expected identities remain unchanged; they are regression evidence
for v1, not v2 qualification. N11–N13 select the proposed
`quire.model.complete/v2` replacement closure and its v2 rule manifest under
FR-150. They are not authorized by, nor may be relabelled into, the v1 lock.
The v2 transport and strict-reader acceptance uses the complete v2 closure,
including every reachable exact key/block record; an old or mixed cohort is a
refusal, not fallback.

For N01–N10 only, select `quire.model.complete/v1` through
[`complete-model-lock.json`](../../proposals/quire-v1/definitions/complete-model-lock.json),
which also selects the `quire.value.complete/v1` lock. Each N01–N10 fixture is
a domain package with identity `test/orders`. A node written `X` has IR
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
| N04 | F1's JCS bytes stating identity `acme/other`, supplied under their SHA-256, under a ModelSelection naming that digest and identity `test/orders`; then F1 with a variant type `V1`, from artifact `V1` whose construct meaning is `quire.meaning.model.variant-type/v1`, referenced as `M::V1` in a record field of S1 | `refused { code: invalid_model_binding, cause: wrong-model-selection }` with both identities and no effective view; normalization admits `V1` as an effective type, and checking refuses `refused { code: unsupported_construct, cause: declaration-form }` at `M::V1` |
| N05 | F1's JCS bytes under a ModelSelection whose digest is their SHA-256; then with the selection's digest domain replaced by `quire-native-bytes-1`, digest bytes unchanged; then with `A/x`'s multiplicity changed and the selection unchanged | admitted with three declarations; `refused { code: stale_dependency, cause: digest-domain-mismatch }` with expected `sha256-jcs` and actual `quire-native-bytes-1`; `refused { code: stale_dependency, cause: content-mismatch }` naming the selected digest and the recomputed digest of the changed bytes; neither refusal builds a declaration |
| N06 | F2 plus fields `B/x2` and `C/x3` of type `A` and multiplicity `{0,1,false,true}`, with `B/x2.redefines = A/x` and `C/x3.redefines = A/x`; then additionally field `D/x4` of the same type and multiplicity with `D/x4.redefines = A/x` | `refused { code: invalid_model_binding, cause: derivation-conflict }` at `D` with paths `[B, B/x2, A/x]` and `[C, C/x3, A/x]`, and no effective view; admitted: `D` is a proper descendant of `B` and `C`, so only the most-derived owner's redefining member is exposed. `(D, D/x4)` is exposed with its qualify fact and one redefine fact with inputs `[D/x4, A/x]`; `(D, A/x)` is retained but hidden, with redefine facts `[B, B/x2, A/x]`, `[C, C/x3, A/x]` and `[D/x4, A/x]` in that order; `(D, B/x2)` and `(D, C/x3)` keep their inherit facts and their redefine facts (`[B, B/x2, A/x]` and `[C, C/x3, A/x]` respectively) and are hidden |
| N07 | F2 with its IR nodes supplied in reverse order | the same eight identities and view `e10820a6` as N02 |
| N08 | F1 with `A/x.presence` set to `maybe` and `B.supertypes` set to `[Z]` | after three `normalize.record` charges, two refusals in node order and no effective view: `refused { code: invalid_model_binding, cause: malformed-declaration }` naming `ix://test/orders/A/x`, member path `presence` and its `origin`; then `refused { code: missing_declaration, cause: missing-name }` naming `ix://test/orders/B`, absent node `ix://test/orders/Z` and its `origin` |
| N09 | F1 under its placeholder ModelSelection; then the N01 identities compared with the F1 selection digest and with the I04 node key of the `model_correspondence` node; then F1's nodes under the placeholder selection `n01v2` | ModelSelection `{identity: test/orders, digest_domain: sha256-jcs, digest: <SHA-256 of n01>}`, exactly the `model_selection` of the `n01-view` vector; no effective, view or universe identity equals the F1 selection digest or the I04 node key, because their domains differ; the `n01v2` run yields declarations byte-equal to `n01-view`'s, view `c478fa11` and universe `0b675aaf`, because only its ModelSelection differs |
| N10 | Each mutation in `invalid_mutations` of the vectors file applied to its base preimage | `absent-node` refuses as a stale key; `cross-domain-declaration-key` and `owner-as-declaration-key` are refused by the schema; `unsorted-derivation`, `duplicate-path` and `unsorted-view` are refused by the semantic check; none exposes its retained digest as a valid identity |
| N11 | Under the v2 selected closure, normalize a deterministic single-inheritance chain of 10,000 edges (10,001 object types), canonical package bytes ≤ 524288, and limits `declaration_records: 10001`, `derivation_facts: 50015001`, `effective_declarations: 10001`, `hashed_bytes: 268435456`, `work_units: 16777216` | Produce all 10,001 type identities, every logical qualify/inherit fact (50,015,001 facts), and the complete universe/view. Shared-prefix fact-run, cycle-check, normal-form segment/link, and range/binding-proof charges total ≤ `12 × 10001`; total exact identity hashing is bounded by 268435456 bytes and total work by 16777216 units. Emit and verify the unchanged exact v2 preimage bytes through the sole writer. Independently expand smaller ladders including shared tails/diamonds to verify exact fact order, multiplicity and ordinals. The N01–N10 v1 vector bytes remain unchanged. The proposed full-size acceptance profile is release on reference host Terra (Intel Core i9-10900K, 14 allocated threads, WSL2, 94 GiB), with peak resident memory ≤ 21474836480 bytes and monotonic wall time ≤ 300 seconds from package input admission through complete-result retention; fixture generation, process launch and compilation are excluded. These host/profile/RSS/time targets are pending owner approval under e11, are diagnostic only, and cannot close the owner-decision blocker or qualify runtime performance. For any ephemeral measuring-run report, name the app version that ran; do not persist rustc/Cargo versions or toolchain-version tracking. Do not infer duration by extrapolation or claim runtime/memory/time qualification before owner approval and measured evidence. | FR-150-AC-16, FR-150-AC-17 |
| N12 | Under the v2 selected closure, have the sole writer emit the complete retained-provenance transport for a fixture with inherited prefixes, a shared tail/diamond and a hidden redefinition; feed those exact bytes to an independent strict reader. Mutate separate copies by omitting a reachable exact key, a reachable block, an effective preimage, an eager effective identity, and a non-root effective declaration; also submit a root-only/requested-membership proof and a v1 or mixed-cohort selection. | Writer output round-trips with byte-exact preimages and all expanded facts, inputs, rules, ordinals, multiplicity, hidden declarations and origins; reader independently recomputes every eager identity and the complete ordered view before admission. Each omission, partial proof, old lock or mixed selection is refused with no admitted view and no digest relabelling/fallback. | FR-150-AC-12, FR-150-AC-13, FR-150-AC-15 |
| N13 | On a small v2 fixture, separately inject the exact limit, one unit less and zero for (a) the first `normalize.metadata` work charge on a selected semantic key-field occurrence, (b) shared-prefix/normal-form work, (c) identity-work, and (d) analytically priced preimage-hash bytes; make the next charge coincide with a dependent operation in each denied run. Include a metadata key with escaped UTF-8/JCS bytes. | Exact-bound runs complete. The one-less/zero metadata-denial runs report `normalize.metadata` as the first unavailable charge before scanning that occurrence; they perform no metadata scan, `normalize.normal-form`, identity-work, node encoding or hashing dependent on that occurrence, and expose no effective view. Every other one-less/zero run reports its first unavailable named charge deterministically, performs none of that event's work, encodes/hashes no denied node, and runs no dependent operation. For every admitted template, the analytical price equals the sole writer's returned length including escaped keys. Shared hits retain prescribed lookup charges and do not rehash. | FR-150-AC-14 |

## Expected Results

Every positive and boundary result matches FR-150; each invalid, unsupported or exhausted mutation returns its exact typed disposition with source/provenance, emits no approximation, and preserves independent sibling results.
