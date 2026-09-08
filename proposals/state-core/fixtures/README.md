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

The checksum command verifies this file set only. It does not validate language
syntax, imported model semantics, canonical identity, logical outcomes or the
claimed diagnostic behavior. Full compiler integration acceptance still needs real producer,
parser, linker, evaluator, generated backend and clean-consumer execution.
