use super::*;

fn ctx() -> Hash {
    hash_bytes(b"ctx-A")
}
fn policy() -> PolicyId {
    hash_bytes(b"policy-v1")
}
fn provenance(source: &str, parents: Vec<ObjectId>) -> Provenance {
    Provenance {
        parent_object_ids: parents,
        evidence_hash: hash_bytes(b"evidence"),
        source_system: source.into(),
    }
}
fn object(state: AuthorityState) -> MemoryObject {
    let mut o = MemoryObject::new("ns", "uuid-1", b"hello", 1, ctx(), 1, provenance("user", vec![]));
    o.authority_state = state;
    o
}
fn setup(state: AuthorityState) -> (Core, MemoryObject) {
    let o = object(state);
    let mut c = Core::new(policy());
    c.insert(o.clone()).unwrap();
    (c, o)
}
fn cap(
    c: &mut Core,
    subject: SubjectId,
    id: ObjectId,
    ops: impl IntoIterator&lt;Item = Operation&gt;,
) -> CapabilityId {
    c.issue_capability(
        subject,
        [id].into_iter().collect(),
        ops.into_iter().collect(),
        ctx(),
        100,
        100,
    )
}
fn subject() -> SubjectId {
    hash_bytes(b"model")
}

&lt;|open|>_reserved_&lt;|sep|>_reserved&lt;|close|>_reserved
