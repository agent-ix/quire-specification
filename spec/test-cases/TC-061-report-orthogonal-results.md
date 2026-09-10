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

Construct healthy, violating, inactive, pending, incomplete, refused, unsupported and failed results over both closed and open assessed subjects. Include an `eventually` result settled satisfied by an admitted decisive witness while the subject remains open, an `always` result settled violated by an admitted decisive counterexample while the subject remains open, and otherwise-identical open prefixes that remain pending. Exercise every valid row of the normative result-combination table, then mutate assessment execution, subject progress/closure, truth, settlement basis and each decision-support identity independently. Delete each witness or counterexample fact, trigger, role, provider observation, population member and adverse case. Exercise every assumption, observation-adequacy and protocol-adequacy state independently, then cross-wire the two adequacy domains. Mutate every provenance identity. Submit a later observation and a later closure record that each contradict one retained settlement premise; compare the old and superseding result bytes and identities.

## Expected Results

All valid boundary combinations admit and all one-axis forbidden combinations refuse. Decisive satisfied and violated truths retain their exact support and coexist with open subject progress; open non-decisive prefixes remain pending. Removing decision support, substituting a non-decisive basis or relabelling open progress as closed refuses the settled combination. A later contradiction creates a linked superseding or invalidating result without changing the earlier bytes, and dependent package success becomes unavailable. Result states remain structurally discriminable; missing evidence never improves truth, adequacy or package status; assumption, observation-adequacy and protocol-adequacy facts remain independent; and strict readers refuse stale identities or adequacy cross-wiring while retaining independent valid results.
