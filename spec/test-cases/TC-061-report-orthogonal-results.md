---
id: TC-061
title: "Discriminate protocol result dimensions"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-061
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-240
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-242
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-243
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-285
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-286
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-281
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-295
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-290
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-291
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-292
    type: verifies
---
## Description

Verify the shared result contract and its monotonic response to missing evidence.

## Test Procedure

Construct healthy, violating, inactive, pending, incomplete, refused, unsupported
and failed results over both closed and open assessed subjects. Include an
`eventually` result settled satisfied by an admitted decisive witness while the
decision scope and surrounding execution remain open, an `always` result settled
violated by an admitted decisive counterexample under the same open states, and
otherwise-identical open prefixes that remain pending. Exercise every valid row
of the normative result-combination table, then mutate assessment execution,
decision-scope progress, decision-scope closure, surrounding-execution progress,
surrounding-execution closure, complete-global-conformance closure, truth,
settlement basis and each decision-support identity independently. For each
global-closure state, independently omit, duplicate, cross-wire, change scope on
and change identity on every required execution, branch and workflow premise.
Substitute each other closure axis and completed assessment execution for the
global closure record. Exercise `not-required` on every non-global claim and
refuse it on a global claim or when it carries a global premise. Delete each
witness or counterexample fact, trigger, role, provider observation, population
member and adverse case. Exercise every assumption, observation-adequacy and
protocol-adequacy state independently, then cross-wire the two adequacy domains.
Build independent, shared, derived and unknown decision-support relations;
remove each predecessor and dependence group, introduce self-reference and a
cycle, and relabel shared support independent. Mutate every provenance identity
and every canonical result member independently. For every valid result,
independently recompute `resultId` from the FR-286 preimage, permute JSON object
member presentation, mutate the identity-domain prefix, omit `resultId`, reuse
one valid identity on different canonical result bytes, and mutate the complete
canonical bytes without updating the enclosing digest. Exercise each of the
five FR-243 basis members, an unknown label and an absent basis with every
permitted and forbidden truth/execution combination.

For every exact required fact population, derive each FR-295 state; remove a
member/status, combine missing and contradicted members, substitute each
FR-284 progress/closure label, and vary all completeness and closure axes
independently. Enumerate every FR-290 protocol-family claim kind and every
Engineering Assurance FR-022 claim strength carried under FR-281; carry each
strength as its exact FR-022 wire spelling and refuse any case, separator,
whitespace or unknown variant; keep two claims with different strengths
distinct with neither promoted to the other; and confirm that key-order
adjacency between two strengths establishes no dominance. Substitute each of the four non-protocol FR-290 members and expect a
mismatch refusal; omit, duplicate and mismatch claim kind, strength, admitted
fragment and backend, including every identity/revision/digest/domain/capability
mutation.

Construct FR-285 `known` participation with an empty, singleton and multi-member
required set and with all, some and none participating. Construct `unknown`
participation with known participants, missing dependencies and typed reasons.
Independently omit, duplicate, reorder and cross-wire every role-instance
reference; mix the two variants; substitute a display name, role definition,
component identity and each E02 target participation label; and compare empty,
absent and unknown participation. Submit a later
observation and a later record from each closure axis that contradicts one
retained settlement or global-conformance premise; compare the old and
superseding result bytes and identities.

## Expected Results

All valid boundary combinations admit and all one-axis forbidden combinations
refuse. Decisive satisfied and violated truths retain their exact support and
coexist with open decision scope and surrounding execution; open non-decisive
prefixes remain pending. All five progress/closure axes remain independently
readable and scope-bound. Complete global success requires the exact `closed`
global record and its complete premise set; its mutations never change an
independently settled per-obligation truth. Removing decision support,
substituting a non-decisive basis or relabelling any open progress/closure axis
refuses the settled combination. A later contradiction creates a linked
superseding or invalidating result without changing the earlier bytes, and
dependent package success becomes unavailable. Result states remain structurally
discriminable; missing evidence never improves truth, adequacy or package status;
assumption, observation-adequacy and protocol-adequacy facts remain independent;
valid support relations and lineage remain exact; and strict readers refuse stale
identities, same-ID/different-byte results, digest mutations, malformed lineage,
invented independence or adequacy cross-wiring while retaining independent valid
results. Object-member presentation permutations preserve the independently
recomputed result identity; every semantic member mutation changes it. The five
settlement bases admit only their exact truth/execution combinations, while
unknown and absent bases refuse. Known and unknown participation remain
structurally distinct; empty is not absent or unknown, exact role-instance
identity and canonical order are preserved, and target participation labels are
never accepted as source values. Completeness is recomputed with contradiction
precedence and never supplies a progress/closure fact. Claim, strength, fragment
and backend are exact, single-valued and capability-compatible; missing,
multiple, unknown, cross-domain or display-derived selections refuse.
