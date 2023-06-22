use bevy::prelude::*;
use bevy_oneshot::CommandRunOnce;

fn some_system(mut commands: Commands) {
    // run once without arguments
    commands.run_once(|_query: Query<&Window>| {
        // your code here
    });

    // run once with input arguments
    commands.run_once_with(
        |input: In<u32>| {
            println!("{}", input.0);
        },
        42,
    );
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(some_system)
        .run();
}
