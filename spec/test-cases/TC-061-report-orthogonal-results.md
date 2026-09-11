---
id: TC-061
title: "Discriminate protocol result dimensions"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-061
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
and the canonical result bytes without updating the digest. Submit a later
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
identities, digest mutations, malformed lineage, invented independence or
adequacy cross-wiring while retaining independent valid results.
