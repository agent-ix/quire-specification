# Qualification implementation and dependency rights

The first-party Rust test source in tests/shared_reference.rs and its Cargo
package are AGPL-3.0-or-later, the license of this repository (see
[LICENSE](../LICENSE); owner decision 2026-09-13).
The repository stays private; no public distribution decision is implied.

The dependencies below are dev-dependencies of the qualification tests. The
`quire_specification` library has none. Versions and sources live in
Cargo.toml and Cargo.lock only.

| Direct dependency | Existing grant | Purpose |
| --- | --- | --- |
| quire-verification-contracts | AGPL-3.0-or-later | B's JCS canonicalizer and the sole copy of the closed checked-operation catalog, read by the qualification tests |
| jsonschema | MIT | Structural schema/meta-schema validation, default features disabled so no HTTP/file retrieval |
| serde_json | MIT OR Apache-2.0 | JSON values and authored fixture operations |
| sha2 | MIT OR Apache-2.0 | Exact fixture-byte hashing and independent hashing of hand-authored golden bytes |
| num-bigint | MIT OR Apache-2.0 | Arbitrary-precision signed integers for exact nominal-unit rational qualification |
| num-integer | MIT OR Apache-2.0 | Exact greatest-common-divisor reduction for canonical nominal-unit rational qualification |
| ix-trace-rs | AGPL-3.0-or-later | Compiler-checked single-line trace attributes |

These are consumed grants, not relicensing. Alternative expressions are
retained; MIT or Apache-2.0 can be selected where offered, including
dependencies also offering LGPL. Unicode-3.0, MIT-0 and Zlib terms remain
present where required.

The test crate depends on B only for qualification. B does not depend on this
Rust crate and no production dependency cycle or second canonicalizer is added.
Its own Rust schema generation runs as the existing B build dependency; no
Python, Node or TypeSpec producer is invoked. Quoin's retained tooling is used
only for the authorized specification/review workflow.

This is not an ecosystem-wide license decision or security audit. There is no
deny.toml in this repository; no cargo-deny result is claimed.
