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
the checked package's v2 bytes; the caller's
frame-length, total-byte, deadline, grace-period, memory and CPU-time limits.

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
| `hello` | plugin | the FR-331 `quire.backend-provider/v1` manifest stage, whose one descriptor is the plugin's: its advertised FR-290 (kind, mode) pairs, domains, options and bounds |
| `request` | host | the FR-331 request stage: the routed items; the checked package as `package`, the unpadded base64url spelling of its RFC 8785 v2 bytes; and the provider limits, including `deadline_ms` |
| `result` | plugin | the FR-331 outcome stage for the request whose id it names |
| `cancel` | host | the id of the request to cancel |
| `shutdown` | host | empty |
| `fault` | plugin | a typed cause with a catalog code |

Each body is one stage of the FR-331 wire schema
([`proposals/backend-provider-v1/schema.json`](../../../proposals/backend-provider-v1/schema.json)).
The bytes travel as a string because a JSON frame cannot hold raw bytes, and
unpadded base64url with zero trailing bits gives each byte string one
spelling, so the frame keeps its one RFC 8785 encoding. The plugin decodes
`package`, admits it with the v2 reader before any item runs. A plugin that refuses a `request` body answers it with one
`result` that settles every routed item FR-331 `failed` with the refusal as
its `diagnostic`: `invalid_runtime_input`/`invalid-value` for a `package`
that is not the canonical spelling. No item runs.

### Backend identity

The plugin's first frame is `hello`. The plugin's backend identity is the
backend identity its manifest names, and each terminal record names that
identity.

A `hello` whose manifest advertises an absent or unknown kind, an unknown
mode, `probabilistic-satisfaction` with no or unknown evidence kinds, a
`bounded` mode with no `domains`, or, whatever modes it advertises, a
`domains` that is empty, holds a non-boundable or unknown kind or repeats a
kind is a registration that FR-290
refuses, with `invalid_capability`/`absent-kind`, `unknown-kind`,
`unknown-mode`, `unknown-evidence-kind` or `invalid-domains` keyed by the
backend identity. It is not a wire failure: the plugin is not registered, no item is
routed to it, a request naming its identity receives FR-290's unknown-backend
mark, and the host sends `shutdown`.

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

The tool-failure cause is the FR-331 cause `tool-failure{defect}`, with
`defect` naming the failure. The two byte-limit defects also carry `limit`,
the caller's configured maximum in bytes, and `value`, the byte count that
crossed it: the frame's length for `frame-length` and the run's running
total for `total-bytes`. No other defect carries them.

| Failure | `defect` |
| --- | --- |
| crash | `crash` |
| non-zero exit | `nonzero-exit` |
| frame that is not a length and that many bytes of JSON | `malformed-frame` |
| value whose bytes differ from its RFC 8785 re-encoding | `noncanonical-frame` |
| frame above the frame-length limit | `frame-length`, with `limit` and `value` |
| frames above the total-byte limit | `total-bytes`, with `limit` and `value` |
| `protocol` other than `quire.plugin-wire/v1` | `unknown-protocol` |
| unknown `kind` | `unknown-frame-kind` |
| missing or extra envelope member | `envelope-member` |
| `result` naming an id the host did not send | `unknown-request-id` |
| `request` or `result` body that its FR-331 stage schema refuses, or a `hello` body it refuses for any defect other than an FR-290 registration cause | `malformed-body` |
| result for an item not routed to the plugin | `unrouted-item` |
| result for a capability kind the plugin did not advertise | `unadvertised-kind` |
| missing terminal record at `shutdown` | `missing-terminal-record` |

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-462-AC-1 | A plugin that sends `hello` with a valid manifest, answers each `request` with one `result` and exits after `shutdown` yields one terminal record per routed item, and each record names the backend identity the manifest names. | Test (TC-428) |
| FR-462-AC-2 | A frame is accepted only as a 64-bit big-endian length followed by that many bytes of RFC 8785 JSON; a value whose bytes differ from its RFC 8785 re-encoding, and a frame whose length exceeds the frame-length or total-byte limit, settle each affected item `failed` with a tool-failure cause. | Test (TC-428) |
| FR-462-AC-3 | An envelope with a `protocol` other than `quire.plugin-wire/v1`, an unknown `kind`, a missing or extra member, or a `result` naming an id the host did not send is refused before its body is read and settles each affected item `failed`. | Test (TC-428) |
| FR-462-AC-4 | A crash, a non-zero exit, a result for an item not routed to the plugin, a result for an unadvertised capability kind and a missing terminal record each settle every affected item `failed` with a tool-failure cause, while items routed to other providers keep their own records. | Test (TC-428) |
| FR-462-AC-5 | Cancelling a run sends `cancel` with the request id; a plugin that answers settles each open item `incomplete` with cause cancelled, and a plugin that does not answer within the grace period is ended with the same settlement. A deadline overrun ends the process and settles each open item `incomplete` with cause `timed-out`. | Test (TC-428) |
| FR-462-AC-6 | The host starts the plugin only at the path the caller names, and text the plugin writes to standard error changes no settlement. | Test (TC-428) |
| FR-462-AC-7 | A `request` body carries `package`, the unpadded base64url spelling of the package's RFC 8785 v2 bytes, which decodes to exactly those bytes; a plugin given a body whose `package` is padded, uses the standard alphabet or has nonzero trailing bits, runs no item and settles every routed item `failed` with the refusal as its diagnostic. | Test (TC-428) |
| FR-462-AC-8 | Each wire failure of the failure table settles every affected item `failed` with cause `tool-failure` and that row's `defect`. | Test (TC-428) |
| FR-462-AC-9 | A `hello` whose only defects are FR-290 registration causes is refused as that registration, keyed by backend identity, and settles no item `failed`; a request naming that identity receives the unknown-backend mark. | Test (TC-428) |
| FR-462-AC-10 | A frame one byte over a frame-length limit of 1048576 settles each affected item `failed` with `{"type": "tool-failure", "defect": "frame-length", "limit": 1048576, "value": 1048577}`, and frames whose running total first exceeds a total-byte limit of 4194304 at 4194305 bytes settle `{"type": "tool-failure", "defect": "total-bytes", "limit": 4194304, "value": 4194305}`; every other defect's cause carries neither member. | Test (TC-428) |

## Dependencies

- [FR-305](FR-305-load-explicit-rust-plugins.md) owns the host's plugin forms,
  registration and result handling.
- [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md) owns
  the manifest, request and result bodies.
- [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) owns the
  capability-kind vocabulary a manifest advertises.

## References

- Linear STD-141; QSL ADR-029 PL-2 to PL-7.
