// scoped variables: the lattice as a generic. verbatim library — full Rust,
// outside the chain machinery. see scope.md.

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    Local,
    User,
    Group,
    Global,
}

impl Scope {
    pub fn tag(self) -> &'static str {
        match self {
            Scope::Local => "local",
            Scope::User => "user",
            Scope::Group => "group",
            Scope::Global => "global",
        }
    }
}

pub struct Var<T> {
    pub scope: Scope,
    pub key: &'static str,
    _t: std::marker::PhantomData<T>,
}

impl<T: serde::Serialize + serde::de::DeserializeOwned + Default> Var<T> {
    pub const fn new(scope: Scope, key: &'static str) -> Self {
        Var { scope, key, _t: std::marker::PhantomData }
    }
    pub const fn local(key: &'static str) -> Self { Self::new(Scope::Local, key) }
    pub const fn user(key: &'static str) -> Self { Self::new(Scope::User, key) }
    pub const fn group(key: &'static str) -> Self { Self::new(Scope::Group, key) }
    pub const fn global(key: &'static str) -> Self { Self::new(Scope::Global, key) }

    /// read the local replica (Default when absent)
    pub fn get(&self, state: &serde_json::Value) -> T {
        serde_json::from_value(state[self.key].clone()).unwrap_or_default()
    }

    /// write the local replica only (no sync)
    pub fn put(&self, state: &mut serde_json::Value, v: &T) {
        state[self.key] = serde_json::to_value(v).unwrap_or(serde_json::Value::Null);
    }

    /// register semantics: write locally, sync last-write-wins
    pub fn set(&self, state: &mut serde_json::Value, v: &T) {
        self.put(state, v);
        if self.scope != Scope::Local {
            queue_var_op(state, "VarSet", self.scope, self.key,
                         serde_json::to_value(v).unwrap_or(serde_json::Value::Null));
        }
    }
}

impl Var<u64> {
    /// counter semantics: bump locally, sync the operation (concurrent adds all count)
    pub fn add(&self, state: &mut serde_json::Value, delta: u64) {
        let v = self.get(state) + delta;
        self.put(state, &v);
        self.add_op(state, delta);
    }

    /// the value was already applied locally (optimistic) — ship just the op
    pub fn add_op(&self, state: &mut serde_json::Value, delta: u64) {
        if self.scope != Scope::Local {
            queue_var_op(state, "VarAdd", self.scope, self.key,
                         serde_json::json!(delta));
        }
    }
}

fn queue_var_op(state: &mut serde_json::Value, op: &str, scope: Scope,
                key: &str, value: serde_json::Value) {
    if !state["_send"].is_array() {
        state["_send"] = serde_json::json!([]);
    }
    state["_send"].as_array_mut().expect("_send is array").push(serde_json::json!({
        "type": op,
        "data": { "scope": scope.tag(), "key": key, "value": value }
    }));
}
