---
id: TM-009
title: "Complete V1 atomic capability and requirement matrix"
type: TestMatrix
relationships:
  - target: ix://agent-ix/quire-specification/FR-020
    type: references
  - target: ix://agent-ix/quire-specification/FR-311
    type: references
---

# Complete V1 atomic capability and requirement matrix

This matrix is normative traceability for the complete inventory.

## Specification action classification

Each row's specification action records what the complete-V1 authoring pass did
with the owning requirement.

- `Reuse`: an existing requirement already has the exact behavior and
  acceptance criteria.
- `Amend`: the right owner exists and its behavior is extended to cover the
  full atomic obligation.
- `Split`: the existing owner combined independently testable behavior, which
  is separated into its own criteria.
- `Resolve`: normative statements disagreed, or an accepted architecture
  excluded a capability the master inventory requires, and the disagreement is
  decided here.
- `New`: behavior and acceptance criteria are authored for the row.

V1-TYPE-024 is a Resolve row because extending reachability over inheritance
and conformance required deciding what "the same type" means for traversal.
FR-043 fixes it as conformance to the static edge type under
`quire.model.complete/v1`, with identity keeping each object's most-specific
type.

## Atomic Capability Coverage

| Capability | Primary requirement | Test cases | Specification action |
| --- | --- | --- | --- |
| V1-SRC-001 | FR-001 | TC-231 | Reuse |
| V1-SRC-002 | FR-001 | TC-231 | Reuse |
| V1-SRC-003 | FR-131 | TC-180, TC-231 | Amend |
| V1-SRC-004 | FR-131 | TC-180, TC-231 | Amend |
| V1-SRC-005 | FR-003 | TC-231 | Reuse |
| V1-SRC-007 | FR-134 | TC-184, TC-231 | Resolve |
| V1-SRC-008 | FR-002 | TC-231 | Reuse |
| V1-SRC-009 | FR-002 | TC-231 | Reuse |
| V1-SRC-010 | FR-132 | TC-182, TC-231 | Amend |
| V1-SRC-013 | FR-132 | TC-182, TC-231 | Amend |
| V1-SRC-014 | FR-133 | TC-183, TC-231 | Amend |
| V1-SRC-015 | FR-001 | TC-231 | Reuse |
| V1-TYPE-001 | FR-002 | TC-231 | Reuse |
| V1-TYPE-002 | FR-005 | TC-231 | Reuse |
| V1-TYPE-003 | FR-007 | TC-231 | Reuse |
| V1-TYPE-004 | FR-039 | TC-231 | Reuse |
| V1-TYPE-005 | FR-140 | TC-185, TC-231 | New |
| V1-TYPE-006 | FR-141 | TC-186, TC-231 | New |
| V1-TYPE-007 | FR-141 | TC-186, TC-231 | New |
| V1-TYPE-008 | FR-142 | TC-187, TC-231 | Amend |
| V1-TYPE-009 | FR-142 | TC-187, TC-231 | New |
| V1-TYPE-010 | FR-019 | TC-231 | Reuse |
| V1-TYPE-011 | FR-019 | TC-231 | Reuse |
| V1-TYPE-012 | FR-143 | TC-188, TC-231 | Amend |
| V1-TYPE-013 | FR-149 | TC-194, TC-231 | Resolve |
| V1-TYPE-014 | FR-008 | TC-231 | Reuse |
| V1-TYPE-015 | FR-144 | TC-189, TC-231 | Amend |
| V1-TYPE-016 | FR-144 | TC-189, TC-231 | Amend |
| V1-TYPE-017 | FR-144 | TC-189, TC-231 | Amend |
| V1-TYPE-018 | FR-145 | TC-190, TC-231 | Amend |
| V1-TYPE-019 | FR-019 | TC-231 | Reuse |
| V1-TYPE-020 | FR-143 | TC-188, TC-231 | Resolve |
| V1-TYPE-021 | FR-011 | TC-231 | Reuse |
| V1-TYPE-022 | FR-009 | TC-231 | Reuse |
| V1-TYPE-023 | FR-153 | TC-198, TC-231 | Amend |
| V1-TYPE-024 | FR-043 | TC-043, TC-231 | Resolve |
| V1-TYPE-025 | FR-150 | TC-195, TC-231 | New |
| V1-TYPE-026 | FR-151 | TC-196, TC-231 | Resolve |
| V1-TYPE-027 | FR-151 | TC-196, TC-231 | New |
| V1-TYPE-028 | FR-151 | TC-196, TC-231 | New |
| V1-TYPE-029 | FR-152 | TC-197, TC-231 | Amend |
| V1-TYPE-030 | FR-019 | TC-231 | Reuse |
| V1-TYPE-031 | FR-147 | TC-192, TC-231 | Resolve |
| V1-TYPE-032 | FR-143 | TC-262, TC-231 | Amend |
| V1-TYPE-033 | FR-143 | TC-263, TC-231 | Amend |
| V1-TYPE-034 | FR-146 | TC-264, TC-231 | Amend |
| V1-EXPR-001 | FR-006 | TC-231 | Reuse |
| V1-EXPR-002 | FR-033 | TC-231 | Reuse |
| V1-EXPR-003 | FR-146 | TC-191, TC-231 | Amend |
| V1-EXPR-004 | FR-146 | TC-191, TC-231 | Resolve |
| V1-EXPR-005 | FR-007 | TC-231 | Reuse |
| V1-EXPR-006 | FR-147 | TC-192, TC-231 | Resolve |
| V1-EXPR-007 | FR-039 | TC-231 | Reuse |
| V1-EXPR-008 | FR-148 | TC-193, TC-231 | Resolve |
| V1-EXPR-009 | FR-148 | TC-193, TC-231 | New |
| V1-EXPR-010 | FR-041 | TC-231 | Reuse |
| V1-EXPR-011 | FR-008 | TC-231 | Reuse |
| V1-EXPR-012 | FR-145 | TC-190, TC-231 | Amend |
| V1-EXPR-013 | FR-145 | TC-190, TC-231 | Resolve |
| V1-EXPR-014 | FR-145 | TC-190, TC-231 | Resolve |
| V1-EXPR-015 | FR-145 | TC-190, TC-231 | Amend |
| V1-EXPR-016 | FR-149 | TC-194, TC-231 | Amend |
| V1-EXPR-017 | FR-009 | TC-231 | Reuse |
| V1-EXPR-018 | FR-012 | TC-231 | Reuse |
| V1-EXPR-019 | FR-012 | TC-231 | Reuse |
| V1-EXPR-020 | FR-012 | TC-231 | Reuse |
| V1-EXPR-021 | FR-006 | TC-231 | Reuse |
| V1-EXPR-022 | FR-013 | TC-231 | Reuse |
| V1-EXPR-023 | FR-271 | TC-047, TC-231 | Amend |
| V1-EXPR-024 | FR-019 | TC-231 | Reuse |
| V1-EXPR-025 | FR-019 | TC-231 | Reuse |
| V1-EXPR-026 | FR-007 | TC-231 | Reuse |
| V1-EXPR-027 | FR-346 | TC-249, TC-231 | New |
| V1-EXPR-028 | FR-347 | TC-250, TC-231 | New |
| V1-EXPR-029 | FR-348 | TC-251, TC-231 | New |
| V1-TEMP-001 | FR-048 | TC-231 | Reuse |
| V1-TEMP-002 | FR-048 | TC-231 | Reuse |
| V1-TEMP-003 | FR-048 | TC-231 | Reuse |
| V1-TEMP-004 | FR-048 | TC-231 | Reuse |
| V1-TEMP-005 | FR-048 | TC-231 | Reuse |
| V1-TEMP-006 | FR-092 | TC-231 | Reuse |
| V1-TEMP-007 | FR-090 | TC-231 | Reuse |
| V1-TEMP-008 | FR-090 | TC-231 | Reuse |
| V1-TEMP-009 | FR-090 | TC-231 | Reuse |
| V1-TEMP-010 | FR-048 | TC-231 | Reuse |
| V1-TEMP-011 | FR-094 | TC-231 | Reuse |
| V1-TEMP-012 | FR-091 | TC-231 | Reuse |
| V1-TEMP-013 | FR-093 | TC-231 | Reuse |
| V1-TEMP-014 | FR-094 | TC-231 | Reuse |
| V1-TEMP-015 | FR-112 | TC-231 | Reuse |
| V1-TEMP-016 | FR-160 | TC-199, TC-231 | Amend |
| V1-TEMP-017 | FR-160 | TC-199, TC-231 | Resolve |
| V1-TEMP-018 | FR-160 | TC-199, TC-231 | Resolve |
| V1-TEMP-019 | FR-095 | TC-231 | Reuse |
| V1-TEMP-020 | FR-034 | TC-231 | Reuse |
| V1-TEMP-021 | FR-091 | TC-231 | Reuse |
| V1-TEMP-022 | FR-090 | TC-231 | Reuse |
| V1-TEMP-023 | FR-161 | TC-200, TC-231 | New |
| V1-TEMP-024 | FR-048 | TC-238, TC-231 | Amend |
| V1-TEMP-025 | FR-360 | TC-200, TC-231 | New |
| V1-TEMP-026 | FR-161 | TC-200, TC-231 | Amend |
| V1-PROT-001 | FR-049 | TC-231 | Reuse |
| V1-PROT-002 | FR-035 | TC-231 | Reuse |
| V1-PROT-003 | FR-035 | TC-231 | Reuse |
| V1-PROT-004 | FR-049 | TC-231 | Reuse |
| V1-PROT-005 | FR-170 | TC-201, TC-231 | Amend |
| V1-PROT-006 | FR-049 | TC-231 | Reuse |
| V1-PROT-007 | FR-035 | TC-231 | Reuse |
| V1-PROT-008 | FR-049 | TC-231 | Reuse |
| V1-PROT-009 | FR-049 | TC-231 | Reuse |
| V1-PROT-010 | FR-171 | TC-202, TC-231 | Amend |
| V1-PROT-011 | FR-049 | TC-231 | Reuse |
| V1-PROT-012 | FR-171 | TC-202, TC-231 | Resolve |
| V1-PROT-013 | FR-170 | TC-201, TC-231 | Amend |
| V1-PROT-014 | FR-171 | TC-202, TC-231 | Resolve |
| V1-PROT-015 | FR-049 | TC-231 | Reuse |
| V1-PROT-016 | FR-172 | TC-203, TC-231 | Amend |
| V1-PROT-017 | FR-054 | TC-231 | Reuse |
| V1-PROT-018 | FR-172 | TC-203, TC-231 | Amend |
| V1-PROT-019 | FR-172 | TC-203, TC-231 | Amend |
| V1-PROT-020 | FR-049 | TC-231 | Reuse |
| V1-PROT-021 | FR-049 | TC-231 | Reuse |
| V1-PROT-022 | FR-057 | TC-231 | Reuse |
| V1-PROT-023 | FR-173 | TC-204, TC-231 | Amend |
| V1-PROT-024 | FR-051 | TC-231 | Reuse |
| V1-PROT-025 | FR-059 | TC-231 | Reuse |
| V1-PROT-026 | FR-059 | TC-231 | Reuse |
| V1-PROT-027 | FR-174 | TC-205, TC-231 | Resolve |
| V1-PROT-028 | FR-175 | TC-206, TC-231 | Amend |
| V1-PROT-029 | FR-176 | TC-207, TC-231 | New |
| V1-PROT-030 | FR-177 | TC-208, TC-231 | Amend |
| V1-PROT-031 | FR-049 | TC-231 | Reuse |
| V1-RUN-001 | FR-005 | TC-231 | Reuse |
| V1-RUN-002 | FR-090 | TC-231 | Reuse |
| V1-RUN-003 | FR-050 | TC-231 | Reuse |
| V1-RUN-004 | FR-094 | TC-231 | Reuse |
| V1-RUN-005 | FR-031 | TC-231 | Reuse |
| V1-RUN-006 | FR-061 | TC-231 | Reuse |
| V1-RUN-007 | FR-011 | TC-231 | Reuse |
| V1-RUN-008 | FR-180 | TC-209, TC-231 | Amend |
| V1-RUN-009 | FR-035 | TC-231 | Reuse |
| V1-RUN-010 | FR-005 | TC-209, TC-231 | Reuse |
| V1-RUN-011 | FR-180 | TC-209, TC-231 | Amend |
| V1-RUN-012 | FR-180 | TC-209, TC-231 | Amend |
| V1-RUN-013 | FR-181 | TC-210, TC-231 | New |
| V1-RUN-014 | FR-190 | TC-212, TC-231 | New |
| V1-RUN-015 | FR-191 | TC-213, TC-231 | New |
| V1-RUN-016 | FR-191 | TC-213, TC-231 | Resolve |
| V1-RUN-017 | FR-192 | TC-214, TC-231 | New |
| V1-RUN-018 | FR-193 | TC-215, TC-231 | New |
| V1-RUN-019 | FR-194 | TC-216, TC-231 | Resolve |
| V1-RUN-020 | FR-182 | TC-211, TC-231 | Amend |
| V1-BACK-001 | FR-031 | TC-231 | Reuse |
| V1-BACK-002 | FR-031 | TC-231 | Reuse |
| V1-BACK-003 | FR-030 | TC-231 | Reuse |
| V1-BACK-004 | FR-195 | TC-217, TC-231 | Amend |
| V1-BACK-005 | FR-196 | TC-218, TC-231 | Amend |
| V1-BACK-006 | FR-197 | TC-219, TC-231, TC-272 | Amend |
| V1-BACK-007 | FR-120 | TC-231 | Reuse |
| V1-BACK-008 | FR-120 | TC-231 | Reuse |
| V1-BACK-009 | FR-120 | TC-231 | Reuse |
| V1-BACK-010 | FR-125 | TC-231 | Reuse |
| V1-BACK-011 | FR-121 | TC-231 | Reuse |
| V1-BACK-012 | FR-019 | TC-231 | Reuse |
| V1-BACK-013 | FR-052 | TC-231 | Reuse |
| V1-BACK-014 | FR-019 | TC-231 | Reuse |
| V1-BACK-015 | FR-352 | TC-255, TC-231 | New |
| V1-BACK-016 | FR-352 | TC-255, TC-231 | New |
| V1-BACK-017 | FR-290 | TC-060, TC-231, TC-271, TC-282 | Amend |
| V1-BACK-018 | FR-292 | TC-060, TC-231 | Reuse |
| V1-BACK-019 | FR-349 | TC-266, TC-231 | New |
| V1-BACK-020 | FR-350 | TC-267, TC-231 | New |
| V1-BACK-021 | FR-353 | TC-268, TC-231 | New |
| V1-BACK-022 | FR-353 | TC-268, TC-231 | New |
| V1-BACK-023 | FR-354 | TC-269, TC-231 | New |
| V1-BACK-024 | FR-355 | TC-270, TC-231 | New |
| V1-EVID-001 | FR-310 | TC-230, TC-231 | Amend |
| V1-EVID-003 | FR-311 | TC-231 | Amend |
| V1-EVID-004 | FR-020 | TC-231 | Reuse |
| V1-EVID-006 | FR-312 | TC-232, TC-231 | Amend |
| V1-EVID-007 | FR-351 | TC-254, TC-231 | New |
| V1-EVID-008 | FR-351 | TC-254, TC-231 | New |
| V1-EVID-009 | FR-351 | TC-254, TC-231 | New |
| V1-TOOL-001 | FR-300 | TC-220, TC-231 | New |
| V1-TOOL-002 | FR-301 | TC-221, TC-231 | New |
| V1-TOOL-003 | FR-302 | TC-222, TC-231 | New |
| V1-TOOL-004 | FR-303 | TC-223, TC-231 | New |
| V1-TOOL-005 | FR-304 | TC-224, TC-231 | New |
| V1-TOOL-006 | FR-305 | TC-225, TC-231 | Resolve |
| V1-TOOL-007 | FR-306 | TC-226, TC-231 | New |
| V1-TOOL-008 | FR-307 | TC-227, TC-231 | Amend |
| V1-TOOL-009 | FR-308 | TC-228, TC-231 | New |
| V1-TOOL-010 | FR-309 | TC-229, TC-231 | New |
| V1-TOOL-011 | FR-452 | TC-413, TC-231 | New |
| V1-TOOL-012 | FR-453 | TC-414, TC-231 | New |

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
| --- | --- | --- | --- |
| FR-131 | FR-131-AC-1 | TC-180 |  |
| FR-131 | FR-131-AC-2 | TC-180 |  |
| FR-131 | FR-131-AC-3 | TC-180 |  |
| FR-132 | FR-132-AC-2 | TC-182 |  |
| FR-132 | FR-132-AC-3 | TC-182 |  |
| FR-133 | FR-133-AC-1 | TC-183 |  |
| FR-133 | FR-133-AC-2 | TC-183 |  |
| FR-133 | FR-133-AC-3 | TC-183 |  |
| FR-134 | FR-134-AC-1 | TC-184 |  |
| FR-134 | FR-134-AC-2 | TC-184 |  |
| FR-134 | FR-134-AC-3 | TC-184 |  |
| FR-134 | FR-134-AC-4 | TC-184 |  |
| FR-134 | FR-134-AC-5 | TC-184 |  |
| FR-140 | FR-140-AC-1 | TC-185 |  |
| FR-140 | FR-140-AC-2 | TC-185 |  |
| FR-140 | FR-140-AC-3 | TC-185 |  |
| FR-141 | FR-141-AC-1 | TC-186 |  |
| FR-141 | FR-141-AC-2 | TC-186 |  |
| FR-141 | FR-141-AC-3 | TC-186 |  |
| FR-142 | FR-142-AC-1 | TC-187 |  |
| FR-142 | FR-142-AC-2 | TC-187 |  |
| FR-142 | FR-142-AC-3 | TC-187 |  |
| FR-143 | FR-143-AC-1 | TC-188 |  |
| FR-143 | FR-143-AC-2 | TC-188 |  |
| FR-143 | FR-143-AC-3 | TC-188 |  |
| FR-144 | FR-144-AC-1 | TC-189 |  |
| FR-144 | FR-144-AC-2 | TC-189 |  |
| FR-144 | FR-144-AC-3 | TC-189 |  |
| FR-145 | FR-145-AC-1 | TC-190 |  |
| FR-145 | FR-145-AC-2 | TC-190 |  |
| FR-145 | FR-145-AC-3 | TC-190 |  |
| FR-146 | FR-146-AC-1 | TC-191 |  |
| FR-146 | FR-146-AC-2 | TC-191 |  |
| FR-146 | FR-146-AC-3 | TC-191 |  |
| FR-147 | FR-147-AC-1 | TC-192 |  |
| FR-147 | FR-147-AC-2 | TC-192 |  |
| FR-147 | FR-147-AC-3 | TC-192 |  |
| FR-148 | FR-148-AC-1 | TC-193 |  |
| FR-148 | FR-148-AC-2 | TC-193 |  |
| FR-148 | FR-148-AC-3 | TC-193 |  |
| FR-149 | FR-149-AC-1 | TC-194 |  |
| FR-149 | FR-149-AC-2 | TC-194 |  |
| FR-149 | FR-149-AC-3 | TC-194 |  |
| FR-150 | FR-150-AC-1 | TC-195 |  |
| FR-150 | FR-150-AC-2 | TC-195 |  |
| FR-150 | FR-150-AC-3 | TC-195 |  |
| FR-151 | FR-151-AC-1 | TC-196 |  |
| FR-151 | FR-151-AC-2 | TC-196 |  |
| FR-151 | FR-151-AC-3 | TC-196 |  |
| FR-152 | FR-152-AC-1 | TC-197 |  |
| FR-152 | FR-152-AC-2 | TC-197 |  |
| FR-152 | FR-152-AC-3 | TC-197 |  |
| FR-153 | FR-153-AC-1 | TC-198 |  |
| FR-153 | FR-153-AC-2 | TC-198 |  |
| FR-153 | FR-153-AC-3 | TC-198 |  |
| FR-160 | FR-160-AC-1 | TC-199 |  |
| FR-160 | FR-160-AC-2 | TC-199 |  |
| FR-160 | FR-160-AC-3 | TC-199 |  |
| FR-161 | FR-161-AC-1 | TC-200 |  |
| FR-161 | FR-161-AC-2 | TC-200 |  |
| FR-161 | FR-161-AC-3 | TC-200, TC-292 |  |
| FR-161 | FR-161-AC-6 | TC-200 |  |
| FR-161 | FR-161-AC-7 | TC-200, TC-301 |  |
| FR-170 | FR-170-AC-1 | TC-201 |  |
| FR-170 | FR-170-AC-2 | TC-201 |  |
| FR-170 | FR-170-AC-3 | TC-201 |  |
| FR-171 | FR-171-AC-1 | TC-202 |  |
| FR-171 | FR-171-AC-2 | TC-202 |  |
| FR-171 | FR-171-AC-3 | TC-202 |  |
| FR-172 | FR-172-AC-1 | TC-203 |  |
| FR-172 | FR-172-AC-2 | TC-203 |  |
| FR-172 | FR-172-AC-3 | TC-203 |  |
| FR-173 | FR-173-AC-1 | TC-204 |  |
| FR-173 | FR-173-AC-2 | TC-204 |  |
| FR-173 | FR-173-AC-3 | TC-204 |  |
| FR-174 | FR-174-AC-1 | TC-205 |  |
| FR-174 | FR-174-AC-2 | TC-205 |  |
| FR-174 | FR-174-AC-3 | TC-205 |  |
| FR-175 | FR-175-AC-1 | TC-206 |  |
| FR-175 | FR-175-AC-2 | TC-206 |  |
| FR-175 | FR-175-AC-3 | TC-206 |  |
| FR-176 | FR-176-AC-1 | TC-207 |  |
| FR-176 | FR-176-AC-2 | TC-207 |  |
| FR-176 | FR-176-AC-3 | TC-207 |  |
| FR-177 | FR-177-AC-1 | TC-208 |  |
| FR-177 | FR-177-AC-2 | TC-208 |  |
| FR-177 | FR-177-AC-3 | TC-208 |  |
| FR-180 | FR-180-AC-1 | TC-209 |  |
| FR-180 | FR-180-AC-2 | TC-209 |  |
| FR-180 | FR-180-AC-3 | TC-209 |  |
| FR-181 | FR-181-AC-1 | TC-210 |  |
| FR-181 | FR-181-AC-2 | TC-210 |  |
| FR-181 | FR-181-AC-3 | TC-210 |  |
| FR-182 | FR-182-AC-1 | TC-211 |  |
| FR-182 | FR-182-AC-2 | TC-211 |  |
| FR-182 | FR-182-AC-3 | TC-211 |  |
| FR-190 | FR-190-AC-1 | TC-212 |  |
| FR-190 | FR-190-AC-2 | TC-212 |  |
| FR-190 | FR-190-AC-3 | TC-212 |  |
| FR-191 | FR-191-AC-1 | TC-213 |  |
| FR-191 | FR-191-AC-2 | TC-213 |  |
| FR-191 | FR-191-AC-3 | TC-213 |  |
| FR-192 | FR-192-AC-1 | TC-214 |  |
| FR-192 | FR-192-AC-2 | TC-214 |  |
| FR-192 | FR-192-AC-3 | TC-214 |  |
| FR-193 | FR-193-AC-1 | TC-215 |  |
| FR-193 | FR-193-AC-2 | TC-215, TC-373 |  |
| FR-193 | FR-193-AC-3 | TC-215 |  |
| FR-194 | FR-194-AC-1 | TC-216 |  |
| FR-194 | FR-194-AC-2 | TC-216 |  |
| FR-194 | FR-194-AC-3 | TC-216 |  |
| FR-195 | FR-195-AC-1 | TC-217 |  |
| FR-195 | FR-195-AC-2 | TC-217 |  |
| FR-195 | FR-195-AC-3 | TC-217 |  |
| FR-196 | FR-196-AC-1 | TC-218 |  |
| FR-196 | FR-196-AC-2 | TC-218 |  |
| FR-196 | FR-196-AC-3 | TC-218 |  |
| FR-197 | FR-197-AC-1 | TC-219 |  |
| FR-197 | FR-197-AC-2 | TC-219 |  |
| FR-197 | FR-197-AC-3 | TC-219 |  |
| FR-300 | FR-300-AC-1 | TC-220 |  |
| FR-300 | FR-300-AC-2 | TC-220 |  |
| FR-300 | FR-300-AC-3 | TC-220 |  |
| FR-300 | FR-300-AC-4 | TC-220 |  |
| FR-300 | FR-300-AC-5 | TC-220 |  |
| FR-301 | FR-301-AC-1 | TC-221 |  |
| FR-301 | FR-301-AC-2 | TC-221 |  |
| FR-301 | FR-301-AC-3 | TC-221 |  |
| FR-302 | FR-302-AC-1 | TC-222 |  |
| FR-302 | FR-302-AC-2 | TC-222 |  |
| FR-302 | FR-302-AC-3 | TC-222 |  |
| FR-303 | FR-303-AC-1 | TC-223 |  |
| FR-303 | FR-303-AC-2 | TC-223 |  |
| FR-303 | FR-303-AC-3 | TC-223 |  |
| FR-303 | FR-303-AC-4 | TC-223 |  |
| FR-303 | FR-303-AC-5 | TC-223 |  |
| FR-304 | FR-304-AC-1 | TC-224 |  |
| FR-304 | FR-304-AC-2 | TC-224 |  |
| FR-304 | FR-304-AC-3 | TC-224 |  |
| FR-305 | FR-305-AC-1 | TC-225 |  |
| FR-305 | FR-305-AC-2 | TC-225 |  |
| FR-305 | FR-305-AC-3 | TC-225 |  |
| FR-305 | FR-305-AC-4 | TC-225 |  |
| FR-306 | FR-306-AC-1 | TC-226 |  |
| FR-306 | FR-306-AC-2 | TC-226 |  |
| FR-306 | FR-306-AC-3 | TC-226 |  |
| FR-306 | FR-306-AC-4 | TC-226 |  |
| FR-306 | FR-306-AC-5 | TC-226 |  |
| FR-307 | FR-307-AC-1 | TC-227 |  |
| FR-307 | FR-307-AC-2 | TC-227 |  |
| FR-307 | FR-307-AC-4 | TC-227 |  |
| FR-307 | FR-307-AC-5 | TC-227 |  |
| FR-307 | FR-307-AC-6 | TC-227 |  |
| FR-308 | FR-308-AC-1 | TC-228 |  |
| FR-308 | FR-308-AC-2 | TC-228 |  |
| FR-308 | FR-308-AC-3 | TC-228 |  |
| FR-309 | FR-309-AC-1 | TC-229 |  |
| FR-309 | FR-309-AC-2 | TC-229 |  |
| FR-309 | FR-309-AC-3 | TC-229 |  |
| FR-310 | FR-310-AC-1 | TC-230 |  |
| FR-310 | FR-310-AC-2 | TC-230 |  |
| FR-310 | FR-310-AC-3 | TC-230 |  |
| FR-311 | FR-311-AC-1 | TC-231 |  |
| FR-311 | FR-311-AC-2 | TC-231 |  |
| FR-312 | FR-312-AC-1 | TC-232 |  |
| FR-312 | FR-312-AC-2 | TC-232 |  |
| FR-312 | FR-312-AC-3 | TC-232 |  |

