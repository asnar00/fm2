// the enabled gate's read primitive. verbatim library — full Rust, because the
// predicate is a closure over the generated Context and the chain parser cannot
// express a `impl Fn(&Context) -> bool` parameter type. see enabled.md.
//
// fm:context-gate — the linker's hook: this token in a composed node's source
// is what asks for the implicit `enabled` var on every composed node, the
// `<node>_on()` conjunction chain, and the gates themselves. Untick this node
// and none of the three is emitted.

/// ask the context whether a node is effectively on.
///
/// The read goes through `with_context`, so inside a turn it is the turn's
/// FROZEN view: an edit that lands while an event is being processed cannot
/// flip a gate halfway through that event. That is the boundary law, and this
/// is the one function that has to honour it — every generated gate calls here
/// rather than reaching for the held context itself.
///
/// `f` is a generated `|c| c.<node>_on()`: a conjunction of typed bool fields
/// that rustc resolves and inlines. Nothing here compares a path string.
pub fn gate_open(f: impl Fn(&Context) -> bool) -> bool {
    with_context(|c| f(c))
}
