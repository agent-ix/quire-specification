---
id: SR-5665
title: spec-review/base review of typed source selections
type: SpecReview
analysis: base
scope: agent-ix/quire-specification@42d81a670ffd370000e1198b9207f4dfaceb9f2d; spec/objects/interfaces/FR-330-contract-package.md,
  spec/objects/interfaces/interface_012-contract-package.md, spec/test-cases/TC-935-contract-package-profile.md,
  spec/objects/interfaces/FR-322-checked-package-artifact.md
review_set: base
---

## Summary

Ticket: QSL-706. Frozen spec-only PR 223. CONDITIONAL. Reviewed typed input ownership, exact row equality, refusal ordering, all four vector classes and unchanged lock preservation. No production implementation or runtime qualification is claimed. Optional analyses are pending the user selection.

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
- id: FR-330-AC-10
  path: spec/objects/interfaces/FR-330-contract-package.md
  role: examined
  excerpt: '| FR-330-AC-10 | The resolved profile carries authoritative typed `required_source_selections`;
    exact role/authority/identity matches admit lowering, empty requirements impose
    no restriction, and unrelated lock rows or reordered/repeated requirements do
    not change compatibility. A missing or different required row yields one request-ordered
    `invalid_input` per item before node traversal, with no extra payload, no nodes
    and exact available source occurrences; both success and refusal retain the exact
    originating lock and package id. | Test |'
- id: interface_012
  path: spec/objects/interfaces/interface_012-contract-package.md
  role: examined
  excerpt: '| checked_package | CheckedPackage | 1..1 | admitted immutable package
    |

    | lowering_profile | CompleteLoweringProfileV2 | 1..1 | explicitly selected resolved
    profile; owns `reference` and typed `required_source_selections` |

    | requested_items | RequestedItem | 0..* | each retains its own disposition |

    | limits | caller-supplied limits (FR-195) | 1..1 | bounds lowering work and output
    |


    Returns: ContractPackage and per-item accounting


    `CompleteLoweringProfileV2` is the resolved lowering-operation input owned by

    Contract IR FR-035. Its `reference: LoweringProfileRef` identifies the explicitly

    '
- id: TC-935
  path: spec/test-cases/TC-935-contract-package-profile.md
  role: examined
  excerpt: "change ContractPackage members or identity.\n\n## Typed source-selection\
    \ vectors\n\nUse these exact FR-322 `Selection` rows (the short labels below denote\
    \ typed\nvalues, never identifier-parsing rules):\n\n```json\n{\n  \"T\": {\"\
    role\": \"temporal_profile\", \"definition\": {\"authority\": \"agent-ix\", \"\
    identity\": \"quire.temporal.infinite-trace/v1\"}},\n  \"D\": {\"role\": \"temporal_profile\"\
    , \"definition\": {\"authority\": \"agent-ix\", \"identity\": \"quire.temporal.event-position.false-extension/v1\"\
    }},\n  \"U\": {\"role\": \"protocol_profile\", \"definition\": {\"authority\"\
    : \"agent-ix\", \"identity\": \"quire.protocol.finite-global/v1\"}}"
- id: TC-935
  path: spec/test-cases/TC-935-contract-package-profile.md
  role: examined
  excerpt: '| Case | `required_source_selections` | Input `lock.profile_selections`
    | Expected result |

    | --- | --- | --- | --- |

    | matching | `[T]` | `[T]` | Both requested items lower; source lock and package
    id equal that input exactly. |

    | missing | `[T]` | `[]` | Exactly `invalid_input(b)`, `invalid_input(a)`; no
    semantic nodes; both exact request occurrence sets retained. |

    | differing identity | `[T]` | `[D]` | The same two `invalid_input` records; no
    substitute `T`, no node or backend artifact; output still retains `[D]` and its
    originating package id. |

    | unrelated selection | `[T]` | `[U, T]` | '
