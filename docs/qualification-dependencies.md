# Qualification implementation and dependency rights

The first-party Rust test source in tests/shared_reference.rs and its Cargo
package are AGPL-3.0-or-later, the license of this repository (see
[LICENSE](../LICENSE); owner decision 2026-09-13).
The repository stays private; no public distribution decision is implied.

| Direct dependency | Exact selection | Existing grant | Purpose |
| --- | --- | --- | --- |
| quire-verification | 5305b40d0bc78bf2a566bd5609669373361e87b8 | AGPL-3.0-or-later | Actual B JCS canonicalizer and legacy bounded fixture intake |
| quire-verification-contracts | 61f4a44d76d51cc53b2dfe7ed10aba7b23cfa6ef | AGPL-3.0-or-later | The sole copy of the closed checked-operation catalog, read by the checked-package qualification tests |
| jsonschema | 0.55.0, default features disabled | MIT | Structural schema/meta-schema validation without HTTP/file retrieval features |
| serde_json | 1.0.151 | MIT OR Apache-2.0 | JSON values and authored fixture operations |
| sha2 | 0.10.9 | MIT OR Apache-2.0 | Exact fixture-byte hashing and independent hashing of hand-authored golden bytes |
| num-bigint | 0.4.8 | MIT OR Apache-2.0 | Arbitrary-precision signed integers for exact nominal-unit rational qualification |
| num-integer | 0.1.47 | MIT OR Apache-2.0 | Exact greatest-common-divisor reduction for canonical nominal-unit rational qualification |
| ix-trace-rs | 2ce4ebf47f726b9d76388220545cd0abda8a5cfb | AGPL-3.0-or-later | Compiler-checked single-line trace attributes |

Cargo.lock pins the actual transitive graph. The retained
[metadata inventory](../reviews/data/verification-reference-dependencies.json)
records every package's version, original license expression and source. These
are consumed grants, not relicensing. Alternative expressions are retained;
MIT or Apache-2.0 can be selected where offered, including dependencies also
offering LGPL. Unicode-3.0, MIT-0 and Zlib terms remain present where required.
No package has a missing license declaration in this inspected graph.

The test crate depends on B only for qualification. B does not depend on this
Rust crate and no production dependency cycle or second canonicalizer is added.
Its own Rust schema generation runs as the existing B build dependency; no
Python, Node or TypeSpec producer is invoked. Quoin's retained tooling is used
only for the authorized specification/review workflow.

The first offline attempt lacked the private git checkout. The selected fetch
succeeded. The initial exact serde_json 1.0.145 pin conflicted with B's typify
dependency; selecting 1.0.151, admitted by B's manifest and used elsewhere in A,
resolved that constraint. Metadata required one additional cached target-only
source download. No B source or manifest was edited.

This inventory is not an ecosystem-wide license decision or security audit.
There is no deny.toml in this repository; no cargo-deny result is claimed.
