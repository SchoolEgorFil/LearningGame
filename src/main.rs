use bevy::prelude::*;
mod lib;
use lib::*;
use bevy_editor_pls::prelude::*;

fn main() {
    // std::env::set_var("RUST_BACKTRACE", "full");
    
    let app = App::new()
                .add_plugin(GamePlugin {})
                .add_plugin(EditorPlugin::default())
                .run();
}