- id: TC-935
  path: spec/test-cases/TC-935-contract-package-profile.md
  role: examined
  excerpt: '`[T, T]` and `[U, T]`/`[T, U]`: duplicate and order changes do not change

    compatibility. With requirements `[T, D]`, lock `[T]` refuses and an independently

    admitted lock containing both rows matches. Repeat a mismatch with only an

    unrelated row `[U]` to prove it cannot satisfy `T`. With requirements `[]`, the

    `[D]` input lowers without inferred requirements or lock repair.

    For a same-identity/different-authority control, require the `T` row with only

    its authority changed to `other-authority`; the original `[T]` input refuses

    rather than matching its display identity alone. In every refusal, '
- id: FR-322
  path: spec/objects/interfaces/FR-322-checked-package-artifact.md
  role: context_only
  excerpt: "- `contract_version` \u2014 exactly `quire.checked-package/v2`, the document-type\n\
    \  tag that tells a checked package apart from every other document.\n- `identity_preimage`\
    \ \u2014 exact `quire.checked-package-id/v2` JCS value whose\n  ordered members\
    \ are `edition`, `profile_selections`, `definition_selections`,\n  `model_selections`,\
    \ `required_features`, `dependency_selections` and the\n  semantic graph's `identity_projection`.\
    \ Source regions, raw artifact\n  bytes/digests, diagnostic rendering, reader\
    \ limits, runtime inputs, backend\n  availability and presentation are excluded.\n\
    - `package_id` \u2014 a `quire.pac"
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The blank line before AC-10 terminates the acceptance table, so the new criterion is absent from the computed Quire matrix. Remove the separator and confirm FR-330-AC-10 appears; current matrix contains AC-1 through AC-9 only. | spec/objects/interfaces/FR-330-contract-package.md:423-425 |
| FND-002 | medium | The compatibility pass can exhaust work, but its charge units, counter lifetime and order relative to the existing per-request first charge are undefined. FR-038 requires zero work to yield failed before lookup; the new global pre-item check leaves zero/near-limit behavior ambiguous. Specify the existing budget integration and exact-limit/one-under refusal controls. | spec/objects/interfaces/FR-330-contract-package.md:101-113 |

## Verdict

CONDITIONAL. Resolve every finding before merge. Existing CheckedSelection and CompleteLoweringProfileV2.reference reuse is grounded in the actual model source. Exact row membership preserves unrelated/reordered lock rows and source identity; the four requested vectors are explicitly present. Canonical semantic identity is the existing proof-content binding, not new file/version tracking.

## Method and limits

Base checklist and language-independent code review only. No applicable AssuranceProfile found. No new user story, mutable lifecycle, security boundary or performance target is claimed. Runnable implementation bindings are deferred by the explicitly spec-only scope. Quire matrix was inspected; authoring additional executable tests is outside this review. No build or gate was run.

## Dispositions

Round 1 reviewed `8042b09868674c46d6a694aa890091425faf1b9c`. The original findings above remain unchanged.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 8042b09868674c46d6a694aa890091425faf1b9c |
| FND-002 | fixed | 8042b09868674c46d6a694aa890091425faf1b9c |

```yaml
dispositions:
- fnd: FND-001
  outcome: fixed
  fix_sha: 8042b09868674c46d6a694aa890091425faf1b9c
  after_excerpt: '| FR-330-AC-10 | The resolved profile carries authoritative typed
    `required_source_selections`; exact role/authority/identity matches admit lowering,
    empty requirements impose no restriction, and unrelated lock rows or reordered/repeated
    requirements do not change compatibility. Per-request compatibility uses the existing
    first-charge meter and stated row-visit charges without reset; zero-work and an
    unpayable comparison charge yield `failed` before compatibility refusal. With
    sufficient work a missing or different required row yields one request-ordered
    `invalid_input` per item befo'
- fnd: FND-002
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
```

### Round 1 evidence

Both new criteria now appear in actual Quire matrices as untagged, consistent with this specification-only slice; implementation coverage is not claimed. QSpec owns request/requirement/candidate charges, no reset, exact boundary vectors and unchanged source provenance. IR invokes that contract and owns only typed adapter assertions. Whole copied vectors were removed. No source or build was changed by review.

Current disposition verdict: PASS.
