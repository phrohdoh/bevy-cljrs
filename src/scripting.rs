use std::rc::Rc;
use bevy::prelude::*;
use cljrs::{
    repl::Repl,
    environment::Environment as Env,
    value::{Value,ToValue},
};

pub struct ClojureScriptingPlugin;
impl Plugin for ClojureScriptingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_non_send_resource(build_cljrs_repl())
           .add_startup_system(do_something_with_cljrs_repl.system());
    }
}

fn do_something_with_cljrs_repl(
    mut _cmds: Commands,
    repl: NonSend<Repl>,
) {
    let clj_src: &str = "(clojure.core/print-string (bevy-cljrs/heart [:in :bevy :ecs]))";
    println!("{}", repl.eval_readable(&mut clj_src.as_bytes()));
}

fn build_cljrs_repl() -> Repl {
    let env = build_cljrs_env();
    Repl::new(env)
}
fn build_cljrs_env() -> Rc<Env> {
    let env = Env::new_main_environment();
    let rc_env = Rc::new(env);
    Env::populate_with_clojure_core(rc_env.clone());

    ////////////////////////////////////////////////////////////////////////////
    // custom stuff
    ////////////////////////////////////////////////////////////////////////////
    let custom_stuff_clj_ns_name = "bevy-cljrs";
    let custom_stuff_clj_ns_sym = cljrs::symbol::Symbol::intern(custom_stuff_clj_ns_name);
    rc_env.change_or_create_namespace(&custom_stuff_clj_ns_sym);

    let heart_fn = HeartFn{}.to_rc_value();
    let heart_fn_sym_name = "heart";
    rc_env.insert_into_namespace(
        &custom_stuff_clj_ns_sym,
        cljrs::symbol::Symbol::intern(heart_fn_sym_name),
        heart_fn,
    );

    ////////////////////////////////////////////////////////////////////////////
    // custom stuff - end
    ////////////////////////////////////////////////////////////////////////////

    rc_env.change_or_create_namespace(&cljrs::symbol::Symbol::intern("user"));
    rc_env
}
#[derive(Debug, Clone)]
pub struct HeartFn {}
impl ToValue for HeartFn { fn to_value(&self) -> Value { Value::IFn(Rc::new(self.clone())) } }
impl cljrs::ifn::IFn for HeartFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        eprintln!("(heart ,,,) args: {:#?}", args);
        Value::String("Clojure string from Rust-implemented, exposed-to-Clojure function\n".into())
    }
}
