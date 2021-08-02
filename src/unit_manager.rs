use bevy::prelude::*;

pub struct UnitManagerPlugin;
impl Plugin for UnitManagerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
           .insert_resource(ListUnitSelectionTimer({
               let mut t = Timer::from_seconds(3.0, true);
               t.pause();
               t
            }))
           .add_startup_system(sys_startup.system())
           .add_system(list_unit_selection_state.system())
           ;
    }
}

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
pub struct UnitComponent {
    pub player_id: PlayerId,
    pub unit_type_id: UnitTypeId,
}
impl UnitComponent {
    pub fn new(player_id: PlayerId, unit_type_id: UnitTypeId) -> Self {
        Self {
            player_id,
            unit_type_id,
        }
    }
}
macro_rules! unit {
    ($pid:expr, $uid:expr) => {{
        $crate::unit_manager::UnitComponent::new($pid, $uid)
    }};
}

#[derive(Debug, Clone, PartialEq)]
pub enum SelectableState {
    Selected,
    NotSelected,
}
#[derive(Debug, Clone)]
pub struct SelectableComponent {
    pub is: SelectableState,
}

fn sys_startup(
    cmds: Commands,
    mut list_unit_selection_timer: ResMut<ListUnitSelectionTimer>,
) {
    populate_with_units(cmds);
    list_unit_selection_timer.0.unpause();
}

fn add_unit(
    cmds: &mut Commands,
    unit: UnitComponent,
    selectable: SelectableState,
) {
    let mut ent = cmds.spawn();
    ent.insert(unit);
    ent.insert(SelectableComponent { is: selectable });
}

fn populate_with_units(mut cmds: Commands) {
    add_unit(&mut cmds,
        unit!(PlayerId(1), UnitTypeId(1)),
        SelectableState::NotSelected);
    add_unit(&mut cmds,
        unit!(PlayerId(1), UnitTypeId(1)),
        SelectableState::NotSelected);
    //
    add_unit(&mut cmds,
        unit!(PlayerId(1), UnitTypeId(2)),
        SelectableState::NotSelected);
    add_unit(&mut cmds,
        unit!(PlayerId(1), UnitTypeId(2)),
        SelectableState::Selected);
    //
    add_unit(&mut cmds,
        unit!(PlayerId(2), UnitTypeId(1)),
        SelectableState::Selected);
    add_unit(&mut cmds,
        unit!(PlayerId(2), UnitTypeId(1)),
        SelectableState::Selected);
    //
    add_unit(&mut cmds,
        unit!(PlayerId(2), UnitTypeId(2)),
        SelectableState::Selected);
    add_unit(&mut cmds,
        unit!(PlayerId(2), UnitTypeId(2)),
        SelectableState::NotSelected);
}

pub(crate) struct ListUnitSelectionTimer(pub Timer);
fn list_unit_selection_state(
    time: Res<Time>,
    mut timer: ResMut<ListUnitSelectionTimer>,
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
                not_ = if selectable.is == SelectableState::Selected { "not " } else { "" },
            );
        }
    }
}
