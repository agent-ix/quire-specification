# Review fixtures and expected cases

These files are **proposed fixtures, not executed language results**.
`model-expectations.json` illustrates existing semantic-core field declarations
and the proposed identity-reference mapping; it is explicitly not a compiled
model package. `parent-order.native` is a parse/source fixture. Its literal
`unresolved-model-package` is an intentional link refusal, not a fake digest or
claim of a working import. Replace it only with a real reviewed producer's
package digest when qualifying linking, and record the resulting source change.

`subject.json` supplies concrete authored/profile/source identities to the
verification owner. Its model binding is explicitly unavailable. Exact SHA-256
values bind the source and model-expectation file bytes; the expression span
selects the original body bytes after a non-ASCII comment. It carries no
method/plan/result envelope and no fabricated linked/projection digest.

`operation-subjects.json` supplies pre/post subjects for `version-update.native`,
including exact clause and expression digests/spans and a stable operation ID.
`invocation-cases.json` pairs those subjects with immutable snapshots, invocation
identities, creation/deletion inventories and proposed frame bindings. A changed
version number is permitted by the frame so its adverse case actually violates
the postcondition; it cannot be accidentally refused by an earlier frame check.
The wrong-invocation, missing-pre, forbidden-frame and deleted-object controls
have distinct expected failures. Operation/model linking remains unavailable.
The invocation fixture declares separate named frame expectations; cases select
one by exact artifact reference. A runtime caller is not authorized to redefine an accepted
operation frame. These alternate synthetic bindings still require their own
model/operation review and differ in evidence identity.

`state-cases.json` supplies original synthetic input/expected-value cases for
the eventual evaluator. The healthy and violating expectations are manually
specified, independent of any evaluator. These inputs do not bypass the unresolved
model linking requirement. `boundary-cases.json` describes precise malformed,
unsupported, stale and incomplete mutations and expected diagnostics for later
independent readers. Expected diagnostics are assertions to implement, not
observed tool output.

Reproduce the fixture byte/source checks from this directory:

```sh
sha256sum --check SHA256SUMS
python3 - <<'PY'
import hashlib, json
from pathlib import Path
s = json.loads(Path('subject.json').read_text())
for entry in s['reviewArtifacts']:
    raw = Path(entry['path']).read_bytes()
    assert 'sha256:' + hashlib.sha256(raw).hexdigest() == entry['digest']
raw = Path(s['source']['path']).read_bytes()
span = s['source']['expressionSpan']
actual = raw[span['startByte']:span['endByte']].decode('utf-8')
assert actual == s['source']['expectedExpression']
for prefix in ('start', 'end'):
    before = raw[:span[prefix + 'Byte']].decode('utf-8')
    assert span[prefix + 'Line'] == before.count('\n') + 1
    assert span[prefix + 'Column'] == len(before.rsplit('\n', 1)[-1]) + 1
assert s['modelBinding']['state'] == 'unavailable'
assert 'linkedPackageDigest' not in s and 'projectionDigest' not in s
print('fixture digests and source span verified; no compiler executed')
PY
```

The same source checks apply to each `subjects` entry in
`operation-subjects.json`. Each source also supplies `clauseSpan`/`clauseDigest`
and `expressionSpan`/`expressionDigest`; both digests cover their exact original
byte ranges. The clause region starts at its introducer and ends after its
closing brace, excluding the trailing line ending. The expression region
excludes surrounding indentation and line endings.

The expanded source subject uses fixture version `agent-a-source-subject/1-draft`.
It adds clause-region and expression digests/spans to the earlier `0-draft`
fixture. A consumer must explicitly support this version; neither version is
frozen or registered. Source-language edition and semantic profile remain
unchanged. Old fixture bytes and their identities stay associated with their
original revision rather than being rewritten as current evidence.

The checksum command verifies this file set only. It does not validate language
syntax, imported model semantics, canonical identity, logical outcomes or the
claimed diagnostic behavior. Full compiler integration acceptance still needs real producer,
parser, linker, evaluator, generated backend and clean-consumer execution.

Operation fixtures now use `agent-a-invocation-cases/1-draft`. Each case selects
one exact invocation file. That file binds case-scoped immutable pre/post
observation files and one frame file using `artifactRef` with authority, identity,
revision namespace/value, SHA-256 of exact file bytes, and explicit fixture profile.
Expected outcomes remain outside invocation bytes. Changes to any selected frame
or snapshot digest therefore change the invocation artifact bytes and digest.
No fixture contains a claimed evaluator result/evidence identity.

Snapshots, frames and invocation inputs each have one authoritative payload file;
there is no second inline copy to synchronize. Paths locate local bytes and are
not identity. Reusing an authority/identity/revision with changed content must
refuse even if the new transport digest is internally correct. These are review
profiles, not registered production wire contracts. Model/operation binding
remains unavailable. The source-subject fixture version stays unchanged.
