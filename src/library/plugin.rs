use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollectionApp;

use crate::library::resources::Library;

pub struct LibraryPlugin;
impl Plugin for LibraryPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Library>();

        app.init_collection::<Library>();
    }
}
