// imports /////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

// plugins / add to bevy app ///////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

pub struct UnitPlugin;
impl Plugin for UnitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(ShapePlugin);
        app
           .insert_resource(SetUnitVisTimer({
               let mut t = Timer::from_seconds(1.2, true);
               t.pause();
               t
            }))
           .add_startup_system(sys_startup.system())
           .add_system(sys_set_unit_visibility_based_on_selection_status_on_timer.system())
           ;
    }
}

// resources ///////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

pub(crate) struct SetUnitVisTimer(pub Timer);

// components //////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct UnitComponent {
    pub player_id: PlayerId,
    pub unit_type_id: UnitTypeId,
}

#[derive(Debug)]
pub struct SelectableComponent {
    pub is_selected: bool,
}

// systems /////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

fn sys_startup(
    mut cmds: Commands,
    mut list_unit_selections_timer: ResMut<SetUnitVisTimer>,
) {
    populate_with_units(&mut cmds);

    cmds.spawn_bundle(OrthographicCameraBundle::new_2d());

    list_unit_selections_timer.0.unpause();
}

fn sys_set_unit_visibility_based_on_selection_status_on_timer(
    time: Res<Time>,
    mut timer: ResMut<SetUnitVisTimer>,
    mut q: Query<(Entity, &SelectableComponent, &mut Visible), With<UnitComponent>>,
) {
    let when = time.time_since_startup();
    let rs_dbg_curr_file = file!();

    if timer.0.tick(time.delta()).just_finished() {
        for (ent, selectable, mut visible) in q.iter_mut() {
            print!("{:?} vis={},sel={} -> vis=", ent, visible.is_visible, selectable.is_selected);
            visible.is_visible = selectable.is_selected;
            println!("{}", visible.is_visible);
            //println!(
            //    "[{when:?} {where}] {ent:?} is {not_}selected",
            //    when = when,
            //    where = rs_dbg_curr_file,
            //    ent = ent,
            //    not_ = if !selectable.is_selected { "not " } else { "" },
            //);
        }
    }
}

// unorganized /////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct PlayerId(pub u8);
impl std::fmt::Display for PlayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "#player-id,{}", self.0)
    }
}

#[derive(Debug)]
pub struct UnitTypeId(pub u32);
impl std::fmt::Display for UnitTypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "#unit-type-id,{}", self.0)
    }
}

fn add_unit(
    mut cmds: &mut Commands,
    unit: UnitComponent,
    selectable: SelectableComponent,
) {
    let mut ent = cmds.spawn();
    ent.insert(unit);
    ent.insert(selectable);

    use rand::Rng as _;
    let mut rng = rand::thread_rng();
    let fill_color: Color = {
        let r = rng.gen_range(0.0..=1.0);
        let g = rng.gen_range(0.0..=1.0);
        let b = rng.gen_range(0.0..=1.0);
        Color::rgba(r, g, b, 1.0)
    };
    let stroke_color: Color = {
        let r = rng.gen_range(0.0..=1.0);
        let g = rng.gen_range(0.0..=1.0);
        let b = rng.gen_range(0.0..=1.0);
        Color::rgba(r, g, b, 1.0)
    };
    let width: f32 = {
        rng.gen_range(20.0..=80.0)
    };
    let height: f32 = {
        rng.gen_range(20.0..=80.0)
    };
    let origin: shapes::RectangleOrigin = {
        let x = rng.gen_range(-90.0..=90.0);
        let y = rng.gen_range(-90.0..=90.0);
        shapes::RectangleOrigin::CustomCenter(Vec2::new(x, y))
    };

    ent.insert_bundle(GeometryBuilder::build_as(
        &shapes::Rectangle { width, height, origin },
        ShapeColors::outlined(fill_color, stroke_color),
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(10.0),
        },
        Transform::default(),
    ));
}

fn populate_with_units(mut cmds: &mut Commands) {
    add_unit(&mut cmds,
        UnitComponent { player_id: PlayerId(1), unit_type_id: UnitTypeId(1) },
        SelectableComponent { is_selected: false });
    add_unit(&mut cmds,
        UnitComponent { player_id: PlayerId(1), unit_type_id: UnitTypeId(1) },
        SelectableComponent { is_selected: false });
    //
    add_unit(&mut cmds,
        UnitComponent { player_id: PlayerId(1), unit_type_id: UnitTypeId(2) },
        SelectableComponent { is_selected: false });
    add_unit(&mut cmds,
        UnitComponent { player_id: PlayerId(1), unit_type_id: UnitTypeId(2) },
        SelectableComponent { is_selected: true });
    //
    add_unit(&mut cmds,
        UnitComponent { player_id: PlayerId(2), unit_type_id: UnitTypeId(1) },
        SelectableComponent { is_selected: true });
    add_unit(&mut cmds,
        UnitComponent { player_id: PlayerId(2), unit_type_id: UnitTypeId(1) },
        SelectableComponent { is_selected: true });
    //
    add_unit(&mut cmds,
        UnitComponent { player_id: PlayerId(2), unit_type_id: UnitTypeId(2) },
        SelectableComponent { is_selected: true });
    add_unit(&mut cmds,
        UnitComponent { player_id: PlayerId(2), unit_type_id: UnitTypeId(2) },
        SelectableComponent { is_selected: false });
}
