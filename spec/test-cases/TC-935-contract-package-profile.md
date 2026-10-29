---
id: TC-935
title: "ContractPackage retains the explicit lowering-profile reference"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-330
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-330-AC-1
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-330-AC-5
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-330-AC-10
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322-AC-180
    type: verifies
---
# TC-935: ContractPackage retains the explicit lowering-profile reference

## Description

Verify that the explicit `package.lowering_profile` reference is retained
separately from the source selections and that caller resource limits are not
package settings.

## Test Procedure

Use an admitted checked package whose lock selects source profiles with
distinct semantic laws, and lower the same request under one explicit lowering
profile reference with two different caller resource limits high enough for
both runs to complete. For one represented request, compare the emitted typed
node with its originating FR-322 checked-package node. Inspect the closed
`{authority, identity}` reference, lock, typed graph values, and package
identity. Separately add an extra member to the profile reference by adding
`"settings": {}` inside `package.lowering_profile`, and add an extra free-standing
`settings` member at the ContractPackage root. Vary only the caller limits.

## Expected Results

Each admitted result carries exactly the explicit `package.lowering_profile`
reference in its closed `{authority, identity}` form, separately from the
source-profile selections; semantic values are those admitted by the lock and
typed graph and match the originating checked-package node, with no default or
second settings map. An extra profile-reference member is refused as
`invalid_package`/`unknown-member` at `/package/lowering_profile`; the extra
root `settings` member is refused as `invalid_package`/`unknown-member` at
`""`. A change to an included semantic value changes the graph and identity.
Changing only caller resource limits across those successful runs does not
change ContractPackage members or identity.

## Typed source-selection vectors

Use these exact FR-322 `Selection` rows (the short labels below denote typed
values, never identifier-parsing rules). The authoritative machine-readable form
is `proposals/checked-package-v2/contract-package-selection-vectors.json`
(`selections`, `compatibility_cases` with their expected results, and `repeat_controls`); a consumer reads that file and does not
copy or redefine the rows, and the QSpec test fails if this block or the table
below disagrees with it (FR-322-AC-180):

```json
{
  "T": {"role": "temporal_profile", "definition": {"authority": "agent-ix", "identity": "quire.temporal.infinite-trace/v1"}},
  "D": {"role": "temporal_profile", "definition": {"authority": "agent-ix", "identity": "quire.temporal.event-position.false-extension/v1"}},
  "U": {"role": "protocol_profile", "definition": {"authority": "agent-ix", "identity": "quire.protocol.finite-global/v1"}}
}
```

Construct independently admitted FR-322 checked packages with the selected rows
below and graph meaning valid under their own selections; do not mutate an
admitted value's lock. Retain each input's exact package value and full lock as the
independent expected output provenance. Use two distinct existing requested keys
`a`, `b` with available source occurrences, in request order `[b, a]`; both can
otherwise lower under the resolved profile's supported tags/bounds. Use resource
limits sufficient for comparison and encoding. Its explicit `reference` is
unchanged across the rows.

| Case | `required_source_selections` | Input `lock.profile_selections` | Expected result |
| --- | --- | --- | --- |
| matching | `[T]` | `[T]` | Both requested items lower; source lock and package equal that input exactly. |
| missing | `[T]` | `[]` | Exactly `invalid_input(b)`, `invalid_input(a)`; no semantic nodes; both exact request occurrence sets retained. |
| differing identity | `[T]` | `[D]` | The same two `invalid_input` records; no substitute `T`, no node or backend artifact; output still retains `[D]` and its originating package. |
| unrelated selection | `[T]` | `[U, T]` | Both items lower; output retains `[U, T]` in that order, including the unrelated row. |

Repeat the unrelated case with lock order `[T, U]` in a separately admitted
package: compatibility remains true and the output preserves that input's own
order/id, rather than equating the two package identities. Repeat with requirements
`[T, T]` and `[U, T]`/`[T, U]`: duplicate and order changes do not change
compatibility. With requirements `[T, D]`, lock `[T]` refuses and an independently
admitted lock containing both rows matches. Repeat a mismatch with only an
unrelated row `[U]` to prove it cannot satisfy `T`. With requirements `[]`, the
`[D]` input lowers without inferred requirements or lock repair.
For a same-identity/different-authority control, require the `T` row with only
its authority changed to `other-authority`; the original `[T]` input refuses
rather than matching its display identity alone. In every refusal, the serialized
record has only `node_id` and `disposition: "invalid_input"`; no fabricated
cause/pointer is added. Compare the entire emitted source lock and originating
package against the retained expected values on every successful and refused
run, not merely the row used by the predicate. These are producer controls; the
ContractPackage reader is not given or assumed to reconstruct requirements.

## Compatibility work boundaries

For the missing case `[T]` against `[]` and for one existing requested key,
compatibility's completed mismatch costs exactly two units: the first request
charge and the requirement-row visit. At `work_limit = 2`, expect `invalid_input`;
at `1`, expect `failed` for `work`, limit `1`, consumed `2`. For the differing
case `[T]` against `[D]`, the total is exactly three units: first request,
requirement visit, candidate comparison. At `3`, expect `invalid_input`; at `2`,
expect `failed`, limit `2`, consumed `3`. Repeat both with `work_limit = 0`:
expect `failed`, limit `0`, consumed `1`, even for an absent requested key.
At sufficient work an absent key is `invalid_input` immediately after the first
charge, with no comparison work or invented source region. Repeat a mismatch
with `[b, a]`: both records retain request order and the same independently
expected counters, proving there is no shared global or cached comparison budget.

For the matching `[T]` against `[T]` case, choose one existing requested root
with a finite independently walked closure and retained bytes high enough for
encoding. Let `C` be the independently counted FR-038 node/body-term/successor-edge
charges for that closure, excluding the initial request charge. The exact total
is `W = 1 + 1 + 1 + C`, including request, requirement and candidate visits.
At `work_limit = W`, expect `lowered`; at `W - 1`, expect `failed` for `work`
with limit `W - 1`, consumed `W`, no returned node and no substitute artifact.
This binds comparison and traversal to one meter: restarting at the closure
would incorrectly admit the one-under run. Independently derive `C` from the
fixture's graph/body structure rather than from a previous implementation count.
Every emitted result still preserves the entire originating lock and package;
its `failed` counters retain the existing canonical unsigned-decimal wire form.
