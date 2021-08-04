// imports /////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

use std::rc::Rc;
use bevy::prelude::*;
use cljrs::{
    environment::Environment as Env,
    ifn::IFn,
    keyword::Keyword,
    maps::MapEntry,
    persistent_list::PersistentList,
    persistent_list_map::PersistentListMap,
    repl::Repl,
    rust_core::LoadFileFn,
    symbol::Symbol,
    value::{Value, ToValue},
};

use crate::units::{
    SelectableComponent,
    UnitComponent,
};

// macros (must be defined before usage) ///////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

macro_rules! sym {
    (@ns $ns:expr,$n:expr) => {::cljrs::symbol::Symbol::intern_with_ns($ns,$n)};
    (@ns @val $ns:expr,$n:expr) => {sym!($ns,$n).to_value()};
    (@ns @rc-val $ns:expr,$n:expr) => {sym!($ns,$n).to_rc_value()};
    //
    ($n:expr) => {::cljrs::symbol::Symbol::intern($n)};
    (@val $n:expr) => {sym!($n).to_value()};
    (@rc-val $n:expr) => {sym!($n).to_rc_value()};
}

// plugins / add to bevy app ///////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

pub(crate) trait ScriptableApp {
    fn add_scripting(
        &mut self,
        env: Rc<Env>,
        cfg: ScriptingConfig,
    ) -> &mut Self;
}
impl ScriptableApp for AppBuilder {
    fn add_scripting(
        &mut self,
        env: Rc<Env>,
        cfg: ScriptingConfig,
    ) -> &mut Self {
        self
           .insert_non_send_resource(env)
           .insert_resource(StartupRepl(cfg.startup_repl))
           .add_startup_system(sys_startup.system())
           ;
        self
           .insert_resource(ToggleUnitSelectionTimer({
               let mut t = Timer::from_seconds(1.0, true);
               t.pause();
               t
            }))
           .add_system(sys_toggle_unit_selection_on_timer.system())
           ;
        self
    }
}

// resources ///////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub(crate) struct ScriptingConfig {
    pub startup_repl: bool,
}

#[derive(Debug)]
pub(crate) struct StartupRepl(bool);

#[derive(Debug)]
pub(crate) struct ToggleUnitSelectionTimer(pub Timer);

// systems /////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

fn sys_startup(
    rc_env: NonSend<Rc<Env>>,
    startup_repl: Res<StartupRepl>,
    mut toggle_unit_selection_timer: ResMut<ToggleUnitSelectionTimer>,
) {
    let load_file_fn = LoadFileFn::new(rc_env.clone());

    let env = rc_env.as_ref();
    let user_ns = sym!("user");
    env.change_or_create_namespace(&user_ns);

    _load_file(&load_file_fn, format!(
       "{cargo_manifest_dir}/src/scripts/{file_stem}.clj",
       cargo_manifest_dir = env!("CARGO_MANIFEST_DIR"),
       file_stem = "some-user-script",
    ));

    if startup_repl.0 {
        let repl = Repl::new(rc_env.clone());
        let i = std::io::stdin();
        let i = i.lock();
        let o = std::io::stdout();
        let mut o = o.lock();

        use std::io::Write as _;
        let _ = writeln!(o, "to end this REPL session, thus launching the app, enter    :repl/quit");

        repl.run(i, o);
    }

    env.change_or_create_namespace(&user_ns);

    _invoke_clj_sym_as_fn(
        Box::new(Repl::new),
        rc_env.clone(),
        sym!(@ns "user", "on-startup"),
        vec![],
    );

    toggle_unit_selection_timer.0.unpause();
}

