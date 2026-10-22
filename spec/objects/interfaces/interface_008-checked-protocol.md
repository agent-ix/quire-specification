---
id: interface_008
title: "I08 checked protocol artifact"
type: interface
relationships:
  - target: ix://agent-ix/quire-specification/FR-326
    type: references
---

# [interface_008] I08 checked protocol artifact

## Features

| Feature | Kind |
| --- | --- |
| check_protocol | operation |

## Contract

```yaml
name: CheckedProtocol
associated_types: [ProtocolId, RoleInstance, Message, ChannelQueue, ControlGraph, Obligation]
operations:
  - { name: check_protocol, inputs: [CheckedPackage, protocol declaration], output: immutable protocol graph or located refusal, semantics: validates static and dynamic topology control guards queues and recovery }
invariants:
  - message delivery attempt effect and compensation identities are distinct
  - no business action executes during checking
```