## Reconciled Existing Requirement Coverage

The rows below close acceptance-criterion coverage omitted from the historical
family matrices. A directly linked test case is used where one exists;
otherwise TC-231 owns the criterion in the complete-V1 qualification corpus.

| Functional Req | Acceptance Criteria | Test Cases | Status |
| --- | --- | --- | --- |
| FR-002 | FR-002-AC-1 | TC-231 |  |
| FR-002 | FR-002-AC-2 | TC-231 |  |
| FR-002 | FR-002-AC-3 | TC-231 |  |
| FR-002 | FR-002-AC-4 | TC-231 |  |
| FR-002 | FR-002-AC-5 | TC-231 |  |
| FR-003 | FR-003-AC-1 | TC-231 |  |
| FR-003 | FR-003-AC-2 | TC-231 |  |
| FR-003 | FR-003-AC-3 | TC-231 |  |
| FR-003 | FR-003-AC-4 | TC-231 |  |
| FR-004 | FR-004-AC-1 | TC-231 |  |
| FR-004 | FR-004-AC-2 | TC-231 |  |
| FR-004 | FR-004-AC-3 | TC-231 |  |
| FR-004 | FR-004-AC-4 | TC-231 |  |
| FR-005 | FR-005-AC-1 | TC-231 |  |
| FR-005 | FR-005-AC-2 | TC-231 |  |
| FR-005 | FR-005-AC-3 | TC-231 |  |
| FR-005 | FR-005-AC-4 | TC-231 |  |
| FR-006 | FR-006-AC-1 | TC-231 |  |
| FR-006 | FR-006-AC-2 | TC-231 |  |
| FR-006 | FR-006-AC-3 | TC-231 |  |
| FR-006 | FR-006-AC-4 | TC-231 |  |
| FR-006 | FR-006-AC-5 | TC-231 |  |
| FR-006 | FR-006-AC-6 | TC-231 |  |
| FR-006 | FR-006-AC-7 | TC-231 |  |
| FR-006 | FR-006-AC-8 | TC-231 |  |
| FR-006 | FR-006-AC-9 | TC-231 |  |
| FR-007 | FR-007-AC-1 | TC-231 |  |
| FR-007 | FR-007-AC-2 | TC-231 |  |
| FR-007 | FR-007-AC-3 | TC-231 |  |
| FR-007 | FR-007-AC-4 | TC-231 |  |
| FR-007 | FR-007-AC-5 | TC-231 |  |
| FR-007 | FR-007-AC-6 | TC-231 |  |
| FR-007 | FR-007-AC-7 | TC-192 |  |
| FR-008 | FR-008-AC-1 | TC-231 |  |
| FR-008 | FR-008-AC-2 | TC-231 |  |
| FR-008 | FR-008-AC-3 | TC-231 |  |
| FR-008 | FR-008-AC-4 | TC-231 |  |
| FR-008 | FR-008-AC-5 | TC-231 |  |
| FR-008 | FR-008-AC-6 | TC-189, TC-190, TC-231 |  |
| FR-009 | FR-009-AC-1 | TC-231 |  |
| FR-009 | FR-009-AC-2 | TC-231 |  |
| FR-009 | FR-009-AC-3 | TC-231 |  |
| FR-010 | FR-010-AC-1 | TC-231 |  |
| FR-010 | FR-010-AC-2 | TC-231 |  |
| FR-010 | FR-010-AC-3 | TC-231 |  |
| FR-010 | FR-010-AC-4 | TC-231 |  |
| FR-011 | FR-011-AC-1 | TC-231 |  |
| FR-011 | FR-011-AC-2 | TC-231 |  |
| FR-011 | FR-011-AC-3 | TC-231 |  |
| FR-011 | FR-011-AC-4 | TC-231 |  |
| FR-011 | FR-011-AC-5 | TC-231 |  |
| FR-012 | FR-012-AC-1 | TC-231 |  |
| FR-012 | FR-012-AC-2 | TC-231 |  |
| FR-012 | FR-012-AC-3 | TC-231 |  |
| FR-012 | FR-012-AC-4 | TC-231 |  |
| FR-012 | FR-012-AC-5 | TC-231 |  |
| FR-012 | FR-012-AC-6 | TC-231 |  |
| FR-012 | FR-012-AC-7 | TC-231 |  |
| FR-012 | FR-012-AC-8 | TC-231 |  |
| FR-013 | FR-013-AC-1 | TC-231 |  |
| FR-013 | FR-013-AC-2 | TC-231 |  |
| FR-013 | FR-013-AC-3 | TC-231 |  |
| FR-013 | FR-013-AC-4 | TC-231 |  |
| FR-013 | FR-013-AC-5 | TC-236 |  |
| FR-013 | FR-013-AC-6 | TC-236 |  |
| FR-013 | FR-013-AC-7 | TC-236 |  |
| FR-014 | FR-014-AC-1 | TC-231 |  |
| FR-014 | FR-014-AC-2 | TC-231 |  |
| FR-014 | FR-014-AC-3 | TC-231 |  |
| FR-014 | FR-014-AC-4 | TC-231 |  |
| FR-014 | FR-014-AC-5 | TC-231 |  |
| FR-015 | FR-015-AC-1 | TC-231 |  |
| FR-015 | FR-015-AC-2 | TC-231 |  |
| FR-015 | FR-015-AC-3 | TC-231 |  |
| FR-015 | FR-015-AC-4 | TC-231 |  |
| FR-019 | FR-019-AC-7 | TC-006 |  |
| FR-020 | FR-020-AC-11 | TC-020 |  |
| FR-091 | FR-091-AC-7 | TC-111 |  |
| FR-091 | FR-091-AC-8 | TC-200 |  |
| FR-093 | FR-093-AC-6 | TC-114 |  |
| FR-120 | FR-120-AC-2 | TC-150 |  |
| FR-120 | FR-120-AC-3 | TC-150 |  |
| FR-120 | FR-120-AC-4 | TC-150 |  |
| FR-121 | FR-121-AC-2 | TC-150 |  |
| FR-121 | FR-121-AC-3 | TC-150 |  |
| FR-121 | FR-121-AC-4 | TC-150 |  |
| FR-121 | FR-121-AC-5 | TC-150 |  |
| FR-122 | FR-122-AC-2 | TC-151 |  |
| FR-122 | FR-122-AC-3 | TC-151 |  |
| FR-122 | FR-122-AC-4 | TC-151 |  |
| FR-122 | FR-122-AC-5 | TC-151 |  |
| FR-123 | FR-123-AC-2 | TC-152 |  |
| FR-123 | FR-123-AC-3 | TC-152 |  |
| FR-123 | FR-123-AC-4 | TC-152 |  |
| FR-123 | FR-123-AC-5 | TC-152 |  |
| FR-124 | FR-124-AC-2 | TC-153 |  |
| FR-124 | FR-124-AC-3 | TC-153 |  |
| FR-124 | FR-124-AC-4 | TC-153 |  |
| FR-124 | FR-124-AC-5 | TC-153 |  |
| FR-125 | FR-125-AC-2 | TC-154 |  |
| FR-125 | FR-125-AC-3 | TC-154 |  |
| FR-125 | FR-125-AC-4 | TC-154 |  |
| FR-125 | FR-125-AC-5 | TC-154 |  |
| FR-200 | FR-200-AC-1 | TC-231 |  |
| FR-200 | FR-200-AC-2 | TC-231 |  |
| FR-200 | FR-200-AC-3 | TC-231 |  |
| FR-200 | FR-200-AC-4 | TC-231 |  |
| FR-200 | FR-200-AC-5 | TC-231 |  |
| FR-200 | FR-200-AC-6 | TC-231 |  |
| FR-201 | FR-201-AC-1 | TC-231 |  |
| FR-201 | FR-201-AC-2 | TC-231 |  |
| FR-201 | FR-201-AC-3 | TC-231 |  |
| FR-201 | FR-201-AC-4 | TC-231 |  |
| FR-201 | FR-201-AC-7 | TC-233 |  |
| FR-201 | FR-201-AC-9 | TC-233 |  |
| FR-201 | FR-201-AC-8 | TC-226 |  |
| FR-202 | FR-202-AC-1 | TC-231 |  |
| FR-202 | FR-202-AC-2 | TC-231 |  |
| FR-202 | FR-202-AC-3 | TC-231 |  |
| FR-202 | FR-202-AC-4 | TC-231 |  |
| FR-203 | FR-203-AC-1 | TC-231 |  |
| FR-203 | FR-203-AC-2 | TC-231 |  |
| FR-203 | FR-203-AC-3 | TC-231 |  |
| FR-203 | FR-203-AC-4 | TC-231 |  |
| FR-204 | FR-204-AC-1 | TC-231 |  |
| FR-204 | FR-204-AC-2 | TC-231 |  |
| FR-204 | FR-204-AC-3 | TC-231 |  |
| FR-204 | FR-204-AC-4 | TC-231 |  |
| FR-205 | FR-205-AC-1 | TC-231 |  |
| FR-205 | FR-205-AC-2 | TC-231 |  |
| FR-205 | FR-205-AC-3 | TC-231 |  |
| FR-205 | FR-205-AC-4 | TC-231 |  |
| FR-206 | FR-206-AC-1 | TC-231 |  |
| FR-206 | FR-206-AC-2 | TC-231 |  |
| FR-206 | FR-206-AC-3 | TC-231 |  |
| FR-206 | FR-206-AC-4 | TC-231 |  |
| FR-207 | FR-207-AC-1 | TC-231 |  |
| FR-207 | FR-207-AC-2 | TC-231 |  |
| FR-207 | FR-207-AC-3 | TC-231 |  |
| FR-207 | FR-207-AC-4 | TC-231 |  |
| FR-207 | FR-207-AC-5 | TC-231 |  |
| FR-207 | FR-207-AC-6 | TC-231 |  |
| FR-207 | FR-207-AC-7 | TC-231 |  |
| FR-207 | FR-207-AC-8 | TC-235 |  |
| FR-207 | FR-207-AC-9 | TC-429 |  |
| FR-207 | FR-207-AC-10 | TC-429 |  |
| FR-207 | FR-207-AC-11 | TC-429 |  |
| FR-210 | FR-210-AC-1 | TC-050 |  |
| FR-210 | FR-210-AC-2 | TC-050 |  |
| FR-210 | FR-210-AC-3 | TC-050 |  |
| FR-211 | FR-211-AC-1 | TC-050 |  |
| FR-211 | FR-211-AC-2 | TC-050 |  |
| FR-211 | FR-211-AC-4 | TC-050 |  |
| FR-212 | FR-212-AC-1 | TC-050 |  |
| FR-212 | FR-212-AC-2 | TC-050 |  |
| FR-212 | FR-212-AC-3 | TC-050 |  |
| FR-213 | FR-213-AC-1 | TC-050 |  |
| FR-213 | FR-213-AC-2 | TC-050 |  |
| FR-213 | FR-213-AC-3 | TC-050 |  |
| FR-214 | FR-214-AC-1 | TC-050 |  |
| FR-214 | FR-214-AC-2 | TC-050 |  |
| FR-214 | FR-214-AC-3 | TC-050 |  |
| FR-215 | FR-215-AC-1 | TC-050 |  |
| FR-215 | FR-215-AC-2 | TC-050 |  |
| FR-215 | FR-215-AC-3 | TC-050 |  |
| FR-215 | FR-215-AC-4 | TC-050 |  |
| FR-221 | FR-221-AC-1 | TC-171 |  |
| FR-221 | FR-221-AC-2 | TC-171 |  |
| FR-221 | FR-221-AC-3 | TC-171 |  |
| FR-222 | FR-222-AC-1 | TC-231 |  |
| FR-222 | FR-222-AC-2 | TC-231 |  |
| FR-222 | FR-222-AC-3 | TC-231 |  |
| FR-223 | FR-223-AC-1 | TC-231 |  |
| FR-223 | FR-223-AC-2 | TC-231 |  |
| FR-223 | FR-223-AC-3 | TC-231 |  |
| FR-224 | FR-224-AC-1 | TC-051 |  |
| FR-224 | FR-224-AC-2 | TC-051 |  |
| FR-225 | FR-225-AC-1 | TC-051 |  |
| FR-225 | FR-225-AC-2 | TC-051 |  |
| FR-225 | FR-225-AC-3 | TC-051 |  |
| FR-226 | FR-226-AC-1 | TC-051 |  |
| FR-226 | FR-226-AC-2 | TC-051 |  |
| FR-227 | FR-227-AC-1 | TC-231 |  |
| FR-227 | FR-227-AC-2 | TC-231 |  |
| FR-227 | FR-227-AC-3 | TC-231 |  |
| FR-227 | FR-227-AC-4 | TC-231 |  |
| FR-228 | FR-228-AC-1 | TC-231 |  |
| FR-228 | FR-228-AC-2 | TC-231 |  |
| FR-228 | FR-228-AC-3 | TC-231 |  |
| FR-228 | FR-228-AC-4 | TC-231 |  |
| FR-228 | FR-228-AC-5 | TC-248 |  |
| FR-230 | FR-230-AC-1 | TC-231 |  |
| FR-230 | FR-230-AC-2 | TC-231 |  |
| FR-230 | FR-230-AC-3 | TC-231 |  |
| FR-230 | FR-230-AC-4 | TC-231 |  |
| FR-231 | FR-231-AC-1 | TC-231 |  |
| FR-231 | FR-231-AC-2 | TC-231 |  |
| FR-231 | FR-231-AC-3 | TC-231 |  |
| FR-231 | FR-231-AC-4 | TC-231 |  |
| FR-232 | FR-232-AC-1 | TC-231 |  |
| FR-232 | FR-232-AC-2 | TC-231 |  |
| FR-232 | FR-232-AC-3 | TC-231 |  |
| FR-232 | FR-232-AC-4 | TC-231 |  |
| FR-234 | FR-234-AC-1 | TC-231 |  |
| FR-234 | FR-234-AC-2 | TC-231 |  |
| FR-234 | FR-234-AC-3 | TC-231 |  |
| FR-234 | FR-234-AC-4 | TC-231 |  |
| FR-235 | FR-235-AC-1 | TC-058 |  |
| FR-235 | FR-235-AC-2 | TC-058 |  |
| FR-235 | FR-235-AC-3 | TC-058 |  |
| FR-235 | FR-235-AC-4 | TC-058 |  |
| FR-238 | FR-238-AC-1 | TC-057 |  |
| FR-238 | FR-238-AC-2 | TC-057 |  |
| FR-238 | FR-238-AC-3 | TC-057 |  |
| FR-238 | FR-238-AC-4 | TC-057 |  |
| FR-241 | FR-241-AC-1 | TC-231 |  |
| FR-241 | FR-241-AC-2 | TC-231 |  |
| FR-241 | FR-241-AC-3 | TC-231 |  |
| FR-241 | FR-241-AC-4 | TC-231 |  |
| FR-244 | FR-244-AC-1 | TC-231 |  |
| FR-244 | FR-244-AC-2 | TC-231 |  |
| FR-244 | FR-244-AC-3 | TC-231 |  |
| FR-244 | FR-244-AC-4 | TC-231 |  |
| FR-245 | FR-245-AC-1 | TC-231 |  |
| FR-245 | FR-245-AC-2 | TC-231 |  |
| FR-245 | FR-245-AC-3 | TC-231 |  |
| FR-245 | FR-245-AC-4 | TC-231 |  |
| FR-246 | FR-246-AC-1 | TC-231 |  |
| FR-246 | FR-246-AC-2 | TC-231 |  |
| FR-246 | FR-246-AC-3 | TC-231 |  |
| FR-246 | FR-246-AC-4 | TC-231 |  |
| FR-247 | FR-247-AC-1 | TC-231 |  |
| FR-247 | FR-247-AC-2 | TC-231 |  |
| FR-247 | FR-247-AC-3 | TC-231 |  |
| FR-247 | FR-247-AC-4 | TC-231 |  |
| FR-247 | FR-247-AC-5 | TC-231 |  |
| FR-248 | FR-248-AC-1 | TC-231 |  |
| FR-248 | FR-248-AC-2 | TC-231 |  |
| FR-248 | FR-248-AC-3 | TC-231 |  |
| FR-248 | FR-248-AC-4 | TC-231 |  |
| FR-248 | FR-248-AC-5 | TC-231 |  |
| FR-248 | FR-248-AC-6 | TC-231 |  |
| FR-250 | FR-250-AC-1 | TC-231 |  |
| FR-250 | FR-250-AC-2 | TC-231 |  |
| FR-250 | FR-250-AC-3 | TC-231 |  |
| FR-250 | FR-250-AC-5 | TC-231 |  |
| FR-250 | FR-250-AC-6 | TC-231 |  |
| FR-251 | FR-251-AC-1 | TC-231 |  |
| FR-251 | FR-251-AC-2 | TC-231 |  |
| FR-251 | FR-251-AC-3 | TC-231 |  |
| FR-251 | FR-251-AC-4 | TC-231 |  |
| FR-252 | FR-252-AC-1 | TC-231 |  |
| FR-252 | FR-252-AC-2 | TC-231 |  |
| FR-252 | FR-252-AC-3 | TC-231 |  |
| FR-252 | FR-252-AC-4 | TC-231 |  |
| FR-252 | FR-252-AC-5 | TC-231 |  |
| FR-253 | FR-253-AC-1 | TC-231 |  |
| FR-253 | FR-253-AC-2 | TC-231 |  |
| FR-253 | FR-253-AC-3 | TC-231 |  |
| FR-253 | FR-253-AC-4 | TC-231 |  |
| FR-254 | FR-254-AC-1 | TC-231 |  |
| FR-254 | FR-254-AC-2 | TC-231 |  |
| FR-254 | FR-254-AC-3 | TC-231 |  |
| FR-254 | FR-254-AC-4 | TC-231 |  |
| FR-254 | FR-254-AC-5 | TC-231 |  |
| FR-255 | FR-255-AC-1 | TC-231 |  |
| FR-255 | FR-255-AC-2 | TC-231 |  |
| FR-255 | FR-255-AC-3 | TC-231 |  |
| FR-255 | FR-255-AC-4 | TC-231 |  |
| FR-255 | FR-255-AC-5 | TC-231 |  |
| FR-256 | FR-256-AC-1 | TC-231 |  |
| FR-256 | FR-256-AC-2 | TC-231 |  |
| FR-256 | FR-256-AC-3 | TC-231 |  |
| FR-256 | FR-256-AC-4 | TC-231 |  |
| FR-256 | FR-256-AC-5 | TC-231 |  |
| FR-256 | FR-256-AC-6 | TC-231 |  |
| FR-257 | FR-257-AC-1 | TC-231 |  |
| FR-257 | FR-257-AC-2 | TC-231 |  |
| FR-257 | FR-257-AC-3 | TC-231 |  |
| FR-257 | FR-257-AC-4 | TC-231 |  |
| FR-257 | FR-257-AC-5 | TC-231 |  |
| FR-257 | FR-257-AC-6 | TC-231 |  |
| FR-258 | FR-258-AC-1 | TC-231 |  |
| FR-258 | FR-258-AC-2 | TC-231 |  |
| FR-258 | FR-258-AC-3 | TC-231 |  |
| FR-258 | FR-258-AC-4 | TC-231 |  |
| FR-258 | FR-258-AC-5 | TC-231 |  |
| FR-258 | FR-258-AC-6 | TC-231 |  |
| FR-258 | FR-258-AC-7 | TC-231 |  |
| FR-259 | FR-259-AC-1 | TC-231 |  |
| FR-259 | FR-259-AC-2 | TC-231 |  |
| FR-259 | FR-259-AC-3 | TC-231 |  |
| FR-259 | FR-259-AC-4 | TC-231 |  |
| FR-259 | FR-259-AC-5 | TC-231 |  |
| FR-259 | FR-259-AC-6 | TC-231 |  |
| FR-269 | FR-269-AC-1 | TC-231 |  |
| FR-269 | FR-269-AC-2 | TC-231 |  |
| FR-269 | FR-269-AC-3 | TC-231 |  |
| FR-269 | FR-269-AC-4 | TC-231 |  |
| FR-270 | FR-270-AC-1 | TC-231 |  |
| FR-270 | FR-270-AC-2 | TC-231 |  |
| FR-270 | FR-270-AC-3 | TC-231 |  |
| FR-270 | FR-270-AC-4 | TC-231 |  |
| FR-271 | FR-271-AC-6 | TC-233 |  |
| FR-272 | FR-272-AC-1 | TC-231 |  |
| FR-272 | FR-272-AC-2 | TC-231 |  |
| FR-272 | FR-272-AC-3 | TC-231 |  |
| FR-272 | FR-272-AC-4 | TC-231 |  |
| FR-272 | FR-272-AC-5 | TC-231 |  |
| FR-274 | FR-274-AC-1 | TC-231 |  |
| FR-274 | FR-274-AC-2 | TC-231 |  |
| FR-274 | FR-274-AC-3 | TC-231 |  |
| FR-274 | FR-274-AC-4 | TC-231 |  |
| FR-274 | FR-274-AC-5 | TC-231 |  |
| FR-280 | FR-280-AC-1 | TC-231 |  |
| FR-280 | FR-280-AC-2 | TC-231 |  |
| FR-280 | FR-280-AC-3 | TC-231 |  |
| FR-281 | FR-281-AC-1 | TC-061 |  |
| FR-281 | FR-281-AC-2 | TC-061 |  |
| FR-281 | FR-281-AC-3 | TC-061 |  |
| FR-282 | FR-282-AC-1 | TC-231 |  |
| FR-282 | FR-282-AC-2 | TC-231 |  |
| FR-282 | FR-282-AC-3 | TC-231 |  |
| FR-282 | FR-282-AC-4 | TC-231 |  |
| FR-284 | FR-284-AC-1 | TC-231 |  |
| FR-284 | FR-284-AC-2 | TC-231 |  |
| FR-284 | FR-284-AC-3 | TC-231 |  |
| FR-284 | FR-284-AC-4 | TC-231 |  |
| FR-284 | FR-284-AC-5 | TC-231 |  |
| FR-284 | FR-284-AC-6 | TC-231 |  |
| FR-284 | FR-284-AC-7 | TC-231 |  |
| FR-284 | FR-284-AC-8 | TC-231 |  |
| FR-297 | FR-297-AC-2 | TC-150 |  |
| FR-297 | FR-297-AC-3 | TC-150 |  |
| FR-297 | FR-297-AC-4 | TC-150 |  |
| FR-298 | FR-298-AC-2 | TC-150 |  |
| FR-298 | FR-298-AC-3 | TC-150 |  |
| FR-298 | FR-298-AC-4 | TC-150 |  |
| FR-298 | FR-298-AC-5 | TC-150 |  |
| FR-299 | FR-299-AC-2 | TC-231 |  |
| FR-299 | FR-299-AC-3 | TC-231 |  |
| FR-299 | FR-299-AC-4 | TC-231 |  |
| FR-299 | FR-299-AC-5 | TC-231 |  |
| FR-140 | FR-140-AC-4 | TC-185 |  |
| FR-140 | FR-140-AC-5 | TC-185, TC-233 |  |
| FR-140 | FR-140-AC-6 | TC-185 |  |
| FR-140 | FR-140-AC-7 | TC-185 |  |
| FR-141 | FR-141-AC-4 | TC-186 |  |
| FR-141 | FR-141-AC-5 | TC-186 |  |
| FR-141 | FR-141-AC-6 | TC-186 |  |
| FR-142 | FR-142-AC-4 | TC-187 |  |
| FR-142 | FR-142-AC-5 | TC-187 |  |
| FR-142 | FR-142-AC-6 | TC-187 |  |
| FR-142 | FR-142-AC-7 | TC-187 |  |
| FR-142 | FR-142-AC-8 | TC-233 |  |
| FR-142 | FR-142-AC-9 | TC-187 |  |
| FR-142 | FR-142-AC-10 | TC-187 |  |
| FR-143 | FR-143-AC-4 | TC-188 |  |
| FR-143 | FR-143-AC-5 | TC-188 |  |
| FR-143 | FR-143-AC-6 | TC-188 |  |
| FR-143 | FR-143-AC-7 | TC-188 |  |
| FR-143 | FR-143-AC-8 | TC-188 |  |
| FR-143 | FR-143-AC-9 | TC-188 |  |
| FR-143 | FR-143-AC-10 | TC-188 |  |
| FR-144 | FR-144-AC-4 | TC-189 |  |
| FR-144 | FR-144-AC-5 | TC-189 |  |
| FR-144 | FR-144-AC-6 | TC-189 |  |
| FR-144 | FR-144-AC-7 | TC-189 |  |
| FR-144 | FR-144-AC-8 | TC-189 |  |
| FR-144 | FR-144-AC-9 | TC-189 |  |
| FR-144 | FR-144-AC-10 | TC-189 |  |
| FR-145 | FR-145-AC-4 | TC-190 |  |
| FR-145 | FR-145-AC-5 | TC-190 |  |
| FR-145 | FR-145-AC-6 | TC-190 |  |
| FR-145 | FR-145-AC-7 | TC-190 |  |
| FR-145 | FR-145-AC-8 | TC-190 |  |
| FR-145 | FR-145-AC-9 | TC-190 |  |
| FR-146 | FR-146-AC-4 | TC-191 |  |
| FR-146 | FR-146-AC-5 | TC-191 |  |
| FR-146 | FR-146-AC-6 | TC-191 |  |
| FR-146 | FR-146-AC-7 | TC-191 |  |
| FR-146 | FR-146-AC-8 | TC-191 |  |
| FR-146 | FR-146-AC-9 | TC-191 |  |
| FR-146 | FR-146-AC-10 | TC-264 |  |
| FR-146 | FR-146-AC-11 | TC-265 |  |
| FR-146 | FR-146-AC-12 | TC-398 |  |
| FR-146 | FR-146-AC-13 | TC-399 |  |
| FR-146 | FR-146-AC-14 | TC-398 |  |
| FR-147 | FR-147-AC-4 | TC-192 |  |
| FR-147 | FR-147-AC-5 | TC-192 |  |
| FR-147 | FR-147-AC-6 | TC-192 |  |
| FR-147 | FR-147-AC-7 | TC-192, TC-266 |  |
| FR-147 | FR-147-AC-8 | TC-192, TC-266 |  |
| FR-147 | FR-147-AC-9 | TC-192 |  |
| FR-147 | FR-147-AC-10 | TC-192 |  |
| FR-148 | FR-148-AC-4 | TC-193 |  |
| FR-148 | FR-148-AC-5 | TC-193 |  |
| FR-148 | FR-148-AC-6 | TC-193 |  |
| FR-148 | FR-148-AC-7 | TC-193 |  |
| FR-148 | FR-148-AC-8 | TC-193 |  |
| FR-148 | FR-148-AC-9 | TC-193 |  |
| FR-148 | FR-148-AC-10 | TC-193 |  |
| FR-149 | FR-149-AC-4 | TC-194 |  |
| FR-149 | FR-149-AC-5 | TC-194 |  |
| FR-149 | FR-149-AC-6 | TC-194 |  |
| FR-149 | FR-149-AC-7 | TC-194 |  |
| FR-149 | FR-149-AC-8 | TC-194 |  |
| FR-149 | FR-149-AC-9 | TC-194 |  |
| FR-149 | FR-149-AC-10 | TC-194 |  |
| FR-149 | FR-149-AC-11 | TC-194 |  |
| FR-149 | FR-149-AC-13 | TC-194 |  |
| FR-150 | FR-150-AC-4 | TC-195 |  |
| FR-150 | FR-150-AC-5 | TC-195 |  |
| FR-151 | FR-151-AC-4 | TC-196 |  |
| FR-151 | FR-151-AC-5 | TC-196 |  |
| FR-152 | FR-152-AC-4 | TC-197 |  |
| FR-152 | FR-152-AC-5 | TC-197 |  |
| FR-153 | FR-153-AC-4 | TC-198 |  |
| FR-153 | FR-153-AC-5 | TC-198 |  |
| FR-143 | FR-143-AC-11 | TC-198 |  |
| FR-143 | FR-143-AC-12 | TC-262 |  |
| FR-143 | FR-143-AC-13 | TC-263 |  |
| FR-143 | FR-143-AC-14 | TC-399 |  |
| FR-143 | FR-143-AC-15 | TC-400 |  |
| FR-143 | FR-143-AC-16 | TC-400 |  |
| FR-144 | FR-144-AC-11 | TC-198 |  |
| FR-144 | FR-144-AC-12 | TC-246 |  |
| FR-144 | FR-144-AC-13 | TC-246 |  |
| FR-144 | FR-144-AC-14 | TC-397 |  |
| FR-149 | FR-149-AC-12 | TC-198 |  |
| FR-150 | FR-150-AC-6 | TC-195 |  |
| FR-150 | FR-150-AC-7 | TC-195 |  |
| FR-150 | FR-150-AC-8 | TC-195 |  |
| FR-150 | FR-150-AC-12 | TC-195 |  |
| FR-150 | FR-150-AC-13 | TC-195 |  |
| FR-150 | FR-150-AC-14 | TC-195 |  |
| FR-150 | FR-150-AC-15 | TC-195 |  |
| FR-150 | FR-150-AC-16 | TC-195 |  |
| FR-151 | FR-151-AC-6 | TC-196 |  |
| FR-151 | FR-151-AC-7 | TC-196 |  |
| FR-151 | FR-151-AC-8 | TC-196 |  |
| FR-151 | FR-151-AC-9 | TC-196 |  |
| FR-151 | FR-151-AC-10 | TC-196 |  |
| FR-151 | FR-151-AC-11 | TC-196 |  |
| FR-151 | FR-151-AC-16 | TC-938 |  |
| FR-152 | FR-152-AC-6 | TC-197 |  |
| FR-152 | FR-152-AC-7 | TC-197 |  |
| FR-152 | FR-152-AC-8 | TC-197 |  |
| FR-153 | FR-153-AC-6 | TC-198 |  |
| FR-153 | FR-153-AC-7 | TC-198 |  |
| FR-153 | FR-153-AC-8 | TC-198 |  |
| FR-153 | FR-153-AC-9 | TC-247 |  |
| FR-154 | FR-154-AC-1 | TC-234 |  |
| FR-154 | FR-154-AC-2 | TC-234 |  |
| FR-154 | FR-154-AC-3 | TC-234 |  |
| FR-154 | FR-154-AC-4 | TC-234 |  |
| FR-154 | FR-154-AC-5 | TC-234 |  |
| FR-154 | FR-154-AC-6 | TC-234 |  |
| FR-154 | FR-154-AC-7 | TC-234 |  |
| FR-154 | FR-154-AC-8 | TC-234 |  |
| FR-204 | FR-204-AC-5 | TC-198 |  |
| FR-208 | FR-208-AC-1 | TC-231 |  |
| FR-208 | FR-208-AC-2 | TC-197 |  |
| FR-208 | FR-208-AC-3 | TC-234 |  |
| FR-208 | FR-208-AC-4 | TC-235 |  |
| FR-208 | FR-208-AC-5 | TC-235 |  |
| FR-208 | FR-208-AC-6 | TC-235 |  |
| FR-208 | FR-208-AC-7 | TC-235 |  |
| FR-208 | FR-208-AC-8 | TC-235 |  |
| FR-208 | FR-208-AC-9 | TC-235 |  |
| FR-208 | FR-208-AC-10 | TC-235 |  |
| FR-208 | FR-208-AC-11 | TC-235 |  |
| FR-208 | FR-208-AC-12 | TC-235 |  |
| FR-208 | FR-208-AC-13 | TC-401 |  |
| FR-272 | FR-272-AC-6 | TC-196 |  |
| FR-272 | FR-272-AC-7 | TC-233 |  |
| FR-272 | FR-272-AC-8 | TC-264 |  |
| FR-272 | FR-272-AC-9 | TC-271, TC-282 |  |
| FR-272 | FR-272-AC-10 | TC-233 |  |
| FR-272 | FR-272-AC-11 | TC-233 |  |
| FR-272 | FR-272-AC-12 | TC-233 |  |
| FR-272 | FR-272-AC-13 | TC-233 |  |
| FR-272 | FR-272-AC-14 | TC-047 |  |
| FR-321 | FR-321-AC-4 | TC-195 |  |
| FR-160 | FR-160-AC-4 | TC-199 |  |
| FR-160 | FR-160-AC-5 | TC-199 |  |
| FR-161 | FR-161-AC-4 | TC-200 |  |
| FR-161 | FR-161-AC-5 | TC-200 |  |
| FR-170 | FR-170-AC-4 | TC-201 |  |
| FR-170 | FR-170-AC-5 | TC-201 |  |
| FR-171 | FR-171-AC-4 | TC-202 |  |
| FR-171 | FR-171-AC-5 | TC-202 |  |
| FR-172 | FR-172-AC-4 | TC-203 |  |
| FR-172 | FR-172-AC-5 | TC-203 |  |
| FR-173 | FR-173-AC-4 | TC-204 |  |
| FR-173 | FR-173-AC-5 | TC-204 |  |
| FR-174 | FR-174-AC-4 | TC-205 |  |
| FR-174 | FR-174-AC-5 | TC-205 |  |
| FR-175 | FR-175-AC-4 | TC-206 |  |
| FR-175 | FR-175-AC-5 | TC-206 |  |
| FR-176 | FR-176-AC-4 | TC-207 |  |
| FR-176 | FR-176-AC-5 | TC-207 |  |
| FR-177 | FR-177-AC-4 | TC-208 |  |
| FR-177 | FR-177-AC-5 | TC-208 |  |
| FR-180 | FR-180-AC-4 | TC-209 |  |
| FR-180 | FR-180-AC-5 | TC-209 |  |
| FR-181 | FR-181-AC-4 | TC-210 |  |
| FR-181 | FR-181-AC-5 | TC-210, TC-335, TC-327 |  |
| FR-181 | FR-181-AC-6 | TC-210 |  |
| FR-181 | FR-181-AC-13 | TC-335 |  |
| FR-182 | FR-182-AC-4 | TC-211 |  |
| FR-182 | FR-182-AC-5 | TC-211 |  |
| FR-190 | FR-190-AC-4 | TC-212 |  |
| FR-190 | FR-190-AC-5 | TC-212 |  |
| FR-191 | FR-191-AC-4 | TC-213 |  |
| FR-191 | FR-191-AC-5 | TC-213 |  |
| FR-191 | FR-191-AC-6 | TC-338, TC-339, TC-346 |  |
| FR-192 | FR-192-AC-4 | TC-214 |  |
| FR-192 | FR-192-AC-5 | TC-214 |  |
| FR-192 | FR-192-AC-6 | TC-214 |  |
| FR-192 | FR-192-AC-7 | TC-214 |  |
| FR-193 | FR-193-AC-4 | TC-215 |  |
| FR-193 | FR-193-AC-5 | TC-215 |  |
| FR-194 | FR-194-AC-4 | TC-216 |  |
| FR-194 | FR-194-AC-5 | TC-216 |  |
| FR-195 | FR-195-AC-4 | TC-217 |  |
| FR-195 | FR-195-AC-5 | TC-217 |  |
| FR-195 | FR-195-AC-6 | TC-933 |  |
| FR-196 | FR-196-AC-4 | TC-218 |  |
| FR-196 | FR-196-AC-5 | TC-218 |  |
| FR-197 | FR-197-AC-4 | TC-219 |  |
| FR-197 | FR-197-AC-5 | TC-219 |  |
| FR-197 | FR-197-AC-6 | TC-272 |  |
| FR-197 | FR-197-AC-7 | TC-272 |  |
| FR-197 | FR-197-AC-8 | TC-272 |  |
| FR-197 | FR-197-AC-9 | TC-272 |  |
| FR-197 | FR-197-AC-10 | TC-272 |  |
| FR-197 | FR-197-AC-11 | TC-272 |  |
| FR-197 | FR-197-AC-12 | TC-272 |  |
| FR-197 | FR-197-AC-13 | TC-272 |  |
| FR-197 | FR-197-AC-14 | TC-272 |  |
| FR-197 | FR-197-AC-15 | TC-272 |  |
| FR-197 | FR-197-AC-16 | TC-272 |  |
| FR-197 | FR-197-AC-17 | TC-272 |  |
| FR-197 | FR-197-AC-18 | TC-272 |  |
| FR-197 | FR-197-AC-19 | TC-219 |  |
| FR-197 | FR-197-AC-20 | TC-272 |  |
| FR-197 | FR-197-AC-21 | TC-272 |  |
| FR-197 | FR-197-AC-22 | TC-272 |  |
| FR-320 | FR-320-AC-1 | TC-231 |  |
| FR-320 | FR-320-AC-2 | TC-231 |  |
| FR-320 | FR-320-AC-3 | TC-231 |  |
| FR-321 | FR-321-AC-1 | TC-231 |  |
| FR-321 | FR-321-AC-2 | TC-231 |  |
| FR-321 | FR-321-AC-3 | TC-231 |  |
| FR-321 | FR-321-AC-5 | TC-231 |  |
| FR-322 | FR-322-AC-1 | TC-231 |  |
| FR-322 | FR-322-AC-2 | TC-231 |  |
| FR-322 | FR-322-AC-3 | TC-231 |  |
| FR-322 | FR-322-AC-4 | TC-217 |  |
| FR-322 | FR-322-AC-5 | TC-217 |  |
| FR-322 | FR-322-AC-6 | TC-217 |  |
| FR-322 | FR-322-AC-7 | TC-217 |  |
| FR-322 | FR-322-AC-8 | TC-217 |  |
| FR-322 | FR-322-AC-9 | TC-217 |  |
| FR-322 | FR-322-AC-10 | TC-217 |  |
| FR-322 | FR-322-AC-13 | TC-233 |  |
| FR-322 | FR-322-AC-14 | TC-233 |  |
| FR-322 | FR-322-AC-15 | TC-233 |  |
| FR-322 | FR-322-AC-17 | TC-233 |  |
| FR-322 | FR-322-AC-18 | TC-233 |  |
| FR-322 | FR-322-AC-19 | TC-233 |  |
| FR-322 | FR-322-AC-21 | TC-233 |  |
| FR-322 | FR-322-AC-22 | TC-233 |  |
| FR-322 | FR-322-AC-23 | TC-233 |  |
| FR-322 | FR-322-AC-24 | TC-233 |  |
| FR-322 | FR-322-AC-26 | TC-231 |  |
| FR-322 | FR-322-AC-27 | TC-231 |  |
| FR-322 | FR-322-AC-28 | TC-279 |  |
| FR-322 | FR-322-AC-29 | TC-280 |  |
| FR-322 | FR-322-AC-30 | TC-280 |  |
| FR-322 | FR-322-AC-31 | TC-280 |  |
| FR-322 | FR-322-AC-32 | TC-280 |  |
| FR-322 | FR-322-AC-33 | TC-281 |  |
| FR-322 | FR-322-AC-34 | TC-281 |  |
| FR-322 | FR-322-AC-35 | TC-233 |  |
| FR-322 | FR-322-AC-36 | TC-233 |  |
| FR-322 | FR-322-AC-37 | TC-233 |  |
| FR-322 | FR-322-AC-38 | TC-281 |  |
| FR-322 | FR-322-AC-39 | TC-427 |  |
| FR-322 | FR-322-AC-40 | TC-427 |  |
| FR-322 | FR-322-AC-41 | TC-427 |  |
| FR-322 | FR-322-AC-42 | TC-427 |  |
| FR-322 | FR-322-AC-43 | TC-280 |  |
| FR-322 | FR-322-AC-44 | TC-280 |  |
| FR-322 | FR-322-AC-48 | TC-233 |  |
| FR-322 | FR-322-AC-49 | TC-233 |  |
| FR-322 | FR-322-AC-50 | TC-233 |  |
| FR-322 | FR-322-AC-58 | TC-430 |  |
| FR-322 | FR-322-AC-62 | TC-233 |  |
| FR-322 | FR-322-AC-66 | TC-931 |  |
| FR-322 | FR-322-AC-67 | TC-233 |  |
| FR-340 | FR-340-AC-1 | TC-233 |  |
| FR-340 | FR-340-AC-2 | TC-233 |  |
| FR-340 | FR-340-AC-3 | TC-233 |  |
| FR-340 | FR-340-AC-4 | TC-233 |  |
| FR-340 | FR-340-AC-5 | TC-233 |  |
| FR-340 | FR-340-AC-6 | TC-233 |  |
| FR-340 | FR-340-AC-7 | TC-233 |  |
| FR-340 | FR-340-AC-8 | TC-233 |  |
| FR-340 | FR-340-AC-9 | TC-233 |  |
| FR-340 | FR-340-AC-10 | TC-233 |  |
| FR-340 | FR-340-AC-11 | TC-233 |  |
| FR-340 | FR-340-AC-12 | TC-233 |  |
| FR-340 | FR-340-AC-13 | TC-280 |  |
| FR-340 | FR-340-AC-14 | TC-233 |  |
| FR-340 | FR-340-AC-15 | TC-233 |  |
| FR-341 | FR-341-AC-1 | TC-233 |  |
| FR-341 | FR-341-AC-2 | TC-233 |  |
| FR-341 | FR-341-AC-3 | TC-233 |  |
| FR-341 | FR-341-AC-4 | TC-233 |  |
| FR-341 | FR-341-AC-5 | TC-280 |  |
| FR-341 | FR-341-AC-6 | TC-233 |  |
| FR-341 | FR-341-AC-7 | TC-233 |  |
| FR-341 | FR-341-AC-8 | TC-233 |  |
| FR-341 | FR-341-AC-9 | TC-233 |  |
| FR-341 | FR-341-AC-10 | TC-233 |  |
| FR-342 | FR-342-AC-1 | TC-233 |  |
| FR-342 | FR-342-AC-2 | TC-233 |  |
| FR-342 | FR-342-AC-3 | TC-233 |  |
| FR-342 | FR-342-AC-4 | TC-280 |  |
| FR-342 | FR-342-AC-5 | TC-233 |  |
| FR-342 | FR-342-AC-6 | TC-233 |  |
| FR-440 | FR-440-AC-1 | TC-395 |  |
| FR-440 | FR-440-AC-2 | TC-395 |  |
| FR-440 | FR-440-AC-3 | TC-395 |  |
| FR-440 | FR-440-AC-4 | TC-395 |  |
| FR-440 | FR-440-AC-5 | TC-395 |  |
| FR-440 | FR-440-AC-6 | TC-395 |  |
| FR-441 | FR-441-AC-1 | TC-396 |  |
| FR-441 | FR-441-AC-2 | TC-396 |  |
| FR-441 | FR-441-AC-3 | TC-396 |  |
| interface_004 | interface_004-AC-1 | TC-233 |  |
| interface_004 | interface_004-AC-2 | TC-233 |  |
| FR-323 | FR-323-AC-1 | TC-231 |  |
| FR-323 | FR-323-AC-2 | TC-231 |  |
| FR-323 | FR-323-AC-3 | TC-231 |  |
| FR-323 | FR-323-AC-4 | TC-273 |  |
| FR-323 | FR-323-AC-5 | TC-277 |  |
| FR-323 | FR-323-AC-6 | TC-278 |  |
| FR-323 | FR-323-AC-7 | TC-277 |  |
| FR-323 | FR-323-AC-8 | TC-426 |  |
| FR-323 | FR-323-AC-9 | TC-277 |  |
| FR-324 | FR-324-AC-1 | TC-231 |  |
| FR-324 | FR-324-AC-2 | TC-231 |  |
| FR-324 | FR-324-AC-3 | TC-231 |  |
| FR-325 | FR-325-AC-1 | TC-231 |  |
| FR-325 | FR-325-AC-2 | TC-231 |  |
| FR-325 | FR-325-AC-3 | TC-231 |  |
| FR-326 | FR-326-AC-1 | TC-231 |  |
| FR-326 | FR-326-AC-2 | TC-231 |  |
| FR-326 | FR-326-AC-3 | TC-231 |  |
| FR-327 | FR-327-AC-1 | TC-231 |  |
| FR-327 | FR-327-AC-2 | TC-231 |  |
| FR-327 | FR-327-AC-3 | TC-231 |  |
| FR-328 | FR-328-AC-1 | TC-231 |  |
| FR-328 | FR-328-AC-2 | TC-231 |  |
| FR-328 | FR-328-AC-3 | TC-231 |  |
| FR-329 | FR-329-AC-1 | TC-231 |  |
| FR-329 | FR-329-AC-2 | TC-231 |  |
| FR-329 | FR-329-AC-3 | TC-231 |  |
| FR-330 | FR-330-AC-1 | TC-935 |  |
| FR-330 | FR-330-AC-2 | TC-933, TC-934 |  |
| FR-330 | FR-330-AC-3 | TC-936 |  |
| FR-330 | FR-330-AC-4 | TC-931, TC-937 |  |
| FR-330 | FR-330-AC-5 | TC-932, TC-935 |  |
| FR-330 | FR-330-AC-6 | TC-933 |  |
| FR-330 | FR-330-AC-7 | TC-934 |  |
| FR-330 | FR-330-AC-8 | TC-936 |  |
| FR-330 | FR-330-AC-9 | TC-937 |  |
| FR-331 | FR-331-AC-1 | TC-231 |  |
| FR-331 | FR-331-AC-2 | TC-231 |  |
| FR-331 | FR-331-AC-3 | TC-231 |  |
| FR-331 | FR-331-AC-4 | TC-271 |  |
| FR-331 | FR-331-AC-5 | TC-271 |  |
| FR-331 | FR-331-AC-6 | TC-271 |  |
| FR-331 | FR-331-AC-7 | TC-271 |  |
| FR-331 | FR-331-AC-8 | TC-274 |  |
| FR-331 | FR-331-AC-9 | TC-275 |  |
| FR-331 | FR-331-AC-10 | TC-276 |  |
| FR-332 | FR-332-AC-1 | TC-231 |  |
| FR-332 | FR-332-AC-2 | TC-231 |  |
| FR-332 | FR-332-AC-3 | TC-231 |  |
| FR-333 | FR-333-AC-1 | TC-231 |  |
| FR-333 | FR-333-AC-2 | TC-231 |  |
| FR-333 | FR-333-AC-3 | TC-231 |  |
| FR-334 | FR-334-AC-1 | TC-231 |  |
| FR-334 | FR-334-AC-2 | TC-231 |  |
| FR-334 | FR-334-AC-3 | TC-231 |  |
| FR-336 | FR-336-AC-1 | TC-231 |  |
| FR-336 | FR-336-AC-2 | TC-231 |  |
| FR-336 | FR-336-AC-3 | TC-231 |  |
| FR-337 | FR-337-AC-1 | TC-231 |  |
| FR-337 | FR-337-AC-2 | TC-231 |  |
| FR-337 | FR-337-AC-3 | TC-231 |  |
| FR-338 | FR-338-AC-1 | TC-231 |  |
| FR-338 | FR-338-AC-2 | TC-231 |  |
| FR-338 | FR-338-AC-3 | TC-231 |  |
| FR-339 | FR-339-AC-1 | TC-180 |  |
| FR-339 | FR-339-AC-2 | TC-180 |  |
| FR-339 | FR-339-AC-3 | TC-180 |  |
| FR-339 | FR-339-AC-4 | TC-180 |  |
| FR-360 | FR-360-AC-1 | TC-200 |  |
| FR-360 | FR-360-AC-2 | TC-200 |  |
| FR-360 | FR-360-AC-3 | TC-200 |  |
| FR-360 | FR-360-AC-4 | TC-200 |  |
| FR-360 | FR-360-AC-5 | TC-200 |  |
| FR-346 | FR-346-AC-1 | TC-249 |  |
| FR-346 | FR-346-AC-2 | TC-249 |  |
| FR-346 | FR-346-AC-3 | TC-249 |  |
| FR-346 | FR-346-AC-4 | TC-249 |  |
| FR-346 | FR-346-AC-5 | TC-249, TC-266 |  |
| FR-346 | FR-346-AC-6 | TC-249, TC-266 |  |
| FR-347 | FR-347-AC-1 | TC-250 |  |
| FR-347 | FR-347-AC-2 | TC-250 |  |
| FR-347 | FR-347-AC-3 | TC-250 |  |
| FR-347 | FR-347-AC-4 | TC-250 |  |
| FR-347 | FR-347-AC-5 | TC-250, TC-266 |  |
| FR-347 | FR-347-AC-6 | TC-250, TC-266 |  |
| FR-348 | FR-348-AC-1 | TC-251 |  |
| FR-348 | FR-348-AC-2 | TC-251 |  |
| FR-348 | FR-348-AC-3 | TC-251 |  |
| FR-348 | FR-348-AC-4 | TC-251 |  |
| FR-349 | FR-349-AC-1 | TC-266 |  |
| FR-349 | FR-349-AC-2 | TC-266 |  |
| FR-349 | FR-349-AC-3 | TC-266 |  |
| FR-349 | FR-349-AC-4 | TC-266 |  |
| FR-349 | FR-349-AC-5 | TC-266 |  |
| FR-349 | FR-349-AC-6 | TC-266 |  |
| FR-350 | FR-350-AC-1 | TC-267 |  |
| FR-350 | FR-350-AC-2 | TC-267 |  |
| FR-350 | FR-350-AC-3 | TC-267 |  |
| FR-350 | FR-350-AC-4 | TC-267 |  |
| FR-350 | FR-350-AC-5 | TC-267 |  |
| FR-350 | FR-350-AC-6 | TC-267 |  |
| FR-351 | FR-351-AC-1 | TC-254 |  |
| FR-351 | FR-351-AC-2 | TC-254 |  |
| FR-351 | FR-351-AC-3 | TC-254 |  |
| FR-351 | FR-351-AC-4 | TC-254 |  |
| FR-351 | FR-351-AC-5 | TC-254 |  |
| FR-351 | FR-351-AC-6 | TC-254 |  |
| FR-351 | FR-351-AC-7 | TC-254 |  |
| FR-352 | FR-352-AC-1 | TC-255 |  |
| FR-352 | FR-352-AC-2 | TC-255 |  |
| FR-352 | FR-352-AC-3 | TC-255 |  |
| FR-352 | FR-352-AC-6 | TC-255 |  |
| FR-352 | FR-352-AC-7 | TC-255 |  |
| FR-353 | FR-353-AC-1 | TC-268 |  |
| FR-353 | FR-353-AC-2 | TC-268 |  |
| FR-353 | FR-353-AC-3 | TC-268 |  |
| FR-353 | FR-353-AC-4 | TC-268 |  |
| FR-353 | FR-353-AC-5 | TC-268 |  |
| FR-354 | FR-354-AC-1 | TC-269 |  |
| FR-354 | FR-354-AC-2 | TC-269 |  |
| FR-354 | FR-354-AC-3 | TC-269 |  |
| FR-354 | FR-354-AC-4 | TC-269 |  |
| FR-354 | FR-354-AC-5 | TC-269 |  |
| FR-354 | FR-354-AC-6 | TC-269 |  |
| FR-355 | FR-355-AC-1 | TC-270 |  |
| FR-355 | FR-355-AC-2 | TC-270 |  |
| FR-355 | FR-355-AC-3 | TC-270 |  |
| FR-355 | FR-355-AC-4 | TC-270 |  |
| FR-355 | FR-355-AC-5 | TC-270 |  |
| FR-355 | FR-355-AC-6 | TC-270 |  |
| FR-450 | FR-450-AC-1 | TC-410 |  |
| FR-450 | FR-450-AC-2 | TC-410 |  |
| FR-450 | FR-450-AC-3 | TC-410 |  |
| FR-450 | FR-450-AC-4 | TC-411 |  |
| FR-450 | FR-450-AC-5 | TC-411 |  |
| FR-450 | FR-450-AC-6 | TC-411 |  |
| FR-450 | FR-450-AC-7 | TC-411 |  |
| FR-451 | FR-451-AC-1 | TC-412 |  |
| FR-451 | FR-451-AC-2 | TC-412 |  |
| FR-451 | FR-451-AC-3 | TC-412 |  |
| FR-451 | FR-451-AC-4 | TC-412 |  |
| FR-451 | FR-451-AC-5 | TC-412 |  |
| FR-451 | FR-451-AC-6 | TC-412 |  |
| FR-451 | FR-451-AC-7 | TC-412 |  |
| FR-452 | FR-452-AC-1 | TC-413 |  |
| FR-452 | FR-452-AC-2 | TC-413 |  |
| FR-452 | FR-452-AC-3 | TC-413 |  |
| FR-452 | FR-452-AC-4 | TC-413 |  |
| FR-452 | FR-452-AC-5 | TC-413 |  |
| FR-452 | FR-452-AC-6 | TC-413 |  |
| FR-452 | FR-452-AC-7 | TC-413 |  |
| FR-453 | FR-453-AC-1 | TC-414 |  |
| FR-453 | FR-453-AC-2 | TC-414 |  |
| FR-453 | FR-453-AC-3 | TC-414 |  |
| FR-453 | FR-453-AC-4 | TC-414 |  |
| FR-453 | FR-453-AC-5 | TC-414 |  |
| FR-453 | FR-453-AC-6 | TC-414 |  |
| FR-361 | FR-361-AC-1 | TC-290 |  |
| FR-361 | FR-361-AC-2 | TC-290 |  |
| FR-361 | FR-361-AC-3 | TC-290 |  |
| FR-361 | FR-361-AC-4 | TC-290, TC-291 |  |
| FR-361 | FR-361-AC-5 | TC-290 |  |
| FR-361 | FR-361-AC-6 | TC-290, TC-305 |  |
| FR-361 | FR-361-AC-7 | TC-294 |  |
| FR-362 | FR-362-AC-1 | TC-291 |  |
| FR-362 | FR-362-AC-2 | TC-291 |  |
| FR-362 | FR-362-AC-3 | TC-291 |  |
| FR-362 | FR-362-AC-4 | TC-291 |  |
| FR-362 | FR-362-AC-5 | TC-291, TC-293 |  |
| FR-362 | FR-362-AC-6 | TC-299 |  |
| FR-362 | FR-362-AC-7 | TC-291 |  |
| FR-362 | FR-362-AC-8 | TC-292 |  |
| FR-363 | FR-363-AC-1 | TC-293, TC-294 |  |
| FR-363 | FR-363-AC-2 | TC-298 |  |
| FR-363 | FR-363-AC-3 | TC-294 |  |
| FR-363 | FR-363-AC-4 | TC-294, TC-298 |  |
| FR-363 | FR-363-AC-5 | TC-294 |  |
| FR-363 | FR-363-AC-6 | TC-293 |  |
| FR-363 | FR-363-AC-7 | TC-302 |  |
| FR-363 | FR-363-AC-8 | TC-305 |  |
| FR-363 | FR-363-AC-9 | TC-305 |  |
| FR-364 | FR-364-AC-1 | TC-295, TC-297 |  |
| FR-364 | FR-364-AC-2 | TC-295 |  |
| FR-364 | FR-364-AC-3 | TC-293, TC-295 |  |
| FR-364 | FR-364-AC-4 | TC-295 |  |
| FR-364 | FR-364-AC-5 | TC-295, TC-299 |  |
| FR-364 | FR-364-AC-6 | TC-295, TC-297 |  |
| FR-364 | FR-364-AC-7 | TC-200 |  |
| FR-364 | FR-364-AC-8 | TC-305 |  |
| FR-364 | FR-364-AC-9 | TC-295 |  |
| FR-365 | FR-365-AC-1 | TC-296 |  |
| FR-365 | FR-365-AC-2 | TC-296 |  |
| FR-365 | FR-365-AC-3 | TC-296 |  |
| FR-365 | FR-365-AC-4 | TC-300 |  |
| FR-365 | FR-365-AC-5 | TC-297 |  |
| FR-365 | FR-365-AC-6 | TC-305 |  |
| FR-366 | FR-366-AC-1 | TC-297 |  |
| FR-366 | FR-366-AC-2 | TC-297 |  |
| FR-366 | FR-366-AC-3 | TC-297 |  |
| FR-366 | FR-366-AC-4 | TC-297, TC-302 |  |
| FR-366 | FR-366-AC-5 | TC-297 |  |
| FR-366 | FR-366-AC-6 | TC-297 |  |
| FR-366 | FR-366-AC-7 | TC-297 |  |
| FR-366 | FR-366-AC-8 | TC-305 |  |
| FR-367 | FR-367-AC-1 | TC-238, TC-298 |  |
| FR-367 | FR-367-AC-2 | TC-298 |  |
| FR-367 | FR-367-AC-3 | TC-298 |  |
| FR-367 | FR-367-AC-4 | TC-298 |  |
| FR-367 | FR-367-AC-5 | TC-298 |  |
| FR-367 | FR-367-AC-6 | TC-298 |  |
| FR-367 | FR-367-AC-7 | TC-238, TC-298 |  |
| FR-368 | FR-368-AC-1 | TC-302 |  |
| FR-368 | FR-368-AC-2 | TC-301 |  |
| FR-368 | FR-368-AC-3 | TC-301 |  |
| FR-368 | FR-368-AC-4 | TC-301 |  |
| FR-368 | FR-368-AC-5 | TC-200, TC-301 |  |
| FR-368 | FR-368-AC-6 | TC-301 |  |
| FR-368 | FR-368-AC-7 | TC-298, TC-302 |  |
| FR-369 | FR-369-AC-1 | TC-304 |  |
| FR-369 | FR-369-AC-2 | TC-304 |  |
| FR-369 | FR-369-AC-3 | TC-304 |  |
| FR-369 | FR-369-AC-4 | TC-304 |  |
| FR-369 | FR-369-AC-5 | TC-304 |  |
| FR-370 | FR-370-AC-1 | TC-303 |  |
| FR-370 | FR-370-AC-2 | TC-303 |  |
| FR-370 | FR-370-AC-3 | TC-303 |  |
| FR-370 | FR-370-AC-4 | TC-303 |  |
| FR-370 | FR-370-AC-5 | TC-303 |  |
| FR-370 | FR-370-AC-6 | TC-303 |  |
| FR-370 | FR-370-AC-7 | TC-303 |  |
| FR-090 | FR-090-AC-8 | TC-290 |  |
| FR-091 | FR-091-AC-9 | TC-298 |  |
| FR-092 | FR-092-AC-8 | TC-298 |  |
| FR-181 | FR-181-AC-7 | TC-290 |  |
| FR-048 | FR-048-AC-10 | TC-238 |  |
| FR-255 | FR-255-AC-6 | TC-298 |  |
| FR-250 | FR-250-AC-6 | TC-238, TC-298 |  |
| FR-331 | FR-331-AC-11 | TC-302 |  |
| FR-405 | FR-405-AC-1 | TC-350 |  |
| FR-405 | FR-405-AC-2 | TC-350 |  |
| FR-405 | FR-405-AC-3 | TC-350 |  |
| FR-405 | FR-405-AC-4 | TC-350 |  |
| FR-405 | FR-405-AC-5 | TC-350 |  |
| FR-405 | FR-405-AC-6 | TC-350 |  |
| FR-405 | FR-405-AC-7 | TC-350 |  |
| FR-405 | FR-405-AC-8 | TC-350 |  |
| FR-406 | FR-406-AC-1 | TC-351 |  |
| FR-406 | FR-406-AC-2 | TC-351, TC-352 |  |
| FR-406 | FR-406-AC-3 | TC-351 |  |
| FR-406 | FR-406-AC-4 | TC-352 |  |
| FR-406 | FR-406-AC-5 | TC-352 |  |
| FR-406 | FR-406-AC-6 | TC-351 |  |
| FR-407 | FR-407-AC-1 | TC-353 |  |
| FR-407 | FR-407-AC-2 | TC-353 |  |
| FR-407 | FR-407-AC-3 | TC-353 |  |
| FR-407 | FR-407-AC-4 | TC-353 |  |
| FR-407 | FR-407-AC-5 | TC-353 |  |
| FR-407 | FR-407-AC-6 | TC-353 |  |
| FR-407 | FR-407-AC-7 | TC-353, TC-374 |  |
| FR-407 | FR-407-AC-8 | TC-353, TC-360 |  |
| FR-407 | FR-407-AC-9 | TC-353 |  |
| FR-408 | FR-408-AC-1 | TC-353 |  |
| FR-408 | FR-408-AC-2 | TC-355 |  |
| FR-408 | FR-408-AC-3 | TC-355 |  |
| FR-408 | FR-408-AC-4 | TC-355 |  |
| FR-408 | FR-408-AC-5 | TC-356 |  |
| FR-408 | FR-408-AC-6 | TC-357 |  |
| FR-408 | FR-408-AC-7 | TC-355 |  |
| FR-408 | FR-408-AC-8 | TC-356 |  |
| FR-408 | FR-408-AC-9 | TC-355 |  |
| FR-408 | FR-408-AC-10 | TC-355 |  |
| FR-408 | FR-408-AC-11 | TC-359 |  |
| FR-408 | FR-408-AC-12 | TC-356 |  |
| FR-409 | FR-409-AC-1 | TC-358 |  |
| FR-409 | FR-409-AC-2 | TC-358 |  |
| FR-409 | FR-409-AC-3 | TC-358 |  |
| FR-409 | FR-409-AC-4 | TC-358 |  |
| FR-409 | FR-409-AC-5 | TC-358 |  |
| FR-410 | FR-410-AC-1 | TC-358 |  |
| FR-410 | FR-410-AC-2 | TC-355 |  |
| FR-410 | FR-410-AC-3 | TC-359 |  |
| FR-410 | FR-410-AC-4 | TC-359 |  |
| FR-410 | FR-410-AC-5 | TC-359 |  |
| FR-411 | FR-411-AC-1 | TC-360 |  |
| FR-411 | FR-411-AC-2 | TC-360 |  |
| FR-411 | FR-411-AC-3 | TC-361 |  |
| FR-411 | FR-411-AC-4 | TC-361, TC-374 |  |
| FR-411 | FR-411-AC-5 | TC-361 |  |
| FR-411 | FR-411-AC-6 | TC-362 |  |
| FR-411 | FR-411-AC-7 | TC-365 |  |
| FR-411 | FR-411-AC-8 | TC-365 |  |
| FR-411 | FR-411-AC-9 | TC-360 |  |
| FR-411 | FR-411-AC-10 | TC-361 |  |
| FR-411 | FR-411-AC-11 | TC-361 |  |
| FR-411 | FR-411-AC-12 | TC-364 |  |
| FR-411 | FR-411-AC-13 | TC-360 |  |
| FR-411 | FR-411-AC-14 | TC-360 |  |
| FR-411 | FR-411-AC-15 | TC-365 |  |
| FR-412 | FR-412-AC-1 | TC-363 |  |
| FR-412 | FR-412-AC-2 | TC-362 |  |
| FR-412 | FR-412-AC-3 | TC-363 |  |
| FR-412 | FR-412-AC-4 | TC-363 |  |
| FR-412 | FR-412-AC-5 | TC-363 |  |
| FR-412 | FR-412-AC-6 | TC-363 |  |
| FR-412 | FR-412-AC-7 | TC-363 |  |
| FR-413 | FR-413-AC-1 | TC-364 |  |
| FR-413 | FR-413-AC-2 | TC-364 |  |
| FR-413 | FR-413-AC-3 | TC-364 |  |
| FR-413 | FR-413-AC-4 | TC-364 |  |
| FR-413 | FR-413-AC-5 | TC-364 |  |
| FR-413 | FR-413-AC-6 | TC-364 |  |
| FR-413 | FR-413-AC-7 | TC-364 |  |
| FR-413 | FR-413-AC-8 | TC-374 |  |
| FR-413 | FR-413-AC-9 | TC-364, TC-360 |  |
| FR-414 | FR-414-AC-1 | TC-366 |  |
| FR-414 | FR-414-AC-2 | TC-366 |  |
| FR-414 | FR-414-AC-3 | TC-366 |  |
| FR-414 | FR-414-AC-4 | TC-366 |  |
| FR-414 | FR-414-AC-5 | TC-366 |  |
| FR-414 | FR-414-AC-6 | TC-366 |  |
| FR-415 | FR-415-AC-1 | TC-367 |  |
| FR-415 | FR-415-AC-2 | TC-367 |  |
| FR-415 | FR-415-AC-3 | TC-367 |  |
| FR-415 | FR-415-AC-4 | TC-367 |  |
| FR-415 | FR-415-AC-5 | TC-367 |  |
| FR-416 | FR-416-AC-1 | TC-369 |  |
| FR-416 | FR-416-AC-2 | TC-369 |  |
| FR-416 | FR-416-AC-3 | TC-369 |  |
| FR-416 | FR-416-AC-4 | TC-369 |  |
| FR-416 | FR-416-AC-5 | TC-369, TC-371 |  |
| FR-416 | FR-416-AC-6 | TC-368 |  |
| FR-416 | FR-416-AC-7 | TC-369 |  |
| FR-416 | FR-416-AC-8 | TC-369 |  |
| FR-417 | FR-417-AC-1 | TC-370 |  |
| FR-417 | FR-417-AC-2 | TC-370 |  |
| FR-417 | FR-417-AC-3 | TC-370 |  |
| FR-417 | FR-417-AC-4 | TC-370 |  |
| FR-417 | FR-417-AC-5 | TC-370, TC-371 |  |
| FR-418 | FR-418-AC-1 | TC-371 |  |
| FR-418 | FR-418-AC-2 | TC-371 |  |
| FR-418 | FR-418-AC-3 | TC-371 |  |
| FR-418 | FR-418-AC-4 | TC-371 |  |
| FR-418 | FR-418-AC-5 | TC-371 |  |
| FR-419 | FR-419-AC-1 | TC-372 |  |
| FR-419 | FR-419-AC-2 | TC-372 |  |
| FR-419 | FR-419-AC-3 | TC-372 |  |
| FR-419 | FR-419-AC-4 | TC-372 |  |
| FR-419 | FR-419-AC-5 | TC-372 |  |
| FR-419 | FR-419-AC-6 | TC-372 |  |
| FR-420 | FR-420-AC-1 | TC-374 |  |
| FR-420 | FR-420-AC-2 | TC-374 |  |
| FR-420 | FR-420-AC-3 | TC-374 |  |
| FR-420 | FR-420-AC-4 | TC-374 |  |
| FR-420 | FR-420-AC-5 | TC-374 |  |
| FR-420 | FR-420-AC-6 | TC-374 |  |
| FR-420 | FR-420-AC-7 | TC-374 |  |
| FR-421 | FR-421-AC-1 | TC-374 |  |
| FR-421 | FR-421-AC-2 | TC-374 |  |
| FR-421 | FR-421-AC-3 | TC-374 |  |
| FR-421 | FR-421-AC-4 | TC-374 |  |
| FR-421 | FR-421-AC-5 | TC-374 |  |
| FR-421 | FR-421-AC-6 | TC-374 |  |
| FR-161 | FR-161-AC-13 | TC-369 |  |
| FR-161 | FR-161-AC-14 | TC-369 |  |
| FR-161 | FR-161-AC-15 | TC-370 |  |
| FR-161 | FR-161-AC-16 | TC-369 |  |
| FR-181 | FR-181-AC-9 | TC-354 |  |
| FR-181 | FR-181-AC-10 | TC-354 |  |
| FR-181 | FR-181-AC-11 | TC-369 |  |
| FR-181 | FR-181-AC-12 | TC-369 |  |
| FR-193 | FR-193-AC-6 | TC-373 |  |
| FR-193 | FR-193-AC-7 | TC-373 |  |
| FR-250 | FR-250-AC-7 | TC-368 |  |
| FR-252 | FR-252-AC-6 | TC-368 |  |
| FR-252 | FR-252-AC-7 | TC-368 |  |
| FR-255 | FR-255-AC-7 | TC-368 |  |
| FR-255 | FR-255-AC-8 | TC-368 |  |
| FR-272 | FR-272-AC-15 | TC-350, TC-358, TC-367 |  |
| FR-290 | FR-290-AC-11 | TC-353, TC-370, TC-372 |  |
| FR-290 | FR-290-AC-12 | TC-353 |  |
| FR-331 | FR-331-AC-13 | TC-358 |  |
| FR-331 | FR-331-AC-14 | TC-353 |  |
| FR-331 | FR-331-AC-15 | TC-363, TC-371, TC-372 |  |
| FR-331 | FR-331-AC-16 | TC-358 |  |
| FR-434 | FR-434-AC-1 | TC-388, TC-390 |  |
| FR-434 | FR-434-AC-2 | TC-388 |  |
| FR-434 | FR-434-AC-3 | TC-388 |  |
| FR-434 | FR-434-AC-4 | TC-388 |  |
| FR-434 | FR-434-AC-5 | TC-388 |  |
| FR-435 | FR-435-AC-1 | TC-389 |  |
| FR-435 | FR-435-AC-2 | TC-389 |  |
| FR-435 | FR-435-AC-3 | TC-389 |  |
| FR-435 | FR-435-AC-4 | TC-389 |  |
| FR-435 | FR-435-AC-5 | TC-389 |  |
| FR-435 | FR-435-AC-6 | TC-389 |  |
| FR-436 | FR-436-AC-1 | TC-390 |  |
| FR-436 | FR-436-AC-2 | TC-391 |  |
| FR-436 | FR-436-AC-3 | TC-390 |  |
| FR-436 | FR-436-AC-4 | TC-390 |  |
| FR-436 | FR-436-AC-5 | TC-390 |  |
| FR-437 | FR-437-AC-1 | TC-390 |  |
| FR-437 | FR-437-AC-2 | TC-391 |  |
| FR-437 | FR-437-AC-3 | TC-390 |  |
| FR-437 | FR-437-AC-4 | TC-393 |  |
| FR-437 | FR-437-AC-5 | TC-391 |  |
| FR-437 | FR-437-AC-6 | TC-391 |  |
| FR-438 | FR-438-AC-1 | TC-392 |  |
| FR-438 | FR-438-AC-2 | TC-392 |  |
| FR-438 | FR-438-AC-3 | TC-392 |  |
| FR-438 | FR-438-AC-4 | TC-392 |  |
| FR-438 | FR-438-AC-5 | TC-392 |  |
| FR-439 | FR-439-AC-1 | TC-390, TC-393 |  |
| FR-439 | FR-439-AC-2 | TC-393 |  |
| FR-439 | FR-439-AC-3 | TC-393 |  |
| FR-439 | FR-439-AC-4 | TC-393 |  |
| FR-439 | FR-439-AC-5 | TC-394 |  |
| FR-439 | FR-439-AC-6 | TC-389, TC-394 |  |
| FR-439 | FR-439-AC-7 | TC-394 |  |
| FR-439 | FR-439-AC-8 | TC-393 |  |
| FR-425 | FR-425-AC-1 | TC-375 |  |
| FR-425 | FR-425-AC-2 | TC-375 |  |
| FR-425 | FR-425-AC-3 | TC-375 |  |
| FR-425 | FR-425-AC-4 | TC-375 |  |
| FR-425 | FR-425-AC-5 | TC-375 |  |
| FR-426 | FR-426-AC-1 | TC-376 |  |
| FR-426 | FR-426-AC-2 | TC-377 |  |
| FR-426 | FR-426-AC-3 | TC-378 |  |
| FR-426 | FR-426-AC-4 | TC-376 |  |
| FR-426 | FR-426-AC-5 | TC-376 |  |
| FR-426 | FR-426-AC-6 | TC-376, TC-387 |  |
| FR-426 | FR-426-AC-7 | TC-378 |  |
| FR-426 | FR-426-AC-8 | TC-376 |  |
| FR-426 | FR-426-AC-9 | TC-377, TC-380 |  |
| FR-426 | FR-426-AC-10 | TC-376 |  |
| FR-427 | FR-427-AC-1 | TC-380 |  |
| FR-427 | FR-427-AC-2 | TC-380 |  |
| FR-427 | FR-427-AC-3 | TC-380 |  |
| FR-427 | FR-427-AC-4 | TC-380 |  |
| FR-427 | FR-427-AC-5 | TC-380 |  |
| FR-428 | FR-428-AC-1 | TC-381 |  |
| FR-428 | FR-428-AC-2 | TC-381 |  |
| FR-428 | FR-428-AC-3 | TC-381 |  |
| FR-428 | FR-428-AC-4 | TC-382 |  |
| FR-428 | FR-428-AC-5 | TC-382 |  |
| FR-428 | FR-428-AC-6 | TC-382 |  |
| FR-428 | FR-428-AC-7 | TC-381, TC-382 |  |
| FR-429 | FR-429-AC-1 | TC-376, TC-383 |  |
| FR-429 | FR-429-AC-2 | TC-376 |  |
| FR-429 | FR-429-AC-3 | TC-383 |  |
| FR-429 | FR-429-AC-4 | TC-381, TC-383 |  |
| FR-429 | FR-429-AC-5 | TC-383 |  |
| FR-429 | FR-429-AC-6 | TC-383 |  |
| FR-429 | FR-429-AC-7 | TC-383 |  |
| FR-430 | FR-430-AC-1 | TC-384 |  |
| FR-430 | FR-430-AC-2 | TC-384 |  |
| FR-430 | FR-430-AC-3 | TC-384 |  |
| FR-430 | FR-430-AC-4 | TC-384 |  |
| FR-430 | FR-430-AC-5 | TC-384 |  |
| FR-430 | FR-430-AC-6 | TC-384 |  |
| FR-431 | FR-431-AC-1 | TC-385 |  |
| FR-431 | FR-431-AC-2 | TC-380, TC-385 |  |
| FR-431 | FR-431-AC-3 | TC-385 |  |
| FR-431 | FR-431-AC-4 | TC-385 |  |
| FR-431 | FR-431-AC-5 | TC-385 |  |
| FR-432 | FR-432-AC-1 | TC-386 |  |
| FR-432 | FR-432-AC-2 | TC-386 |  |
| FR-432 | FR-432-AC-3 | TC-386 |  |
| FR-432 | FR-432-AC-4 | TC-386 |  |
| FR-432 | FR-432-AC-5 | TC-386 |  |
| FR-433 | FR-433-AC-1 | TC-387 |  |
| FR-433 | FR-433-AC-2 | TC-387 |  |
| FR-433 | FR-433-AC-3 | TC-384, TC-387 |  |
| FR-433 | FR-433-AC-4 | TC-387 |  |
| FR-433 | FR-433-AC-5 | TC-387 |  |
| FR-433 | FR-433-AC-6 | TC-387 |  |
| FR-433 | FR-433-AC-7 | TC-387 |  |
| FR-460 | FR-460-AC-1 | TC-425 |  |
| FR-460 | FR-460-AC-2 | TC-425 |  |
| FR-460 | FR-460-AC-3 | TC-425 |  |
| FR-460 | FR-460-AC-4 | TC-425 |  |
| FR-460 | FR-460-AC-5 | TC-426 |  |
| FR-461 | FR-461-AC-1 | TC-426 |  |
| FR-461 | FR-461-AC-2 | TC-426 |  |
| FR-461 | FR-461-AC-3 | TC-426 |  |
| FR-461 | FR-461-AC-4 | TC-426 |  |
| FR-461 | FR-461-AC-5 | TC-426 |  |
| FR-461 | FR-461-AC-6 | TC-426 |  |
| FR-461 | FR-461-AC-7 | TC-930 |  |
| FR-462 | FR-462-AC-1 | TC-428 |  |
| FR-462 | FR-462-AC-2 | TC-428 |  |
| FR-462 | FR-462-AC-3 | TC-428 |  |
| FR-462 | FR-462-AC-4 | TC-428 |  |
| FR-462 | FR-462-AC-5 | TC-428 |  |
| FR-462 | FR-462-AC-6 | TC-428 |  |
| FR-331 | FR-331-AC-17 | TC-336, TC-339 |  |
| FR-390 | FR-390-AC-1 | TC-330 |  |
| FR-390 | FR-390-AC-2 | TC-330 |  |
| FR-390 | FR-390-AC-3 | TC-331, TC-332 |  |
| FR-390 | FR-390-AC-4 | TC-333 |  |
| FR-390 | FR-390-AC-5 | TC-332 |  |
| FR-390 | FR-390-AC-6 | TC-330 |  |
| FR-390 | FR-390-AC-7 | TC-330 |  |
| FR-391 | FR-391-AC-1 | TC-331, TC-336 |  |
| FR-391 | FR-391-AC-2 | TC-331, TC-332, TC-334 |  |
| FR-391 | FR-391-AC-3 | TC-333 |  |
| FR-391 | FR-391-AC-4 | TC-332, TC-333, TC-334 |  |
| FR-391 | FR-391-AC-5 | TC-334 |  |
| FR-391 | FR-391-AC-6 | TC-331, TC-336 |  |
| FR-391 | FR-391-AC-7 | TC-334 |  |
| FR-391 | FR-391-AC-8 | TC-334 |  |
| FR-391 | FR-391-AC-9 | TC-336 |  |
| FR-391 | FR-391-AC-10 | TC-349 |  |
| FR-391 | FR-391-AC-11 | TC-349 |  |
| FR-391 | FR-391-AC-12 | TC-336, TC-349 |  |
| FR-391 | FR-391-AC-13 | TC-334, TC-336 |  |
| FR-391 | FR-391-AC-14 | TC-349 |  |
| FR-392 | FR-392-AC-1 | TC-336 |  |
| FR-392 | FR-392-AC-2 | TC-336 |  |
| FR-392 | FR-392-AC-3 | TC-331, TC-336 |  |
| FR-392 | FR-392-AC-4 | TC-331, TC-336 |  |
| FR-392 | FR-392-AC-5 | TC-336 |  |
| FR-392 | FR-392-AC-6 | TC-336 |  |
| FR-392 | FR-392-AC-7 | TC-336 |  |
| FR-392 | FR-392-AC-8 | TC-336 |  |
| FR-392 | FR-392-AC-9 | TC-336 |  |
| FR-393 | FR-393-AC-1 | TC-335, TC-349 |  |
| FR-393 | FR-393-AC-2 | TC-331, TC-335 |  |
| FR-393 | FR-393-AC-3 | TC-335 |  |
| FR-393 | FR-393-AC-5 | TC-335, TC-349 |  |
| FR-393 | FR-393-AC-4 | TC-335 |  |
| FR-394 | FR-394-AC-1 | TC-331, TC-335 |  |
| FR-394 | FR-394-AC-2 | TC-331, TC-332, TC-335 |  |
| FR-394 | FR-394-AC-3 | TC-333, TC-335 |  |
| FR-394 | FR-394-AC-4 | TC-335 |  |
| FR-394 | FR-394-AC-5 | TC-335 |  |
| FR-394 | FR-394-AC-6 | TC-335 |  |
| FR-394 | FR-394-AC-7 | TC-349 |  |
| FR-394 | FR-394-AC-8 | TC-342 |  |
| FR-394 | FR-394-AC-9 | TC-342 |  |
| FR-395 | FR-395-AC-1 | TC-337 |  |
| FR-395 | FR-395-AC-2 | TC-337, TC-338 |  |
| FR-395 | FR-395-AC-3 | TC-338 |  |
| FR-395 | FR-395-AC-4 | TC-339 |  |
| FR-395 | FR-395-AC-5 | TC-340 |  |
| FR-395 | FR-395-AC-6 | TC-340 |  |
| FR-395 | FR-395-AC-7 | TC-341 |  |
| FR-395 | FR-395-AC-8 | TC-341 |  |
| FR-395 | FR-395-AC-9 | TC-337 |  |
| FR-395 | FR-395-AC-10 | TC-340 |  |
| FR-396 | FR-396-AC-1 | TC-342 |  |
| FR-396 | FR-396-AC-2 | TC-342 |  |
| FR-396 | FR-396-AC-3 | TC-342 |  |
| FR-396 | FR-396-AC-4 | TC-342 |  |
| FR-396 | FR-396-AC-5 | TC-342 |  |
| FR-396 | FR-396-AC-6 | TC-342 |  |
| FR-397 | FR-397-AC-1 | TC-343 |  |
| FR-397 | FR-397-AC-2 | TC-343 |  |
| FR-397 | FR-397-AC-3 | TC-343 |  |
| FR-397 | FR-397-AC-4 | TC-343 |  |
| FR-397 | FR-397-AC-5 | TC-343 |  |
| FR-397 | FR-397-AC-6 | TC-337 |  |
| FR-398 | FR-398-AC-1 | TC-344 |  |
| FR-398 | FR-398-AC-2 | TC-344 |  |
| FR-398 | FR-398-AC-3 | TC-337 |  |
| FR-398 | FR-398-AC-4 | TC-344 |  |
| FR-399 | FR-399-AC-1 | TC-338, TC-339, TC-340, TC-342 |  |
| FR-399 | FR-399-AC-2 | TC-339 |  |
| FR-399 | FR-399-AC-3 | TC-339 |  |
| FR-399 | FR-399-AC-4 | TC-343 |  |
| FR-399 | FR-399-AC-5 | TC-340 |  |
| FR-399 | FR-399-AC-6 | TC-338 |  |
| FR-399 | FR-399-AC-7 | TC-345 |  |
| FR-399 | FR-399-AC-8 | TC-348 |  |
| FR-399 | FR-399-AC-9 | TC-348 |  |
| FR-399 | FR-399-AC-10 | TC-348 |  |
| FR-400 | FR-400-AC-1 | TC-345, TC-348 |  |
| FR-400 | FR-400-AC-2 | TC-338, TC-339, TC-343, TC-344 |  |
| FR-400 | FR-400-AC-4 | TC-345, TC-348 |  |
| FR-400 | FR-400-AC-3 | TC-345 |  |
| FR-401 | FR-401-AC-1 | TC-338 |  |
| FR-401 | FR-401-AC-2 | TC-339 |  |
| FR-401 | FR-401-AC-3 | TC-345 |  |
| FR-401 | FR-401-AC-4 | TC-345 |  |
| FR-401 | FR-401-AC-5 | TC-344, TC-345 |  |
| FR-401 | FR-401-AC-6 | TC-343 |  |
| FR-401 | FR-401-AC-7 | TC-347 |  |
| FR-401 | FR-401-AC-8 | TC-348 |  |
| FR-401 | FR-401-AC-9 | TC-345 |  |
| FR-402 | FR-402-AC-1 | TC-346 |  |
| FR-402 | FR-402-AC-2 | TC-346 |  |
| FR-402 | FR-402-AC-3 | TC-346 |  |
| FR-402 | FR-402-AC-4 | TC-343, TC-346 |  |
| FR-403 | FR-403-AC-1 | TC-347 |  |
| FR-403 | FR-403-AC-2 | TC-347 |  |
| FR-403 | FR-403-AC-3 | TC-347 |  |
| FR-403 | FR-403-AC-4 | TC-347 |  |
| FR-375 | FR-375-AC-1 | TC-310 |  |
| FR-375 | FR-375-AC-2 | TC-310 |  |
| FR-375 | FR-375-AC-3 | TC-310 |  |
| FR-375 | FR-375-AC-4 | TC-310 |  |
| FR-375 | FR-375-AC-5 | TC-310 |  |
| FR-375 | FR-375-AC-6 | TC-310 |  |
| FR-375 | FR-375-AC-7 | TC-310 |  |
| FR-375 | FR-375-AC-8 | TC-310 |  |
| FR-376 | FR-376-AC-1 | TC-311 |  |
| FR-376 | FR-376-AC-2 | TC-311 |  |
| FR-376 | FR-376-AC-3 | TC-311, TC-316 |  |
| FR-376 | FR-376-AC-4 | TC-311 |  |
| FR-376 | FR-376-AC-5 | TC-311, TC-316 |  |
| FR-377 | FR-377-AC-1 | TC-311, TC-315 |  |
| FR-377 | FR-377-AC-2 | TC-312, TC-313, TC-316 |  |
| FR-377 | FR-377-AC-3 | TC-313 |  |
| FR-377 | FR-377-AC-4 | TC-312 |  |
| FR-377 | FR-377-AC-5 | TC-313 |  |
| FR-377 | FR-377-AC-6 | TC-313 |  |
| FR-377 | FR-377-AC-7 | TC-312 |  |
| FR-377 | FR-377-AC-8 | TC-312, TC-313 |  |
| FR-378 | FR-378-AC-1 | TC-314 |  |
| FR-378 | FR-378-AC-2 | TC-314 |  |
| FR-378 | FR-378-AC-3 | TC-315 |  |
| FR-378 | FR-378-AC-4 | TC-314 |  |
| FR-378 | FR-378-AC-5 | TC-314 |  |
| FR-378 | FR-378-AC-6 | TC-314 |  |
| FR-378 | FR-378-AC-7 | TC-314 |  |
| FR-378 | FR-378-AC-8 | TC-314 |  |
| FR-379 | FR-379-AC-1 | TC-312, TC-315 |  |
| FR-379 | FR-379-AC-2 | TC-312, TC-313 |  |
| FR-379 | FR-379-AC-3 | TC-315 |  |
| FR-379 | FR-379-AC-4 | TC-315 |  |
| FR-379 | FR-379-AC-5 | TC-315 |  |
| FR-379 | FR-379-AC-6 | TC-315, TC-316, TC-311 |  |
| FR-379 | FR-379-AC-7 | TC-312 |  |
| FR-380 | FR-380-AC-1 | TC-317 |  |
| FR-380 | FR-380-AC-2 | TC-317 |  |
| FR-380 | FR-380-AC-3 | TC-317 |  |
| FR-380 | FR-380-AC-4 | TC-317 |  |
| FR-380 | FR-380-AC-5 | TC-317 |  |
| FR-380 | FR-380-AC-6 | TC-317 |  |
| FR-380 | FR-380-AC-7 | TC-317 |  |
| FR-380 | FR-380-AC-8 | TC-317 |  |
| FR-381 | FR-381-AC-1 | TC-320 |  |
| FR-381 | FR-381-AC-2 | TC-320 |  |
| FR-381 | FR-381-AC-3 | TC-320 |  |
| FR-381 | FR-381-AC-4 | TC-320 |  |
| FR-382 | FR-382-AC-1 | TC-321, TC-327 |  |
| FR-382 | FR-382-AC-2 | TC-327 |  |
| FR-382 | FR-382-AC-3 | TC-320, TC-321 |  |
| FR-382 | FR-382-AC-4 | TC-321 |  |
| FR-382 | FR-382-AC-5 | TC-324 |  |
| FR-382 | FR-382-AC-6 | TC-327 |  |
| FR-383 | FR-383-AC-1 | TC-321 |  |
| FR-383 | FR-383-AC-2 | TC-321 |  |
| FR-383 | FR-383-AC-3 | TC-326 |  |
| FR-383 | FR-383-AC-4 | TC-323 |  |
| FR-383 | FR-383-AC-5 | TC-325 |  |
| FR-383 | FR-383-AC-6 | TC-324 |  |
| FR-383 | FR-383-AC-7 | TC-321, TC-322 |  |
| FR-383 | FR-383-AC-8 | TC-327 |  |
| FR-383 | FR-383-AC-9 | TC-323 |  |
| FR-383 | FR-383-AC-10 | TC-323 |  |
| FR-383 | FR-383-AC-11 | TC-323 |  |
| FR-383 | FR-383-AC-12 | TC-323 |  |
| FR-383 | FR-383-AC-13 | TC-326 |  |
| FR-383 | FR-383-AC-14 | TC-323 |  |
| FR-383 | FR-383-AC-15 | TC-329 |  |
| FR-383 | FR-383-AC-16 | TC-323 |  |
| FR-384 | FR-384-AC-1 | TC-322 |  |
| FR-384 | FR-384-AC-2 | TC-322 |  |
| FR-384 | FR-384-AC-3 | TC-322 |  |
| FR-384 | FR-384-AC-4 | TC-322 |  |
| FR-384 | FR-384-AC-5 | TC-322 |  |
| FR-385 | FR-385-AC-1 | TC-323, TC-327 |  |
| FR-385 | FR-385-AC-2 | TC-323 |  |
| FR-385 | FR-385-AC-3 | TC-321, TC-323, TC-324 |  |
| FR-385 | FR-385-AC-4 | TC-321, TC-323 |  |
| FR-385 | FR-385-AC-5 | TC-324 |  |
| FR-385 | FR-385-AC-6 | TC-323 |  |
| FR-385 | FR-385-AC-7 | TC-327 |  |
| FR-385 | FR-385-AC-8 | TC-325 |  |
| FR-385 | FR-385-AC-9 | TC-323 |  |
| FR-013 | FR-013-AC-8 | TC-326 |  |
| FR-013 | FR-013-AC-9 | TC-326 |  |
| FR-161 | FR-161-AC-12 | TC-328 |  |
| FR-177 | FR-177-AC-6 | TC-318 |  |
| FR-340 | FR-340-AC-16 | TC-326 |  |
| FR-340 | FR-340-AC-17 | TC-326 |  |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
| --- | --- | --- | --- | --- | --- |
| TC-180 | Complete V1 profile bundle accounting | Property | P0 | FR-131 | 🚧 |
| TC-181 | Recursive function termination profiles | Property | P0 | FR-146 | 🚧 |
| TC-182 | Complete package capabilities and identity domains | Property | P0 | FR-132 | 🚧 |
| TC-183 | Uniform extension non-interference | Property | P0 | FR-133 | 🚧 |
| TC-184 | Original source locus preservation | Property | P0 | FR-134 | 🚧 |
| TC-185 | Exact decimal semantics | Property | P0 | FR-140 | 🚧 |
| TC-186 | Text and enum identity semantics | Property | P0 | FR-141 | 🚧 |
| TC-187 | Quantity and unit conversion | Property | P0 | FR-142 | 🚧 |
| TC-188 | Record tuple and recursive values | Property | P0 | FR-143 | 🚧 |
| TC-189 | Collection kind algebra | Property | P0 | FR-144 | 🚧 |
| TC-190 | Collection query and conversion algebra | Property | P0 | FR-145 | 🚧 |
| TC-191 | Total pure function checking | Property | P0 | FR-146 | 🚧 |
| TC-192 | Integer division profiles | Property | P0 | FR-147 | 🚧 |
| TC-193 | IEEE exceptional and rounding profiles | Property | P0 | FR-148 | 🚧 |
| TC-194 | Complete equality matrix | Property | P0 | FR-149 | 🚧 |
| TC-195 | Model normalization provenance | Property | P0 | FR-150 | 🚧 |
| TC-196 | Conformance redefinition and dispatch | Property | P0 | FR-151 | 🚧 |
| TC-938 | Operation bodies are unique by resolved operation identity | Property | P0 | FR-151-AC-16 | 🚧 |
| TC-197 | Systems-model typed binding | Property | P0 | FR-152, FR-208 | 🚧 |
| TC-198 | Closed model lookup | Property | P0 | FR-153 | 🚧 |
| TC-199 | Uncertain revisioned temporal evaluation | Property | P0 | FR-160 | 🚧 |
| TC-200 | Infinite-trace provider boundary | Property | P0 | FR-161, FR-360, FR-095, FR-251, FR-362, FR-364, FR-368 | 🚧 |
| TC-201 | Channel and queue premises | Property | P0 | FR-170 | 🚧 |
| TC-202 | Join loop and dynamic-role control | Property | P0 | FR-171 | 🚧 |
| TC-203 | Choice obligation and capture binding | Property | P0 | FR-172 | 🚧 |
| TC-204 | Complete compensation lifecycle | Property | P0 | FR-173 | 🚧 |
| TC-205 | Sound global projection | Property | P0 | FR-174 | 🚧 |
| TC-206 | Protocol realizability witnesses | Property | P0 | FR-175 | 🚧 |
| TC-207 | Assume-guarantee composition | Property | P0 | FR-176 | 🚧 |
| TC-208 | Protocol refinement relation | Property | P0 | FR-177 | 🚧 |
| TC-209 | Complete reference workflow corpus | Property | P0 | FR-180 | 🚧 |
| TC-210 | Finite simulation reproducibility | Property | P0 | FR-181 | 🚧 |
| TC-211 | Authorized idempotent workflow effects | Property | P0 | FR-182 | 🚧 |
| TC-212 | Model-based generation and shrinking | Property | P0 | FR-190 | 🚧 |
| TC-213 | Relational and hyperproperty analysis | Property | P0 | FR-191 | 🚧 |
| TC-214 | Verification plan dispatch | Property | P0 | FR-192 | 🚧 |
| TC-215 | Continuous hybrid analysis | Property | P0 | FR-193 | 🚧 |
| TC-216 | Bounded synthesis and checking | Property | P0 | FR-194 | 🚧 |
| TC-217 | Complete contract IR lowering | Property | P0 | FR-195 | 🚧 |
| TC-218 | Kani model-domain harnesses | Property | P0 | FR-196 | 🚧 |
| TC-219 | Backend counterexample native replay | Property | P0 | FR-197 | 🚧 |
| TC-220 | Rust lifecycle API contract | Property | P0 | FR-300 | 🚧 |
| TC-221 | CLI lifecycle and exits | Property | P0 | FR-301 | 🚧 |
| TC-222 | Lossless CST round trip | Property | P0 | FR-302 | 🚧 |
| TC-223 | Formatter LSP incremental parity | Property | P0 | FR-303 | 🚧 |
| TC-224 | Model query and rendering determinism | Property | P0 | FR-304 | 🚧 |
| TC-225 | Rust plugin validation and containment | Property | P0 | FR-305 | 🚧 |
| TC-226 | AOT JIT native and cache parity | Property | P0 | FR-306, FR-201 | 🚧 |
| TC-227 | Reusable semantic library resolution | Property | P0 | FR-307 | 🚧 |
| TC-228 | WASM native parity and bounds | Property | P0 | FR-308 | 🚧 |
| TC-229 | Analysis codegen accounting | Property | P0 | FR-309 | 🚧 |
| TC-230 | Semantic assurance traceability | Property | P0 | FR-310 | 🚧 |
| TC-231 | Complete V1 qualification coverage | Property | P0 | FR-311 | 🚧 |
| TC-232 | Counterexample lineage integrity | Property | P0 | FR-312 | 🚧 |
| TC-233 | Published checked-package and scalar contract vectors | Property | P0 | FR-322 | 🚧 |
| TC-234 | Domain package model intake | Property | P0 | FR-154, FR-208 | 🚧 |
| TC-235 | Business construct meaning binding | Property | P0 | FR-208, FR-154, FR-207, FR-047 | 🚧 |
| TC-236 | Frame grant population scope | Property | P0 | FR-013 | 🚧 |
| TC-238 | Unbounded temporal operator grammar admission | Property | P0 | FR-048, FR-090, FR-367, FR-250 | 🚧 |
| TC-239 | Agree replay and incremental assessment on an unsettled temporal result | Property | P0 | FR-114 | 🚧 |
| TC-246 | Unbounded collection type identity and admission | Property | P0 | FR-144 | 🚧 |
| TC-247 | Unbounded population query | Property | P0 | FR-153 | 🚧 |
| TC-248 | Iteration admitted without a maximum or a proved invariant/variant | Property | P0 | FR-228, FR-171 | 🚧 |
| TC-249 | Unbounded quantification negotiation | Property | P0 | FR-346 | 🚧 |
| TC-250 | Inductive verification obligation negotiation | Property | P0 | FR-347 | 🚧 |
| TC-251 | Compositional verification obligation negotiation | Property | P0 | FR-348 | 🚧 |
| TC-262 | Union construction and equality | Property | P0 | FR-143 | 🚧 |
| TC-263 | Union recursion rule | Property | P0 | FR-143 | 🚧 |
| TC-264 | Case exhaustiveness obligations | Property | P0 | FR-146, FR-272 | 🚧 |
| TC-265 | Case evaluation and binding | Property | P0 | FR-146 | 🚧 |
| TC-266 | SMT backend registration and capability advertisement | Property | P0 | FR-349, FR-346, FR-347, FR-147 | 🚧 |
| TC-267 | Verus contract emission from QSL state contracts | Property | P0 | FR-350 | 🚧 |
| TC-268 | Model-to-implementation abstraction relation binding and refusal | Property | P0 | FR-353 | 🚧 |
| TC-269 | Proved settlement certification | Property | P0 | FR-354 | 🚧 |
| TC-270 | SMT/native runtime differential parity corpus | Property | P0 | FR-355 | 🚧 |
| TC-271 | Capability kinds, advertised modes and candidate-set negotiation | Property | P0 | FR-290, FR-331, FR-272 | 🚧 |
| TC-272 | Replay source structural separation | Property | P0 | FR-197 | 🚧 |
| TC-273 | Outcome-to-verdict map fixed per O-16 category | Property | P0 | FR-323 | 🚧 |
| TC-274 | Vacuous Kani proof settles inconclusive | Property | P0 | FR-331 | 🚧 |
| TC-275 | One-arm counterexample entry | Property | P0 | FR-331 | 🚧 |
| TC-276 | Supported-item refusal and unavailability results | Property | P0 | FR-331 | 🚧 |
| TC-277 | Replay byte provision and source digests | Property | P0 | FR-323 | 🚧 |
| TC-278 | Replay envelope members | Property | P0 | FR-323 | 🚧 |
| TC-279 | Model-owner package-scoped node identity | Property | P0 | FR-322 | 🚧 |
| TC-280 | Model-owned member types | Property | P0 | FR-322 | 🚧 |
| TC-281 | Model reference equality and field projection | Property | P0 | FR-322 | 🚧 |
| TC-282 | Duplicate backend identity registration | Property | P0 | FR-290, FR-272 | 🚧 |
| TC-410 | Abstraction relation declaration syntax and binding forms | Property | P0 | FR-450 | 🚧 |
| TC-411 | Abstraction relation declaration refusals | Property | P0 | FR-450, FR-353 | 🚧 |
| TC-412 | Checked-package abstraction relation node | Property | P0 | FR-451, FR-353 | 🚧 |
| TC-413 | Spec-versioning refinement cases | Property | P0 | FR-452 | 🚧 |
| TC-414 | Profile-layering refinement cases | Property | P0 | FR-453 | 🚧 |
| TC-290 | Model behaviours read as temporal traces | Property | P0 | FR-361, FR-090, FR-181 | 🚧 |
| TC-291 | Weak and strong fairness on conformance lassos | Property | P0 | FR-362, FR-361 | 🚧 |
| TC-292 | Fairness over a supplied trace is a missing fairness premise | Property | P0 | FR-362, FR-161 | 🚧 |
| TC-293 | Every-behaviour verdicts on the ConfigVersion subject | Property | P0 | FR-363, FR-362, FR-364 | 🚧 |
| TC-294 | Verdict strength, bound reached, undecided and stopped runs | Property | P0 | FR-363, FR-361 | 🚧 |
| TC-295 | Temporal counterexample wire, replay and refusals | Property | P0 | FR-364 | 🚧 |
| TC-296 | Canonical counterexample of an explicit-state analysis | Property | P0 | FR-365 | 🚧 |
| TC-297 | Deadlock-freedom vectors | Property | P0 | FR-366, FR-364, FR-365 | 🚧 |
| TC-298 | Interval operators under infinite-trace | Property | P0 | FR-367, FR-363, FR-091, FR-092, FR-250, FR-255, FR-368 | 🚧 |
| TC-299 | Strong fairness on the contended mutex | Property | P0 | FR-362, FR-364 | 🚧 |
| TC-300 | Strong-fairness refinement past one level | Property | P0 | FR-365, FR-362 | 🚧 |
| TC-301 | Temporal negotiation, provider commitments and fairness kinds | Property | P0 | FR-368, FR-161 | 🚧 |
| TC-302 | Temporal request members and obligation identity | Property | P0 | FR-368, FR-331, FR-363, FR-366 | 🚧 |
| TC-303 | Checked-package temporal clause body and operation identities | Property | P0 | FR-370, FR-322 | 🚧 |
| TC-304 | Strong-fairness hint on a reproduced refutation | Property | P0 | FR-369 | 🚧 |
| TC-305 | Undefined evaluation refutes a model claim | Property | P0 | FR-361, FR-363, FR-364, FR-365, FR-366 | 🚧 |
| TC-395 | Checked-package union, union value and case nodes | Property | P0 | FR-440, FR-322 | 🚧 |
| TC-396 | Union member key | Property | P0 | FR-441 | 🚧 |
| TC-397 | Union canonical key and union collection elements | Property | P0 | FR-144 | 🚧 |
| TC-398 | Case result type | Property | P0 | FR-146 | 🚧 |
| TC-399 | Union construction and case accounting | Property | P0 | FR-143, FR-146 | 🚧 |
| TC-400 | Supplied union values and the value profile | Property | P0 | FR-143 | 🚧 |
| TC-401 | Pre-call precondition observation | Property | P0 | FR-046, FR-208 | 🚧 |
| TC-350 | Probabilistic declarations admission | Property | P0 | FR-405, FR-272 | 🚧 |
| TC-351 | Step probability, Markov condition and measure per initial state | Property | P0 | FR-406, FR-407 | 🚧 |
| TC-352 | Every-scheduler and fair-scheduler semantics | Property | P0 | FR-406 | 🚧 |
| TC-353 | Probabilistic claim forms, confidence parameters and evidence routing | Property | P0 | FR-407, FR-408, FR-290, FR-331 | 🚧 |
| TC-354 | Weighted sampler, choice index and trace index | Property | P0 | FR-181 | 🚧 |
| TC-355 | Okamoto and SPRT decisions | Property | P0 | FR-408, FR-410 | 🚧 |
| TC-356 | Regenerative long-run fractions and statistical budgets | Property | P0 | FR-408 | 🚧 |
| TC-357 | Bonferroni over tests and symmetry reduction of tests | Property | P0 | FR-408 | 🚧 |
| TC-358 | Measured axis, result combination and pipeline gating | Property | P0 | FR-409, FR-410, FR-331, FR-272 | 🚧 |
| TC-359 | Sampled witness wire and replay | Property | P0 | FR-410, FR-408, FR-409 | 🚧 |
| TC-360 | Exact finite-horizon values | Property | P0 | FR-411 | 🚧 |
| TC-361 | Exact unbounded reachability, expected rewards and fairness sets | Property | P0 | FR-411 | 🚧 |
| TC-362 | Exact long-run fractions | Property | P0 | FR-411, FR-412 | 🚧 |
| TC-363 | Probability certificate checker | Property | P0 | FR-412, FR-331 | 🚧 |
| TC-364 | Probabilistic counterexample replay | Property | P0 | FR-413, FR-411 | 🚧 |
| TC-365 | Exact limits, precision budget and partial evidence | Property | P0 | FR-411 | 🚧 |
| TC-366 | Window aggregates in monitors | Property | P0 | FR-414 | 🚧 |
| TC-367 | Clock and timed model declarations | Property | P0 | FR-415, FR-272 | 🚧 |
| TC-368 | Timed profile, clock bindings and interval openness | Property | P0 | FR-090, FR-250, FR-252, FR-255, FR-416 | 🚧 |
| TC-369 | Timed semantics, verdicts and replay | Property | P0 | FR-416, FR-161, FR-181 | 🚧 |
| TC-370 | Time-lock freedom and vacuity | Property | P0 | FR-417, FR-161, FR-290 | 🚧 |
| TC-371 | Zone certificate checker | Property | P0 | FR-418, FR-416, FR-417, FR-331 | 🚧 |
| TC-372 | Task-set schedulability | Property | P0 | FR-419, FR-290, FR-331 | 🚧 |
| TC-373 | Hybrid solver verdict map | Property | P0 | FR-193 | 🚧 |
| TC-374 | Stochastic delays, timed sampling and exact digital checking | Property | P0 | FR-420, FR-421, FR-411, FR-407 | 🚧 |
| TC-375 | Protocol state key and coalescing | Property | P0 | FR-425 | 🚧 |
| TC-376 | Two branches and a join: states, deadlock and repair | Property | P0 | FR-426, FR-429 | 🚧 |
| TC-377 | Join policies at zero, below, at and past threshold | Property | P0 | FR-426 | 🚧 |
| TC-378 | Channels under each ordering, delivery and overflow policy | Property | P0 | FR-426 | 🚧 |
| TC-379 | Split-shipment causal interleavings | Property | P0 | FR-052 | 🚧 |
| TC-380 | Compensation in a protocol subject | Property | P0 | FR-427, FR-431, FR-426 | 🚧 |
| TC-381 | Per-trigger activation and the instance budget | Property | P0 | FR-428, FR-429 | 🚧 |
| TC-382 | Replicated role spawn, action and lifetimes | Property | P0 | FR-428 | 🚧 |
| TC-383 | Protocol terminal member, deadlock opt-out and wait causes | Property | P0 | FR-429 | 🚧 |
| TC-384 | Scheduler fairness and adversarial scheduling | Property | P0 | FR-430, FR-433 | 🚧 |
| TC-385 | Counted-step interval distance and protocol anchors | Property | P0 | FR-431 | 🚧 |
| TC-386 | Explicit refinement rows for every protocol step class | Property | P0 | FR-432 | 🚧 |
| TC-387 | Protocol counterexample wire and replay | Property | P0 | FR-433, FR-426 | 🚧 |
| TC-388 | Memory model selection, nesting and identity | Property | P0 | FR-434 | 🚧 |
| TC-389 | Memory access classification, orderings and fences | Property | P0 | FR-435, FR-439 | 🚧 |
| TC-390 | Store buffering under sc, tso and ra | Property | P0 | FR-434, FR-436, FR-437, FR-439 | 🚧 |
| TC-391 | Litmus conformance vectors under sc, tso and ra | Property | P0 | FR-436, FR-437 | 🚧 |
| TC-392 | Race-freedom item and race counterexamples | Property | P0 | FR-438 | 🚧 |
| TC-393 | Memory budgets, bound-limited states and verdicts | Property | P0 | FR-439, FR-437 | 🚧 |
| TC-394 | Weak-memory counterexample wire and replay | Property | P0 | FR-439 | 🚧 |
| TC-425 | Deep inputs settle on their content at every stage | Property | P0 | FR-460, FR-146 | 🚧 |
| TC-426 | Limit outcomes name the limit, its value and its setting | Property | P0 | FR-460, FR-461, FR-323 | 🚧 |
| TC-930 | A byte-emitting operation raises its output-byte budget | Property | P0 | FR-461-AC-7 | 🚧 |
| TC-931 | ContractPackage has one identity-keyed semantic graph | Property | P0 | FR-330-AC-4, FR-322-AC-66 | 🚧 |
| TC-427 | The v2 body grammar fixes package depth by schema | Property | P0 | FR-322 | 🚧 |
| TC-428 | The plugin wire frames, identifies and settles every plugin run | Property | P0 | FR-462 | 🚧 |
| TC-429 | Value paths root and step through every witness location | Property | P0 | FR-207, FR-351 | 🚧 |
| TC-430 | Node-key preimage integers keep one exact spelling at every magnitude | Property | P0 | FR-322 | 🚧 |
| TC-330 | State-graph claim syntax, refusals and routing | Property | P0 | FR-390, FR-290 | 🚧 |
| TC-331 | Possible claims on the ConfigVersion subject | Property | P0 | FR-390, FR-391, FR-392, FR-393, FR-394 | 🚧 |
| TC-332 | Always-possible claims on the game subject | Property | P0 | FR-390, FR-391, FR-394 | 🚧 |
| TC-333 | Unique-path claims and path counting | Property | P0 | FR-390, FR-391, FR-394 | 🚧 |
| TC-334 | State-graph partial runs, verdict map and reductions | Property | P0 | FR-391 | 🚧 |
| TC-335 | State-graph evidence wire and replay disagreements | Property | P0 | FR-393, FR-394, FR-181 | 🚧 |
| TC-336 | Witness sampling for possibility claims | Property | P0 | FR-392, FR-391, FR-331 | 🚧 |
| TC-337 | Hyperproperty and step-relation syntax, binding and refusals | Property | P0 | FR-395, FR-397, FR-398 | 🚧 |
| TC-338 | Noninterference in lockstep (universal hyperproperty) | Property | P0 | FR-395, FR-399, FR-400, FR-401, FR-191 | 🚧 |
| TC-339 | ∀∃ safety hyperproperty and the witness-set limit | Property | P0 | FR-395, FR-399, FR-400, FR-401, FR-191, FR-331 | 🚧 |
| TC-340 | Hyperproperty vacuity, terminal stutter and deadlock items | Property | P0 | FR-395, FR-399 | 🚧 |
| TC-341 | Per-variable fairness and mixed interval bodies | Property | P0 | FR-395 | 🚧 |
| TC-342 | Hyperproperty form classification, unsupported forms and single-existential claims | Property | P0 | FR-396, FR-399, FR-290, FR-394 | 🚧 |
| TC-343 | Step relations: model claims and code claims | Property | P0 | FR-397, FR-399, FR-400, FR-401, FR-402, FR-290 | 🚧 |
| TC-344 | Projection-aligned hyperproperty | Property | P0 | FR-398, FR-400, FR-401 | 🚧 |
| TC-345 | Hyper counterexample wire and replay refusals | Property | P0 | FR-400, FR-401, FR-399 | 🚧 |
| TC-346 | Hyperproperty claim body and obligation identity | Property | P0 | FR-402, FR-191 | 🚧 |
| TC-347 | Hyperproperty reductions and copy-swap symmetry | Property | P0 | FR-403, FR-401 | 🚧 |
| TC-348 | Undefined evaluation refutes a hyperproperty | Property | P0 | FR-399, FR-400, FR-401 | 🚧 |
| TC-349 | Undefined evaluation refutes a state-graph claim | Property | P0 | FR-391, FR-393, FR-394 | 🚧 |
| TC-310 | Refinement declaration admission | Property | P0 | FR-375 | 🚧 |
| TC-311 | Population-valued mapping expressions | Property | P0 | FR-376, FR-377 | 🚧 |
| TC-312 | Counter refinement vectors | Property | P0 | FR-377, FR-379 | 🚧 |
| TC-313 | Refinement initial, stutter and terminal-state vectors | Property | P0 | FR-377, FR-379 | 🚧 |
| TC-314 | Refinement history and hidden-field vectors | Property | P0 | FR-378 | 🚧 |
| TC-315 | Refinement undetermined mappings and replay | Property | P0 | FR-379, FR-378, FR-377 | 🚧 |
| TC-316 | Ring-buffer to queue data refinement | Property | P0 | FR-376, FR-377, FR-379 | 🚧 |
| TC-317 | Claim transfer through a refinement | Property | P0 | FR-380 | 🚧 |
| TC-318 | Protocol refinement over the model-refinement step rules | Property | P0 | FR-177 | 🚧 |
| TC-320 | Symmetric population authoring checks | Property | P0 | FR-381, FR-382 | 🚧 |
| TC-321 | Symmetry reduction vectors | Property | P0 | FR-382, FR-383, FR-385 | 🚧 |
| TC-322 | Annotated quotient each-fairness vectors | Property | P0 | FR-384, FR-383 | 🚧 |
| TC-323 | Partial-order reduction vectors | Property | P0 | FR-383, FR-385 | 🚧 |
| TC-324 | State constraint vectors | Property | P0 | FR-382, FR-383, FR-385 | 🚧 |
| TC-325 | Partial-order reduction under fairness | Property | P0 | FR-383, FR-385 | 🚧 |
| TC-326 | Receiver-scoped frames and footprints | Property | P0 | FR-013, FR-340, FR-383 | 🚧 |
| TC-327 | Reduced request identity, negotiation and records | Property | P0 | FR-382, FR-383, FR-385, FR-181 | 🚧 |
| TC-328 | Stutter invariance of since and triggered | Property | P0 | FR-161 | 🚧 |
| TC-329 | Reduced and unreduced runs agree over the conformance corpus | Property | P0 | FR-383 | 🚧 |

