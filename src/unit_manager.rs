use bevy::prelude::*;

pub struct UnitManagerPlugin;
impl Plugin for UnitManagerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
           .insert_resource(ListSelectedUnitsTimer(Timer::from_seconds(2.0, true)))
           .add_startup_system(prepopulate_with_units.system())
           .add_system(list_selected_units.system());
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
    (player_id: $pid:expr, unit_type_id: $uid:expr) => {
        crate::UnitComponent::new($pid, $uid)
    };
}

pub struct SelectedUnitComponent;

enum UnitSelectedState {
    Selected,
    NotSelected,
}
fn add_unit(c: &mut Commands, u: UnitComponent, selected: UnitSelectedState) {
    let mut ent = c.spawn();
    ent.insert(u);
    if let UnitSelectedState::Selected = selected {
        ent.insert(SelectedUnitComponent);
    }
}

pub fn prepopulate_with_units(mut c: Commands) {
    add_unit(&mut c, unit!(player_id: PlayerId(0), unit_type_id: UnitTypeId(0)), UnitSelectedState::Selected);
    add_unit(&mut c, unit!(player_id: PlayerId(0), unit_type_id: UnitTypeId(0)), UnitSelectedState::NotSelected);
    add_unit(&mut c, unit!(player_id: PlayerId(1), unit_type_id: UnitTypeId(1)), UnitSelectedState::Selected);
}
pub struct ListSelectedUnitsTimer(Timer);
pub fn list_selected_units(
    time: Res<Time>,
    mut timer: ResMut<ListSelectedUnitsTimer>,
    query: Query<&UnitComponent, With<SelectedUnitComponent>>,
) {
    let when = time.time_since_startup();
    if timer.0.tick(time.delta()).just_finished() {
        for unit_comp in query.iter() {
            println!("[{:?}] unit is selected: {} {}", when, unit_comp.player_id, unit_comp.unit_type_id);
        }
    }
}
