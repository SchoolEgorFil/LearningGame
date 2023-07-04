use bevy::prelude::*;

mod lib;

use bevy_editor_pls::prelude::*;
use lib::*;

fn main() {
    // std::env::set_var("RUST_BACKTRACE", "full");

    let app = App::new()
        .add_plugin(GamePlugin {})
        .add_plugin(EditorPlugin::default())
        .run();
}

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_system(file_drag_and_drop_system)
//         .run();
// }

// fn file_drag_and_drop_system(mut events: EventReader<FileDragAndDrop>) {
//     for event in events.iter() {
//         info!("{:?}", event);
//     }
// }
