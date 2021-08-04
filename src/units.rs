// imports /////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

use bevy::prelude::*;

// plugins / add to bevy app ///////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

pub struct UnitPlugin;
impl Plugin for UnitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
           .insert_resource(ListUnitSelectionsTimer({
               let mut t = Timer::from_seconds(3.0, true);
               t.pause();
               t
            }))
           .add_startup_system(sys_startup.system())
           .add_system(sys_list_unit_selections.system())
           ;
    }
}

// resources ///////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

pub(crate) struct ListUnitSelectionsTimer(pub Timer);

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
    cmds: Commands,
    mut list_unit_selections_timer: ResMut<ListUnitSelectionsTimer>,
) {
    populate_with_units(cmds);
    list_unit_selections_timer.0.unpause();
}

fn sys_list_unit_selections(
    time: Res<Time>,
    mut timer: ResMut<ListUnitSelectionsTimer>,
    q_selectable_units: Query<(Entity, &SelectableComponent), With<UnitComponent>>,
) {
    let when = time.time_since_startup();
    let rs_dbg_curr_file = file!();

    if timer.0.tick(time.delta()).just_finished() {
        for (ent, selectable) in q_selectable_units.iter() {
            println!(
                "[{when:?} {where}] {ent:?} is {not_}selected",
                when = when,
                where = rs_dbg_curr_file,
                ent = ent,
                not_ = if !selectable.is_selected { "not " } else { "" },
            );
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
    cmds: &mut Commands,
    unit: UnitComponent,
    selectable: SelectableComponent,
) {
    let mut ent = cmds.spawn();
    ent.insert(unit);
    ent.insert(selectable);
}

fn populate_with_units(mut cmds: Commands) {
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
