use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .add_systems(Update, bevy::window::close_on_esc)
    .add_systems(FixedUpdate, move_player)
    .run();
}


struct Player {

}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        sprite: Sprite::new(Vec2::new(100.0, 100.0)),
        ..Default::default()
    });
}

fn move_player(
    mut query: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    fixed_time: Res<FixedTime>
) {
    for (mut transform, _player) in query.iter_mut() {
        transform.translation.x += 1.0;
    }
}
```