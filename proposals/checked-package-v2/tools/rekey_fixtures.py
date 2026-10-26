#!/usr/bin/env python3
"""Recompute the application-node keys and package id of CheckedPackage V2 fixtures.

Usage: rekey_fixtures.py [--check] FIXTURE.json...

Without --check, every text literal whose `type` is not the package's
scalar_type/text node is retyped to it (FR-464), each application node's
`node_id` is recomputed from its `quire.application-node/v1` preimage until the
graph is stable (a changed key is rewritten wherever it is referenced), and the
source-map entries naming no node are dropped, and the
`package_id` is recomputed from `identity_preimage` plus the occurrence-free
node projection (FR-322). With --check nothing is written; the exit status is
1 if any key or package id is stale. The preimages mirror the helpers in
tests/checked_package_v2.rs; the digest is SHA-256 over RFC 8785 JSON (the
fixtures hold only strings, integers, booleans and null, so sorted compact
JSON is the canonical form).

Derived values elsewhere (dependency-selection-vectors.json holds the package
id of positive-all-families.json and ids built from it) are not rewritten
here: rebuild them from the test failures of `cargo test --test
checked_package_v2` after rekeying.
"""
import copy
import hashlib
import json
import sys


def digest(value):
    canonical = json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)
    return hashlib.sha256(canonical.encode()).hexdigest()


def preimage_term(term, group):
    kind = term.get("term")
    if kind == "reference":
        if term["target"] in group:
            return {"term": "group_reference", "ordinal": group.index(term["target"])}
        return term
    mapped = dict(term)
    if kind == "application":
        mapped["arguments"] = [preimage_term(a, group) for a in term["arguments"]]
    elif kind == "aggregate":
        mapped["members"] = [preimage_term(a, group) for a in term["members"]]
    elif kind == "binding":
        mapped["value"] = preimage_term(term["value"], group)
    return mapped


def is_application_node(node):
    body = node.get("body")
    return isinstance(body, dict) and body.get("term") == "application"


def application_preimage(node, nodes):
    label = node.get("recursion_group")
    group = [m["node_id"] for m in nodes if label is not None and m.get("recursion_group") == label]
    recursion = None
    if node["node_id"] in group:
        recursion = {"size": len(group), "ordinal": group.index(node["node_id"])}
    return {
        "version": "quire.application-node/v1",
        "node_tag": node["node_tag"],
        "semantic_form": node["semantic_form"],
        "semantic_type": node["semantic_type"],
        "declaration": node.get("declaration"),
        "recursion": recursion,
        "body": preimage_term(node["body"], group),
    }


def package_id(package):
    preimage = copy.deepcopy(package["identity_preimage"])
    preimage["identity_projection"] = [
        {k: v for k, v in node.items() if k != "occurrences"}
        for node in package["semantic_graph"]["nodes"]
    ]
    return digest(preimage)


def walk(term, visit):
    if isinstance(term, dict):
        visit(term)
        for value in term.values():
            walk(value, visit)
    elif isinstance(term, list):
        for value in term:
            walk(value, visit)


def stale_source_map(package):
    """Positions of source-map entries naming no node of the graph."""
    ids = {n["node_id"]["digest"] for n in package["semantic_graph"]["nodes"]}
    return [i for i, e in enumerate(package["source_map"]) if e["node_id"]["digest"] not in ids]


def stale(package):
    nodes = package["semantic_graph"]["nodes"]
    keys = [
        i for i, n in enumerate(nodes)
        if is_application_node(n) and digest(application_preimage(n, nodes)) != n["node_id"]["digest"]
    ]
    return keys, package_id(package) != package["package_id"]["digest"], stale_source_map(package)


def rekey(text):
    package = json.loads(text)
    nodes = package["semantic_graph"]["nodes"]
    text_nodes = [n["node_id"] for n in nodes if n["node_tag"] == "scalar_type" and n["semantic_form"] == "text"]
    if len(text_nodes) == 1:
        def retype(term):
            if term.get("term") == "literal" and term.get("value_kind") == "text" and term["type"] != text_nodes[0]:
                term["type"] = dict(text_nodes[0])
        for node in nodes:
            walk(node["body"], retype)
    out = json.dumps(package, indent=2, ensure_ascii=False)
    while True:
        package = json.loads(out)
        nodes = package["semantic_graph"]["nodes"]
        for node in nodes:
            if is_application_node(node):
                new = digest(application_preimage(node, nodes))
                if new != node["node_id"]["digest"]:
                    out = out.replace(node["node_id"]["digest"], new)
                    break
        else:
            break
    package = json.loads(out)
    drop = set(stale_source_map(package))
    package["source_map"] = [e for i, e in enumerate(package["source_map"]) if i not in drop]
    package["package_id"]["digest"] = package_id(package)
    return json.dumps(package, indent=2, ensure_ascii=False)


def main(argv):
    check = "--check" in argv
    paths = [a for a in argv if a != "--check"]
    failed = False
    for path in paths:
        text = open(path, encoding="utf-8").read()
        if check:
            keys, package, source_map = stale(json.loads(text))
            if keys or package or source_map:
                failed = True
                print(f"{path}: stale node keys {keys}, stale package id {package}, stale source map {source_map}")
        else:
            tail = "\n" if text.endswith("\n") else ""
            open(path, "w", encoding="utf-8").write(rekey(text) + tail)
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
