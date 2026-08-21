// the context slot family: a slot's attributes live in its type, so a
// mis-declared slot is a rustc error rather than a review finding.
// verbatim library — full Rust, outside the chain machinery. see context.md.
//
// the presence of `pub struct Slot<` in a composed verbatim library is the
// linker's hook: it switches on `.slots` collection and Context emission.

/// where a slot's value lives, and therefore who can see it.
pub trait SlotScope {
    const TAG: &'static str;
}

/// how two replicas of a slot reconcile when they disagree.
pub trait SlotMerge {
    const TAG: &'static str;
}

/// whether a slot's absence means "look at the layer above" or "I own this".
pub trait SlotInherit {
    const TAG: &'static str;
}

pub struct ScopeGlobal;
pub struct ScopeGroup;
pub struct ScopeUser;
pub struct ScopeDevice;

impl SlotScope for ScopeGlobal {
    const TAG: &'static str = "global";
}
impl SlotScope for ScopeGroup {
    const TAG: &'static str = "group";
}
impl SlotScope for ScopeUser {
    const TAG: &'static str = "user";
}
impl SlotScope for ScopeDevice {
    const TAG: &'static str = "device";
}

pub struct MergeLastWrite;
pub struct MergeCrdtSum;
pub struct MergeBetter;
pub struct MergeNone;

impl SlotMerge for MergeLastWrite {
    const TAG: &'static str = "last-write";
}
impl SlotMerge for MergeCrdtSum {
    const TAG: &'static str = "crdt-sum";
}
impl SlotMerge for MergeBetter {
    const TAG: &'static str = "better";
}
impl SlotMerge for MergeNone {
    const TAG: &'static str = "none";
}

pub struct Inherit;
pub struct Own;

impl SlotInherit for Inherit {
    const TAG: &'static str = "inherit";
}
impl SlotInherit for Own {
    const TAG: &'static str = "own";
}

/// the one lifecycle rule this rung enforces in the type system: device is the
/// leaf of the overlay chain, so a device-scoped slot has nothing to inherit
/// from. `ScopeDevice` implements `Permits<Own>` and nothing else, which makes
/// `device, ..., inherit` fail to compile rather than fail in review.
pub trait Permits<I: SlotInherit>: SlotScope {}

impl Permits<Inherit> for ScopeGlobal {}
impl Permits<Own> for ScopeGlobal {}
impl Permits<Inherit> for ScopeGroup {}
impl Permits<Own> for ScopeGroup {}
impl Permits<Inherit> for ScopeUser {}
impl Permits<Own> for ScopeUser {}
impl Permits<Own> for ScopeDevice {}

/// one feature-scoped slot: a value plus its lifecycle, carried in the type.
pub struct Slot<T, S, M, I>
where
    S: Permits<I>,
    M: SlotMerge,
    I: SlotInherit,
{
    pub value: T,
    _attrs: std::marker::PhantomData<(S, M, I)>,
}

impl<T, S, M, I> Slot<T, S, M, I>
where
    S: Permits<I>,
    M: SlotMerge,
    I: SlotInherit,
{
    pub const fn new(value: T) -> Self {
        Slot { value, _attrs: std::marker::PhantomData }
    }

    /// the declared attributes, recoverable at runtime for sync and snapshot
    /// machinery that has to walk slots generically.
    pub fn attrs(&self) -> (&'static str, &'static str, &'static str) {
        (S::TAG, M::TAG, I::TAG)
    }
}

impl<T: Clone, S, M, I> Clone for Slot<T, S, M, I>
where
    S: Permits<I>,
    M: SlotMerge,
    I: SlotInherit,
{
    fn clone(&self) -> Self {
        Slot::new(self.value.clone())
    }
}
