use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::dev::systems::handle_dev_input;

pub struct DevPlugin;
impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        // Rapier
        app.add_plugins(RapierDebugRenderPlugin::default());

        // Inspector
        app.add_plugins((
            bevy_inspector_egui::DefaultInspectorConfigPlugin,
            bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
        ));

        app.add_systems(Update, handle_dev_input);
    }
}
