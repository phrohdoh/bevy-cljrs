pub use bevy_egui::{self, egui::{self, Align, ScrollArea, TextEdit}};

use std::{cell::RefCell, rc::Rc};
use bevy::{input::keyboard::KeyboardInput, prelude::*};
use bevy_egui::{EguiContext, EguiPlugin};
use crate::scripting;

#[derive(Debug, Clone, Copy)]
pub enum Key {
    KeyCode(KeyCode),
    //ScanCode(u32),
}

#[derive(Debug, Clone)]
pub struct Configuration {
    pub title: Option<String>,
    pub left_pos: f32,
    pub top_pos: f32,
    pub height: f32,
    pub width: f32,
    pub is_collapsable: bool,
    pub submit_input_key: egui::Key,
    pub toggle_open_key: Key,
}

#[derive(Debug, Clone, Default)]
pub struct State {
    pub scrollback_line_prompt: Option<String>,
    pub scrollback_line_prompt_prefix: Option<String>,
    pub input_buf: String,
    pub is_open: bool,
    pub scrollback: Vec<String>,
}
pub(crate) type StateRef = Rc<RefCell<State>>;
pub(crate) fn state_ref<S: Into<State>>(state: S) -> StateRef {
    let state = state.into();
    Rc::new(RefCell::new(state))
}

#[derive(Debug)]
pub struct Input(pub String);

#[derive(Debug)]
pub struct Display(pub String);

#[derive(Debug)]
pub struct ClearDisplay;

fn was_key_just_pressed(
    keyboard_input: &KeyboardInput,
    key: Key,
) -> bool {
    if !keyboard_input.state.is_pressed() {
        return false;
    }
    match key {
        Key::KeyCode(k) => keyboard_input.key_code.map_or(false, |pressed_key| pressed_key == k),
        //ConsoleKey::ScanCode(k) => keyboard_input.scan_code == k,
    }
}

fn console_egui_window(
    mut keyboard_input_evt_rdr: EventReader<KeyboardInput>,
    egui_ctx: Res<EguiContext>,
    cfg: Res<Configuration>,
    state: NonSend<StateRef>,
    mut input_evt_wrtr: EventWriter<Input>,
) {
    let mut state = state.as_ref().borrow_mut();
    for keyboard_input in keyboard_input_evt_rdr.iter() {
        let was_toggle_open_key_just_pressed = was_key_just_pressed(keyboard_input, cfg.toggle_open_key);
        if was_toggle_open_key_just_pressed {
            state.is_open = !state.is_open;
        }
    }

    let scroll_height = cfg.height - 30.0;
    let mut is_open = state.is_open;
    egui::Window::new(cfg.title.clone().unwrap_or("".into()))
        .title_bar(cfg.title.is_some())
        .open(&mut is_open)
        .collapsible(cfg.is_collapsable)
        .frame(egui::Frame {
            corner_radius: 0.0,
            margin: egui::Vec2::new(0.0, 0.0),
            fill: egui::Color32::from_rgb(64, 64, 64),
            ..Default::default()
        })
        .fixed_rect(egui::Rect::from_two_pos(
            egui::Pos2::new(cfg.left_pos, cfg.top_pos),
            egui::Pos2::new(cfg.left_pos + cfg.width, cfg.top_pos + cfg.height),
        ))
        .show(egui_ctx.ctx(), |ui| {
            ui.set_min_height(cfg.height);
            ui.set_min_width(cfg.width);

            ScrollArea::from_max_height(scroll_height)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.set_min_height(scroll_height);
                        for x in &state.scrollback {
                            ui.code(x);
                        }
                    });
                    ui.scroll_to_cursor(Align::BOTTOM);
                });

            ui.separator();

            let input_elem = TextEdit::singleline(&mut state.input_buf)
                .frame(false)
                .desired_width(cfg.width)
                .code_editor();
            let add_input_elem_resp = ui.add(input_elem);
            if add_input_elem_resp.lost_focus() && ui.input().key_pressed(cfg.submit_input_key) {
                input_evt_wrtr.send(Input(state.input_buf.clone()));
                state.input_buf.clear();
            }
            ui.memory().request_focus(add_input_elem_resp.id)
        });

    state.is_open = is_open;
}

