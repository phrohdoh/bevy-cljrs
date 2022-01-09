// imports /////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

use std::rc::Rc;
use bevy::prelude::*;
use crate::console;
use cljrs::{
    environment::Environment,
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

pub const STARTUP_SYSTEM_LABEL: &str = "scripting_startup";

pub(crate) fn add_scripting(
    app: &mut App,
    env: Env,
    cfg: Configuration,
) {
    app.insert_non_send_resource(env)
       .insert_resource(cfg)
       .add_startup_system(
           startup_bevy
            .label(STARTUP_SYSTEM_LABEL),
        )
       ;
    app.add_event::<Eval>()
       .add_event::<Evaled>()
       .add_system(eval)
       ;

    app.insert_resource(ToggleUnitSelectionTimer({
            let mut t = Timer::from_seconds(1.0, true);
            t.pause();
            t
        }))
        .add_system(sys_toggle_unit_selection_on_timer)
        ;
}

// events //////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Eval(pub(crate) String);

#[derive(Debug)]
pub struct Evaled(/*pub(crate) Value*/ pub(crate) String);

// resources ///////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub(crate) struct Configuration {
    pub pre_window_repl: bool,
}

//#[derive(Debug)]
//pub(crate) struct StartupRepl(bool);

#[derive(Debug)]
pub(crate) struct ToggleUnitSelectionTimer(pub Timer);

// systems /////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

fn startup_bevy(
    env: NonSend<Env>,
    cfg: Res<Configuration>,
    mut toggle_unit_selection_timer: ResMut<ToggleUnitSelectionTimer>,
) {
    let env = env.clone();
    let cfg = &cfg;
    let toggle_unit_selection_timer = &mut toggle_unit_selection_timer;
    startup(env, cfg, toggle_unit_selection_timer)
}

fn startup(
    env: Env,
    cfg: &Configuration,
    toggle_unit_selection_timer: &mut ToggleUnitSelectionTimer,
) {
    env.change_or_create_namespace(&Symbol::intern("user"));

    _invoke_clj_sym_as_fn(
        Box::new(Repl::new),
        env.clone(),
        Symbol::intern("clojure.core/load-file"),
        vec![
            Value::String(format!(
                "{cargo_manifest_dir}/src/scripts/{file_stem}.clj",
                cargo_manifest_dir = env!("CARGO_MANIFEST_DIR"),
                file_stem = "some-user-script",
            )),
        ],
    );

    if cfg.pre_window_repl {
        let repl = Repl::new(env.clone());
        let i = std::io::stdin();
        let i = i.lock();
        let o = std::io::stdout();
        let mut o = o.lock();
        //
        use std::io::Write as _;
        let _ = writeln!(o, "to end this REPL session, thus launching the app, enter    :repl/quit");
        //
        repl.run(i, o);
    }

    _invoke_clj_sym_as_fn(
        Box::new(Repl::new),
        env.clone(),
        Symbol::intern("user/on-startup"),
        vec![],
    );

    env.change_or_create_namespace(&Symbol::intern("user"));

    toggle_unit_selection_timer.0.unpause();
}

fn _startup(
    rc_env: NonSend<Env>,
    //startup_repl: Res<StartupRepl>,
    mut toggle_unit_selection_timer: ResMut<ToggleUnitSelectionTimer>,
    console_state: NonSend<console::StateRef>,
) {
    console_state.as_ref().borrow_mut().scrollback_line_prompt_prefix = rc_env.get_current_namespace_name().into();

    let load_file_fn = LoadFileFn::new(rc_env.clone());

    let env = rc_env.as_ref();
    let user_ns = sym!("user");
    env.change_or_create_namespace(&user_ns);

    _load_file(&load_file_fn, format!(
       "{cargo_manifest_dir}/src/scripts/{file_stem}.clj",
       cargo_manifest_dir = env!("CARGO_MANIFEST_DIR"),
       file_stem = "some-user-script",
    ));

    /*
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
    */

    ////////////////////////////////////////////////////////////////////////////
    /*
    env.insert_into_namespace(
        &cljrs::symbol::Symbol::intern("console"),
        cljrs::symbol::Symbol::intern("clear-scrollback"),
        {
            #[derive(Debug, Clone)]
            pub struct Fn {
                state: console::StateRef,
            }
            impl ToValue for Fn {
                fn to_value(&self) -> Value {
                    Value::IFn(Rc::new(self.clone()))
                }
            }
            impl cljrs::ifn::IFn for Fn {
                fn invoke(&self, _args: Vec<Rc<Value>>) -> Value {
                    (*self.state.borrow_mut()).scrollback.clear();
                    Value::Nil
                }
            }
            Fn { state: console_state.clone() }.to_rc_value()
        },
    );
    */
    ////////////////////////////////////////////////////////////////////////////

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
    rc_env: NonSend<Env>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    //let when = time.time_since_startup();
    //let rs_dbg_curr_file = file!();

    for (_ent, unit, mut selectable) in query.iter_mut() {
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

pub type Env = Rc<Environment>;
pub type EnvRef<'e> = &'e Environment;

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
    repl_provider: Box<dyn FnOnce(Env) -> Repl>,
    env: Env,
    fn_sym: Symbol,
    arg_vals: Vec<Value>,
) -> Value {
    let list_val = _as_clj_list_val(fn_sym, arg_vals);
    let repl = repl_provider(env);
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

pub(crate) fn create_custom_cljrs_env() -> Env {
    let env = Environment::clojure_core_environment();

    bind(&env, "bevy-cljrs", "hi", {
        #[derive(Debug, Clone)]
        pub struct HiFn {}
        impl ToValue for HiFn {
            fn to_value(&self) -> Value {
                Value::IFn(Rc::new(self.clone()))
            }
        }
        impl cljrs::ifn::IFn for HiFn {
            fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
                println!("(bevy-cljrs/hi ,,,) args: {:?}", args);
                Value::String("Clojure string from Rust-impl'd, exposed-to-Clojure function".into())
            }
        }
        HiFn{}.to_rc_value()
    });

    env.change_or_create_namespace(&Symbol::intern("user"));
    env
}

/*
fn eval_and_display_result_on_console(
    env: NonSend<Env>,
    mut eval_evt_rdr: EventReader<Eval>,
    mut disp_evt_wrtr: EventWriter<console::Display>,
) {
    let repl = Repl::new(env.clone());
    for eval_evt in eval_evt_rdr.iter() {
        let to_eval = eval_evt.0.trim().as_bytes();
        if let Some(val) = repl.eval_readable(to_eval) {
            let val_disp = val.to_string_explicit();
            disp_evt_wrtr.send(console::Display(val_disp));
        }
    }
}
*/

fn eval(
    env: NonSend<Env>,
    mut eval_evt_rdr: EventReader<Eval>,
    mut evaled_evt_wrtr: EventWriter<Evaled>, // todo: cljrs Rc -> Arc
) {
    let repl = Repl::new(env.clone());
    for eval_evt in eval_evt_rdr.iter() {
        let to_eval = eval_evt.0.trim().as_bytes();
        if let Some(val) = repl.eval_readable(to_eval) {
            //evaled_evt_wrtr.send(Evaled(val));
            evaled_evt_wrtr.send(Evaled(val.to_string_explicit()));
        }
    }
}

// utilities, helpers, etc. ////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

pub fn bind(
    env: EnvRef,
    ns: &str,
    n: &str,
    rc_val: Rc<Value>,
) {
    let ns_sym = sym!(ns);
    let n_sym = sym!(n);
    env.insert_into_namespace(&ns_sym, n_sym, rc_val);
}
