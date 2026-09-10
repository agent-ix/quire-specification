---
id: TC-051
title: "Discriminate delivery, attempt and effect identity"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-051
    type: verifies
---
## Description

Verify that transport observations cannot manufacture business effects or activate effect-dependent obligations.

## Test Procedure

Bind one send, two deliveries, one payment attempt and one successful payment effect. Relabel the second delivery as another effect, remove the effect, and substitute its trace ID for each business identity.

## Expected Results

The valid case retains 1/2/1 message-delivery-effect cardinality. Relabeling or identity substitution refuses or violates exactly the affected relation; the attempt-only case registers no compensation.

