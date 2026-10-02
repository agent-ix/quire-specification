---
id: FR-462
title: "Speak the quire.plugin-wire/v1 plugin wire"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-305
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
---
# FR-462: Speak the quire.plugin-wire/v1 plugin wire

## Description

When the host runs a process plugin, the host and the plugin SHALL exchange
only `quire.plugin-wire/v1` frames: length-prefixed RFC 8785 JSON envelopes
carrying FR-331 manifests, requests and results.

## Inputs

The plugin executable at the path the caller names; the items routed to it;
the v2 bytes of the package; the caller's frame-length, total-byte, deadline,
grace-period, memory and CPU-time limits.

## Outputs

The plugin's FR-331 manifest and one FR-331 terminal record for each item
routed to it, or a typed tool-failure or `incomplete` settlement of each
affected item.

## Behavior

### Transport

The host starts the plugin at the path the caller names, with no discovery
and no search-path lookup. The plugin reads frames on its standard input and
writes frames on its standard output. Its standard error is captured as
opaque text up to the caller's byte limit and shown to people only; nothing
parses it.

### Framing

A frame is an unsigned 64-bit big-endian length followed by exactly that
many bytes of one RFC 8785 JSON value. A reader re-encodes each value and
refuses the frame when the bytes differ, so a frame has exactly one encoding.
The caller sets the maximum frame length and the maximum total bytes per run.

### Envelope

Every frame is one closed envelope with exactly these members:

| Member | Value |
| --- | --- |
| `protocol` | exactly `quire.plugin-wire/v1`, the wire's identity |
| `id` | an unsigned integer request id the host assigns; every reply names the id it answers |
| `kind` | exactly one of `hello`, `request`, `result`, `cancel`, `shutdown` and `fault` |
| `body` | the typed body of that kind |

A reader accepts exactly the identity `quire.plugin-wire/v1` and refuses any
other before reading the body. The bodies are:

| Kind | Sender | Body |
| --- | --- | --- |
| `hello` | plugin | the plugin's FR-331 `quire.backend-provider/v1` manifest: its advertised FR-290 (kind, mode) pairs, domains, options and bounds |
| `request` | host | an FR-331 request: the routed items, the v2 bytes of the package and the provider limits, including the deadline |
| `result` | plugin | an FR-331 result for the request whose id it names |
| `cancel` | host | the id of the request to cancel |
| `shutdown` | host | empty |
| `fault` | plugin | a typed cause with a catalog code |

### Backend identity

The plugin's first frame is `hello`. The plugin's backend identity is the
backend identity its manifest names, and each terminal record names that
identity.

### Lifecycle

For one run, each named plugin goes through: start, `hello`, registration,
zero or more `request` and `result` exchanges for the items routed to it,
`shutdown`, and exit. When the caller cancels, the host sends `cancel` with
the request id; the plugin answers with a `result` that settles each open
item FR-331 `incomplete` with the cause cancelled. When the caller's grace
period ends without that answer, the host ends the process and settles each
open item `incomplete` with the cause cancelled.

### Failures

Each of these settles every affected item FR-331 `failed` with a typed
tool-failure cause: a crash; a non-zero exit; a malformed or non-canonical
frame; a frame whose `protocol` is not `quire.plugin-wire/v1`; a frame above
the frame-length or total-byte limit; a result for an item not routed to the
plugin; a result for a capability kind the plugin did not advertise; and a
missing terminal record at `shutdown`. A deadline overrun ends the process
and settles each open item `incomplete` with cause `timed-out`. Items routed
to other providers are unaffected.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-462-AC-1 | A plugin that sends `hello` with a valid manifest, answers each `request` with one `result` and exits after `shutdown` yields one terminal record per routed item, and each record names the backend identity the manifest names. | Test (TC-428) |
| FR-462-AC-2 | A frame is accepted only as a 64-bit big-endian length followed by that many bytes of RFC 8785 JSON; a value whose bytes differ from its RFC 8785 re-encoding, and a frame whose length exceeds the frame-length or total-byte limit, settle each affected item `failed` with a tool-failure cause. | Test (TC-428) |
| FR-462-AC-3 | An envelope with a `protocol` other than `quire.plugin-wire/v1`, an unknown `kind`, a missing or extra member, or a `result` naming an id the host did not send is refused before its body is read and settles each affected item `failed`. | Test (TC-428) |
| FR-462-AC-4 | A crash, a non-zero exit, a result for an item not routed to the plugin, a result for an unadvertised capability kind and a missing terminal record each settle every affected item `failed` with a tool-failure cause, while items routed to other providers keep their own records. | Test (TC-428) |
| FR-462-AC-5 | Cancelling a run sends `cancel` with the request id; a plugin that answers settles each open item `incomplete` with cause cancelled, and a plugin that does not answer within the grace period is ended with the same settlement. A deadline overrun ends the process and settles each open item `incomplete` with cause `timed-out`. | Test (TC-428) |
| FR-462-AC-6 | The host starts the plugin only at the path the caller names, and text the plugin writes to standard error changes no settlement. | Test (TC-428) |

## Dependencies

- [FR-305](FR-305-load-explicit-rust-plugins.md) owns the host's plugin forms,
  registration and result handling.
- [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md) owns
  the manifest, request and result bodies.
- [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) owns the
  capability-kind vocabulary a manifest advertises.

## References

- Linear STD-141; QSL ADR-029 PL-2 to PL-7.
