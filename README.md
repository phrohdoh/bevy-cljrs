# [bevy] + [cljrs]

<details><summary>State as of 25/July/2021</summary>

```shell
cargo build
```
```
   Compiling bevy_cljrs v0.0.0
error[E0277]: `RefCell<HashMap<rust_clojure::symbol::Symbol, Arc<rust_clojure::value::Value>>>` cannot be shared between threads safely
 --> src/lib.rs:8:6
  |
8 | impl Plugin for ScriptViaClojurePlugin {
  |      ^^^^^^ `RefCell<HashMap<rust_clojure::symbol::Symbol, Arc<rust_clojure::value::Value>>>` cannot be shared between threads safely
  | 
 ::: ~/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_app-0.5.0/src/plugin.rs:8:32
  |
8 | pub trait Plugin: Any + Send + Sync {
  |                                ---- required by this bound in `bevy::prelude::Plugin`
  |
  = help: within `Environment`, the trait `Sync` is not implemented for `RefCell<HashMap<rust_clojure::symbol::Symbol, Arc<rust_clojure::value::Value>>>`
  = note: required because it appears within the type `Environment`
  = note: required because of the requirements on the impl of `Sync` for `Arc<Environment>`
  = note: required because it appears within the type `Repl`
  = note: required because of the requirements on the impl of `Sync` for `Arc<Repl>`
note: required because it appears within the type `ScriptViaClojurePlugin`
 --> src/lib.rs:7:12
  |
7 | pub struct ScriptViaClojurePlugin { repl: CljRepl }
  |            ^^^^^^^^^^^^^^^^^^^^^^

error[E0277]: `RefCell<rust_clojure::symbol::Symbol>` cannot be shared between threads safely
 --> src/lib.rs:8:6
  |
8 | impl Plugin for ScriptViaClojurePlugin {
  |      ^^^^^^ `RefCell<rust_clojure::symbol::Symbol>` cannot be shared between threads safely
  | 
 ::: ~/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_app-0.5.0/src/plugin.rs:8:32
  |
8 | pub trait Plugin: Any + Send + Sync {
  |                                ---- required by this bound in `bevy::prelude::Plugin`
  |
  = help: within `Environment`, the trait `Sync` is not implemented for `RefCell<rust_clojure::symbol::Symbol>`
  = note: required because it appears within the type `EnvironmentVal`
  = note: required because it appears within the type `Environment`
  = note: required because of the requirements on the impl of `Sync` for `Arc<Environment>`
  = note: required because it appears within the type `Repl`
  = note: required because of the requirements on the impl of `Sync` for `Arc<Repl>`
note: required because it appears within the type `ScriptViaClojurePlugin`
 --> src/lib.rs:7:12
  |
7 | pub struct ScriptViaClojurePlugin { repl: CljRepl }
  |            ^^^^^^^^^^^^^^^^^^^^^^

error[E0277]: `RefCell<HashMap<rust_clojure::symbol::Symbol, Namespace>>` cannot be shared between threads safely
 --> src/lib.rs:8:6
  |
8 | impl Plugin for ScriptViaClojurePlugin {
  |      ^^^^^^ `RefCell<HashMap<rust_clojure::symbol::Symbol, Namespace>>` cannot be shared between threads safely
  | 
 ::: ~/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_app-0.5.0/src/plugin.rs:8:32
  |
8 | pub trait Plugin: Any + Send + Sync {
  |                                ---- required by this bound in `bevy::prelude::Plugin`
  |
  = help: within `Environment`, the trait `Sync` is not implemented for `RefCell<HashMap<rust_clojure::symbol::Symbol, Namespace>>`
  = note: required because it appears within the type `Namespaces`
  = note: required because it appears within the type `EnvironmentVal`
  = note: required because it appears within the type `Environment`
  = note: required because of the requirements on the impl of `Sync` for `Arc<Environment>`
  = note: required because it appears within the type `Repl`
  = note: required because of the requirements on the impl of `Sync` for `Arc<Repl>`
note: required because it appears within the type `ScriptViaClojurePlugin`
 --> src/lib.rs:7:12
  |
7 | pub struct ScriptViaClojurePlugin { repl: CljRepl }
  |            ^^^^^^^^^^^^^^^^^^^^^^

error[E0277]: `RefCell<rust_clojure::protocols::IPersistentMap>` cannot be shared between threads safely
 --> src/lib.rs:8:6
  |
8 | impl Plugin for ScriptViaClojurePlugin {
  |      ^^^^^^ `RefCell<rust_clojure::protocols::IPersistentMap>` cannot be shared between threads safely
  | 
 ::: ~/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_app-0.5.0/src/plugin.rs:8:32
  |
8 | pub trait Plugin: Any + Send + Sync {
  |                                ---- required by this bound in `bevy::prelude::Plugin`
  |
  = help: within `rust_clojure::value::Value`, the trait `Sync` is not implemented for `RefCell<rust_clojure::protocols::IPersistentMap>`
  = note: required because it appears within the type `rust_clojure::var::Var`
  = note: required because it appears within the type `rust_clojure::value::Value`
  = note: required because of the requirements on the impl of `Send` for `Arc<rust_clojure::value::Value>`
  = note: required because it appears within the type `MapEntry`
  = note: required because it appears within the type `rust_clojure::persistent_list_map::PersistentListMap`
  = note: required because it appears within the type `rust_clojure::symbol::Symbol`
  = note: required because of the requirements on the impl of `Send` for `RefCell<rust_clojure::symbol::Symbol>`
  = note: required because it appears within the type `EnvironmentVal`
  = note: required because it appears within the type `Environment`
  = note: required because of the requirements on the impl of `Sync` for `Arc<Environment>`
  = note: required because it appears within the type `Repl`
  = note: required because of the requirements on the impl of `Sync` for `Arc<Repl>`
note: required because it appears within the type `ScriptViaClojurePlugin`
 --> src/lib.rs:7:12
  |
7 | pub struct ScriptViaClojurePlugin { repl: CljRepl }
  |            ^^^^^^^^^^^^^^^^^^^^^^

error[E0277]: `RefCell<Arc<rust_clojure::value::Value>>` cannot be shared between threads safely
 --> src/lib.rs:8:6
  |
8 | impl Plugin for ScriptViaClojurePlugin {
  |      ^^^^^^ `RefCell<Arc<rust_clojure::value::Value>>` cannot be shared between threads safely
  | 
 ::: ~/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_app-0.5.0/src/plugin.rs:8:32
  |
8 | pub trait Plugin: Any + Send + Sync {
  |                                ---- required by this bound in `bevy::prelude::Plugin`
  |
  = help: within `rust_clojure::value::Value`, the trait `Sync` is not implemented for `RefCell<Arc<rust_clojure::value::Value>>`
  = note: required because it appears within the type `rust_clojure::var::Var`
  = note: required because it appears within the type `rust_clojure::value::Value`
  = note: required because of the requirements on the impl of `Send` for `Arc<rust_clojure::value::Value>`
  = note: required because it appears within the type `MapEntry`
  = note: required because it appears within the type `rust_clojure::persistent_list_map::PersistentListMap`
  = note: required because it appears within the type `rust_clojure::symbol::Symbol`
  = note: required because of the requirements on the impl of `Send` for `RefCell<rust_clojure::symbol::Symbol>`
  = note: required because it appears within the type `EnvironmentVal`
  = note: required because it appears within the type `Environment`
  = note: required because of the requirements on the impl of `Sync` for `Arc<Environment>`
  = note: required because it appears within the type `Repl`
  = note: required because of the requirements on the impl of `Sync` for `Arc<Repl>`
note: required because it appears within the type `ScriptViaClojurePlugin`
 --> src/lib.rs:7:12
  |
7 | pub struct ScriptViaClojurePlugin { repl: CljRepl }
  |            ^^^^^^^^^^^^^^^^^^^^^^

error[E0277]: `(dyn rust_clojure::ifn::IFn + 'static)` cannot be shared between threads safely
 --> src/lib.rs:8:6
  |
8 | impl Plugin for ScriptViaClojurePlugin {
  |      ^^^^^^ `(dyn rust_clojure::ifn::IFn + 'static)` cannot be shared between threads safely
  | 
 ::: ~/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_app-0.5.0/src/plugin.rs:8:32
  |
8 | pub trait Plugin: Any + Send + Sync {
  |                                ---- required by this bound in `bevy::prelude::Plugin`
  |
  = help: the trait `Sync` is not implemented for `(dyn rust_clojure::ifn::IFn + 'static)`
  = note: required because of the requirements on the impl of `Sync` for `Arc<(dyn rust_clojure::ifn::IFn + 'static)>`
  = note: required because it appears within the type `rust_clojure::value::Value`
  = note: required because of the requirements on the impl of `Send` for `Arc<rust_clojure::value::Value>`
  = note: required because it appears within the type `MapEntry`
  = note: required because it appears within the type `rust_clojure::persistent_list_map::PersistentListMap`
  = note: required because it appears within the type `rust_clojure::symbol::Symbol`
  = note: required because of the requirements on the impl of `Send` for `RefCell<rust_clojure::symbol::Symbol>`
  = note: required because it appears within the type `EnvironmentVal`
  = note: required because it appears within the type `Environment`
  = note: required because of the requirements on the impl of `Sync` for `Arc<Environment>`
  = note: required because it appears within the type `Repl`
  = note: required because of the requirements on the impl of `Sync` for `Arc<Repl>`
note: required because it appears within the type `ScriptViaClojurePlugin`
 --> src/lib.rs:7:12
  |
7 | pub struct ScriptViaClojurePlugin { repl: CljRepl }
  |            ^^^^^^^^^^^^^^^^^^^^^^

error[E0277]: `(dyn rust_clojure::ifn::IFn + 'static)` cannot be sent between threads safely
 --> src/lib.rs:8:6
  |
8 | impl Plugin for ScriptViaClojurePlugin {
  |      ^^^^^^ `(dyn rust_clojure::ifn::IFn + 'static)` cannot be sent between threads safely
  | 
 ::: ~/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_app-0.5.0/src/plugin.rs:8:32
  |
8 | pub trait Plugin: Any + Send + Sync {
  |                                ---- required by this bound in `bevy::prelude::Plugin`
  |
  = help: the trait `Send` is not implemented for `(dyn rust_clojure::ifn::IFn + 'static)`
  = note: required because of the requirements on the impl of `Sync` for `Arc<(dyn rust_clojure::ifn::IFn + 'static)>`
  = note: required because it appears within the type `rust_clojure::value::Value`
  = note: required because of the requirements on the impl of `Send` for `Arc<rust_clojure::value::Value>`
  = note: required because it appears within the type `MapEntry`
  = note: required because it appears within the type `rust_clojure::persistent_list_map::PersistentListMap`
  = note: required because it appears within the type `rust_clojure::symbol::Symbol`
  = note: required because of the requirements on the impl of `Send` for `RefCell<rust_clojure::symbol::Symbol>`
  = note: required because it appears within the type `EnvironmentVal`
  = note: required because it appears within the type `Environment`
  = note: required because of the requirements on the impl of `Sync` for `Arc<Environment>`
  = note: required because it appears within the type `Repl`
  = note: required because of the requirements on the impl of `Sync` for `Arc<Repl>`
note: required because it appears within the type `ScriptViaClojurePlugin`
 --> src/lib.rs:7:12
  |
7 | pub struct ScriptViaClojurePlugin { repl: CljRepl }
  |            ^^^^^^^^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0277`.
error: could not compile `bevy_cljrs` due to 7 previous errors
```

</details>

[bevy]: https://bevyengine.org
[cljrs]: https://github.com/phrohdoh/cljrs
