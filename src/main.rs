use std::rc::Rc;
use cljrs::{
    repl::Repl,
    environment::Environment as Env,
    value::{Value,ToValue},
};
use bevy::prelude::*;
use bevy_cljrs::*;

fn main() {
    let repl = build_cljrs_repl();
    //
    {
        let clj_src: &str = "(bevy-cljrs/heart [:pre,:app,:build])";
        println!("{}", repl.eval_readable(&mut clj_src.as_bytes()));
    }

    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(UnitManagerPlugin)
        .insert_non_send_resource(repl)
        .add_startup_system(do_something_with_cljrs_repl.system())
        .run();
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

    #[derive(Debug, Clone)]
    pub struct HeartFn {}
    impl ToValue for HeartFn { fn to_value(&self) -> Value { Value::IFn(Rc::new(self.clone())) } }
    impl cljrs::ifn::IFn for HeartFn {
        fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
            eprintln!("(heart ,,,) args: {:#?}", args);
            Value::String("Clojure string from Rust-implemented, exposed-to-Clojure function".into())
        }
    }
    //
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

fn do_something_with_cljrs_repl(
    mut _cmds: Commands,
    repl: NonSend<Repl>,
) {
    let clj_src: &str = "(bevy-cljrs/heart [:in :bevy :ecs])";
    println!("{}", repl.eval_readable(&mut clj_src.as_bytes()));
}