// single system instead of multiple, single-responsibility event handlers due
// to timing issues (eval result sometimes displayed before input)
fn eval_input_and_add_input_to_scrollback(
    state: NonSend<StateRef>,
    mut input_evt_rdr: EventReader<Input>,
    mut eval_evt_wrtr: EventWriter<scripting::Eval>,
    mut disp_evt_wrtr: EventWriter<Display>,
) {
    let state = state.as_ref().borrow_mut();
    let prefix = state.scrollback_line_prompt_prefix.as_ref().map(|x| x.as_str()).unwrap_or("?");
    let prompt = state.scrollback_line_prompt.as_ref().map(|x| x.as_str()).unwrap_or("=> ");
    //
    for input in input_evt_rdr.iter() {
        let to_eval = input.0.trim();
        eval_evt_wrtr.send(scripting::Eval(to_eval.to_owned()));
        //
        let scrollback_line = [prefix, prompt, to_eval].join("");
        disp_evt_wrtr.send(Display(scrollback_line));
    }
}
//fn eval_input(
//    mut input_evt_rdr: EventReader<Input>,
//    mut eval_evt_wrtr: EventWriter<scripting::Eval>,
//) {
//    for input_evt in input_evt_rdr.iter() {
//        let input = input_evt.0.trim().to_owned();
//        eval_evt_wrtr.send(scripting::Eval(input));
//    }
//}
//fn add_input_to_scrollback(
//    state: NonSend<StateRef>,
//    mut input_evt_rdr: EventReader<Input>,
//    mut disp_evt_wrtr: EventWriter<Display>,
//) {
//    let state = state.as_ref().borrow_mut();
//    let prefix = state.scrollback_line_prompt_prefix.as_ref().map(|x| x.as_str()).unwrap_or("?");
//    let prompt = state.scrollback_line_prompt.as_ref().map(|x| x.as_str()).unwrap_or("=> ");
//
//    for input_evt in input_evt_rdr.iter() {
//        let input = input_evt.0.trim();
//        let line = [prefix, prompt, input].join("");
//        disp_evt_wrtr.send(Display(line));
//    }
//}

fn mut_state_add_to_scrollback(
    state: NonSend<StateRef>,
    mut disp_evt_rdr: EventReader<Display>,
) {
    let mut state = state.as_ref().borrow_mut();

    for disp in disp_evt_rdr.iter() {
        let to_disp = disp.0.to_string();
        state.scrollback.push(to_disp);
    }
}

fn mut_state_clear_scrollback(
    mut clear_disp_evt_rdr: EventReader<ClearDisplay>,
    state: NonSend<StateRef>,
) {
    let mut state = state.as_ref().borrow_mut();
    for _ in clear_disp_evt_rdr.iter() {
        state.scrollback.clear();
    }
}

fn startup_bevy(
    env: NonSend<scripting::Env>,
    state: NonSend<StateRef>,
) {
    let env = env.as_ref();
    let state = state.clone();
    startup(env, state);
}

fn startup(
    env: scripting::EnvRef,
    state: StateRef,
) {
    set_state_per_env(env, &mut state.as_ref().borrow_mut());
    bind_console_commands(env, state);
}

fn set_state_per_env(
    env: scripting::EnvRef,
    state: &mut State,
) {
    state.scrollback_line_prompt_prefix = env.get_current_namespace_name().into();
}

fn bind_console_commands(
    env: scripting::EnvRef,
    state: StateRef,
) {
    scripting::bind(env, "console", "clear-scrollback", {
        use cljrs::{value::{Value, ToValue}, ifn::IFn};
        //
        #[derive(Debug, Clone)]
        struct ClearScrollbackFn {
            console_state: StateRef,
        }
        impl ToValue for ClearScrollbackFn {
            fn to_value(&self) -> Value {
                Value::IFn(Rc::new(self.clone()))
            }
        }
        impl IFn for ClearScrollbackFn {
            fn invoke(&self, _args: Vec<Rc<Value>>) -> Value {
                (*self.console_state.borrow_mut()).scrollback.clear();
                Value::Nil
            }
        }
        //
        ToValue::to_rc_value(&ClearScrollbackFn {
            console_state: state.clone(),
        })
    });
}

// cross-plugin/subsystem/etc concerns /////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

fn display_evaled(
    mut evaled_evt_rdr: EventReader<scripting::Evaled>,
    mut display_evt_wrtr: EventWriter<Display>,
) {
    let conv_evaled_to_display = |evaled: &scripting::Evaled| -> Display {
        Display(evaled.0.clone())
    };
    display_evt_wrtr.send_batch(
        evaled_evt_rdr.iter()
            .map(conv_evaled_to_display),
    );
}

// plugins / add to bevy app ///////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

pub const STARTUP_SYSTEM_LABEL: &str = "console_startup";

pub struct ConsolePlugin;
impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            //.insert_non_send_resource(state_ref(State::default()))
            .add_event::<Input>()
            .add_event::<Display>()
            .add_event::<ClearDisplay>()
            //.add_system(add_input_to_scrollback.system())
            //.add_system(eval_input.system())
            .add_system(eval_input_and_add_input_to_scrollback.system())
            .add_system(mut_state_add_to_scrollback.system())
            .add_system(mut_state_clear_scrollback.system())
            .add_plugin(EguiPlugin)
            .add_system(display_evaled.system())
            .add_system(console_egui_window.exclusive_system())
            //
            .add_startup_system(
                startup_bevy.system()
                    .label(STARTUP_SYSTEM_LABEL)
                    .after(scripting::STARTUP_SYSTEM_LABEL),
            )
            ;
    }
}