fn sys_toggle_unit_selection_on_timer(
    time: Res<Time>,
    mut timer: ResMut<ToggleUnitSelectionTimer>,
    mut query: Query<(Entity, &UnitComponent, &mut SelectableComponent)>,
    rc_env: NonSend<Rc<Env>>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let when = time.time_since_startup();
    let rs_dbg_curr_file = file!();

    for (ent, unit, mut selectable) in query.iter_mut() {
        let player_id_val = Value::I32(unit.player_id.0 as i32);
        let unit_type_id_val = Value::I32(unit.unit_type_id.0 as i32);
        let args = vec![
            Value::PersistentListMap(
                PersistentListMap::Map(
                    Rc::new(PersistentListMap::Map(
                        Rc::new(PersistentListMap::Empty),
                        MapEntry {
                            key: Value::Keyword(Keyword::intern("unit-ty-id")).to_rc_value(),
                            val: unit_type_id_val.to_rc_value(),
                        },
                    )),
                    MapEntry {
                        key: Value::Keyword(Keyword::intern_with_ns("unit", "player-id")).to_rc_value(),
                        val: player_id_val.to_rc_value(),
                    },
                ),
            ),
        ];

        let should_toggle_selection = _invoke_clj_sym_as_fn(
            Box::new(Repl::new),
            rc_env.clone(),
            sym!(@ns "user", "toggle-selection?"),
            args,
        );

        if let Value::Boolean(true) = should_toggle_selection {
            selectable.is_selected = !selectable.is_selected;
        }
    }
}

// unorganized /////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// load: exhaustively sequentially read and eval
fn _load_file(
    f: &LoadFileFn,
    file_path: String,
) -> Value {
    <LoadFileFn as IFn>::invoke(f, vec![
        Value::String(file_path).to_rc_value(),
    ])
}

fn _invoke_clj_sym_as_fn(
    repl_provider: Box<dyn FnOnce(Rc<Env>) -> Repl>,
    rc_env: Rc<Env>,
    fn_sym: Symbol,
    arg_vals: Vec<Value>,
) -> Value {
    let list_val = _as_clj_list_val(fn_sym, arg_vals);
    let repl = repl_provider(rc_env);
    repl.eval(&list_val)
}

fn _as_clj_list_val(
    fn_sym: Symbol,
    arg_vals: Vec<Value>,
) -> Value {
    _as_clj_list(fn_sym, arg_vals).to_value()
}

fn _as_clj_list(
    fn_sym: Symbol,
    arg_vals: Vec<Value>,
) -> PersistentList {
    let mut vals_to_make_list_from = vec![fn_sym.to_value()];
    vals_to_make_list_from.extend(arg_vals.into_iter());

    let prepend = |plst, val| cljrs::persistent_list::cons(val, plst);
    let list = vals_to_make_list_from.into_iter()
        .rfold(PersistentList::Empty, prepend);

    list
}

pub(crate) fn create_custom_cljrs_env() -> Rc<Env> {
    let env = Env::new_main_environment();
    let rc_env = Rc::new(env);
    Env::populate_with_clojure_core(rc_env.clone());


    let custom_stuff_clj_ns_name = "bevy-cljrs";
    let custom_stuff_clj_ns_sym = cljrs::symbol::Symbol::intern(custom_stuff_clj_ns_name);
    rc_env.change_or_create_namespace(&custom_stuff_clj_ns_sym);

    #[derive(Debug, Clone)]
    pub struct HiFn {}
    impl ToValue for HiFn { fn to_value(&self) -> Value { Value::IFn(Rc::new(self.clone())) } }
    impl cljrs::ifn::IFn for HiFn {
        fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
            println!("(hi ,,,) args: {:?}", args);
            Value::String("Clojure string from Rust-impl'd, exposed-to-Clojure function".into())
        }
    }
    let hi_fn = HiFn{}.to_rc_value();

    rc_env.insert_into_namespace(
        &custom_stuff_clj_ns_sym,
        cljrs::symbol::Symbol::intern("hi"),
        hi_fn,
    );


    rc_env.change_or_create_namespace(&cljrs::symbol::Symbol::intern("user"));
    rc_env
}