## Integration Test Matrix

| Integration ID | Purpose | Target Project | Type | Test Cases |
| --- | --- | --- | --- | --- |
| IT-070 | Artifact bundle to checked package | quire-rs extraction + FCD extraction-frontend + quire-spec-language | library | TC-195, TC-196, TC-197, TC-198, TC-234, TC-235 |
| IT-071 | Composed domain execution | quire-integration + domain runtimes | library | TC-209, TC-210 |
| IT-072 | Revisioned observation repair | quire-observation + temporal/protocol | library | TC-199, TC-204 |
| IT-073 | Protocol analysis suite | quire-spec-language + quire-protocol | library | TC-205–TC-208 |
| IT-074 | Verification method dispatch | quire-verification + method providers | library/process | TC-212–TC-216 |
| IT-075 | Contract backend native replay | Contract IR/codegen/runtime + QSL | library/tool | TC-209, TC-217–TC-219 |
| IT-076 | API/CLI/WASM/editor parity | QSL tool surfaces | library/process | TC-220–TC-224, TC-228 |
| IT-077 | Plugin/JIT containment | Rust provider host | library/process | TC-225, TC-226 |
| IT-078 | Workflow to evidence | quire-integration + observation + verification | library/process | TC-211, TC-230–TC-232 |

## Model-domain capability coverage note

V1-TYPE-021 (FR-011), V1-TYPE-022 (FR-009) and V1-EXPR-017 through V1-EXPR-022
(FR-009, FR-012, FR-006, FR-013) keep their primary requirements. Under the
selected `quire.model.complete/v1` definition their inheritance-dependent
meaning is covered by TC-043 and TC-046 together with the TC-196 vectors R02
(frame with a subtype create), R08 and D06 (redefined operation pre- and
postconditions) and the TC-198 vectors L01, L05 and L07 (identity, population
and anchors under inheritance); V1-TYPE-024 is resolved to FR-043, which
supersedes FR-010's parent reachability.
