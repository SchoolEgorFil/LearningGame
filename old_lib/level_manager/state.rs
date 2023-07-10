// use std::time::Instant;

// use bevy::prelude::Component;

// enum StateUpdateResult {
//     goto(u32),
//     exit,
// }
// struct StateUpdateEvent(StateUpdateResult);

// #[derive(Component)]
// struct StateMachine {
//     active: [u32; 4],
//     set_active: &'static dyn Fn() -> (),
//     time: Instant,
// }

// impl StateMachine {
//     fn new() -> StateMachine {
//         StateMachine {
//             active: (),
//             set_active: move |_, _: &_| {},
//             data: (),
//         }
//     }
// }
