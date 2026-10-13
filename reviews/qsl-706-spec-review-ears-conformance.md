---
id: SR-5666
title: ears-conformance review of typed source selections
type: SpecReview
analysis: ears-conformance
scope: agent-ix/quire-specification@42d81a670ffd370000e1198b9207f4dfaceb9f2d; spec/objects/interfaces/FR-330-contract-package.md
review_set: subset
---

## Summary

Ticket: QSL-706. Original frozen candidate, before any fix. CONDITIONAL. Manual statement review supplements the engine check. The engine emitted no warnings for the changed FR; manual review found the wording defects below.

## Examined scope

```yaml
scope:
- id: FR-330
  path: spec/objects/interfaces/FR-330-contract-package.md
  role: examined
  excerpt: "  `PackageId` of the admitted originating `quire.checked-package/v2` value,\n\
    \  including its `quire.package.semantic/v2` domain and SHA-256 algorithm.\n \
    \ `source.lock` is that value's exact FR-322 lock, with the same members,\n  selected\
    \ identities, and ordering. It is provenance, not a second package\n  or authority;\
    \ the producer SHALL NOT reconstruct, normalize, or silently\n  amend it. `lowering_profile`\
    \ is the explicit I12 `LoweringProfileRef`\n  supplied as the resolved profile's\
    \ `reference` to `lower`; it has the same\n  closed `{authority, identity}` value\
    \ form as FR-322 `DefinitionRef`. This\n"
- id: FR-330
  path: spec/objects/interfaces/FR-330-contract-package.md
  role: examined
  excerpt: 'profile''s capability settings are not added to the ContractPackage.

    `package.lowering_profile` records only the explicit I12 profile reference; it

    does not amend the lock''s selected source meaning. Compatibility is checked

    against the resolved profile''s typed requirements, never the reference alone.


    ### Typed source-selection compatibility


    The I12 lowering input is the resolved Contract IR FR-035

    `CompleteLoweringProfileV2`, whose `required_source_selections` field is a

    `Vec<CheckedSelection>` of exact FR-322 `Selection` rows. The owner of those

    requirements is the producer of that resolved '
- id: FR-330
  path: spec/objects/interfaces/FR-330-contract-package.md
  role: examined
  excerpt: '

    For each required row, the lowerer SHALL require a row in the admitted input''s

    `lock.profile_selections` with equal `role`, `definition.authority`, and

    `definition.identity`. Equality is exact enum/string equality, including case;

    there is no alias, version, prefix, subtype, or digest comparison. The predicate

    is conjunction of exact row membership, not array equality or lookup by role

    alone. Extra lock rows, including other definitions of the same role, do not

    invalidate a match. Repeated identical requirements are idempotent; requirement

    order and lock order do not change the predicate. Dif'
- id: FR-330
  path: spec/objects/interfaces/FR-330-contract-package.md
  role: examined
  excerpt: 'After checked-input admission and before any per-item semantic-node traversal
    or

    emission, the lowerer SHALL apply that predicate. If a required row is absent,

    including when only a different authority or identity occurs for its role, every

    requested item receives the existing `invalid_input` disposition with its exact

    requested `node_id` and no additional payload. Records retain caller request

    order and their available source occurrences under the existing source-map

    rules; an absent requested key retains an empty occurrence array. No semantic

    node or backend artifact is emitted on this path.'
- id: FR-330
  path: spec/objects/interfaces/FR-330-contract-package.md
  role: examined
  excerpt: 'On either match or mismatch, the emitted refusal or represented package
    SHALL

    retain the originating `package.source.package_id` and exact `source.lock`,

    including every row and its order, without sorting, filtering, adding or

    substituting selections. The resolved requirements are not serialized into

    `package.lowering_profile` or another ContractPackage member. A strict reader

    cannot re-check compatibility from the emitted reference alone; this is the

    lowerer''s producer obligation, verified against its typed operation input.'
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The new compatibility requirement starts with After rather than the EARS event trigger When. Express admission completion as a named event while retaining pre-traversal ordering. | spec/objects/interfaces/FR-330-contract-package.md:101-102 |
| FND-002 | low | The preservation requirement starts with On either match or mismatch instead of an EARS event or state trigger. State preservation as an unconditional lowerer obligation or a When event. | spec/objects/interfaces/FR-330-contract-package.md:115-118 |

## Engine and semantic check

QSpec original full-spec validation exited 3 with errors outside the changed documents; 883/887 documents grammar-clean, 9 grammar findings. No engine grammar diagnostic names FR-330, I12 or TC-935. Four newly authored SHALL statements examined: two clean, two manual trigger findings. Event/state intent and concrete observable responses otherwise agree.

## Verdict

CONDITIONAL. Resolve the recorded findings; this artifact reviews only the original head. Fix disposition belongs in an appended section. No build or runtime qualification was performed.

## Dispositions

Round 1 reviewed `8042b09868674c46d6a694aa890091425faf1b9c`. The original findings above remain unchanged.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 8042b09868674c46d6a694aa890091425faf1b9c |
| FND-002 | still-open | At reviewed 8042b098, FR-330 still says "On either match or mismatch, the emitted refusal or represented package SHALL retain". This original low wording finding remains; use an unconditional preservation requirement or canonical event trigger. |

```yaml
dispositions:
- fnd: FND-001
  outcome: fixed
  fix_sha: 8042b09868674c46d6a694aa890091425faf1b9c
  after_excerpt: 'For each requested item, the lowerer SHALL use that request''s existing
    FR-038

    work meter and `CompleteLoweringProfileV2.work_limit`. Its counter starts once

    at zero for that request. The FR-038 per-request work unit is charged first;

    if it exceeds the ceiling the record is `failed` for `work`, with the requested

    `node_id`, caller''s limit and consumed counter at that failed charge. Thus zero

    work yields `failed` with consumed `1` before key lookup or compatibility.

    An absent requested key still returns `invalid_input` after that first charge,

    without running compatibility, as FR-038 r'
- fnd: FND-002
  outcome: still-open
  reason: At reviewed 8042b098, FR-330 still says "On either match or mismatch, the
    emitted refusal or represented package SHALL retain". This original low wording
    finding remains; use an unconditional preservation requirement or canonical event
    trigger.
```

### Round 1 evidence

Both new criteria now appear in actual Quire matrices as untagged, consistent with this specification-only slice; implementation coverage is not claimed. QSpec owns request/requirement/candidate charges, no reset, exact boundary vectors and unchanged source provenance. IR invokes that contract and owns only typed adapter assertions. Whole copied vectors were removed. No source or build was changed by review.

Current disposition verdict: CONDITIONAL.

### Round 2

Reviewed `5f41aec6bdf96a19814e763524af1f1c46c99c1e`. Only the requested wording hunk changed; the prior matrix repair, charged predicate, work vectors and single normative ownership remain intact. No new finding or regression.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-002 | fixed | 5f41aec6bdf96a19814e763524af1f1c46c99c1e |

```yaml
dispositions:
- fnd: FND-002
  outcome: fixed
  fix_sha: 5f41aec6bdf96a19814e763524af1f1c46c99c1e
  after_excerpt: 'The lowerer SHALL retain the originating `package.source.package_id`
    and exact `source.lock`,

    including every row and its order, without sorting, filtering, adding or

    substituting selections.'
```

Current disposition verdict: PASS. All findings for this method are resolved.
