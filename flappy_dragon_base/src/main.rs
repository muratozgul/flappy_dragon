use bevy::prelude::*;
use my_library::*;

#[derive(Component)]
struct Flappy {
    gravity: f32,
}

#[derive(Component)]
struct Obstacle;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default, States)]
enum GamePhase {
    Setup,
    #[default]
    MainMenu,
    Flapping,
    GameOver,
}

#[derive(Component)]
struct FlappyElement;

fn main() -> anyhow::Result<()> {
    let mut app = App::new();

    add_phase!(app, GamePhase, GamePhase::Flapping,
        start => [setup],
        run => [gravity, flap, clamp, move_walls, hit_wall],
        exit => [cleanup::<FlappyElement>],
    );
    
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Flappy Dragon - Bevy Edition".to_string(),
                resolution: bevy::window::WindowResolution::new(1024.0, 768.0),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RandomPlugin)
        .add_plugins(
            GameStatePlugin::<GamePhase>::new(
                GamePhase::Setup,
                GamePhase::MainMenu,
                GamePhase::Flapping,
                GamePhase::GameOver
            )
        )
        .add_plugins(
            AssetManager::new()
                .add_image("dragon", "flappy_dragon.png")?
                .add_image("wall", "wall.png")?
        )
        .run();

    Ok(())
}

fn setup(
    mut commands: Commands,
    mut rng: ResMut<RandomNumberGenerator>,
    assets: Res<AssetStore>,
    loaded_assets: AssetResource,
) {
    let dragon = assets.get_handle("dragon", &loaded_assets).unwrap();
    
    commands.spawn(Camera2d::default()).insert(FlappyElement);
    commands
        .spawn(Sprite::from(dragon))
        .insert(Transform::from_xyz(-490.0, 0.0, 1.0))
        .insert(Flappy { gravity: 0.0 })
        .insert(FlappyElement);

    build_wall(&mut commands, &assets, &loaded_assets, rng.range(-5..5));
}

fn build_wall(
    commands: &mut Commands,
    assets: &Res<AssetStore>,
    loaded_assets: &AssetResource,
    gap_y: i32,
) {
    let wall = assets.get_handle("wall", loaded_assets).unwrap();

    for y in -12..=12 {
        if y < gap_y - 4 || y > gap_y + 4 {
            commands
                .spawn(Sprite::from(wall.clone()))
                .insert(Transform::from_xyz(512.0, y as f32 * 32.0, 1.0))
                .insert(Obstacle)
                .insert(FlappyElement);
        }
    }
}

fn gravity(mut query: Query<(&mut Flappy, &mut Transform)>) {
    if let Ok((mut flappy, mut transform)) = query.get_single_mut() {
        flappy.gravity += 0.1;
        transform.translation.y -= flappy.gravity;
    }
}

fn flap(keyboard: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Flappy>) {
    if keyboard.pressed(KeyCode::Space) {
        if let Ok(mut flappy) = query.get_single_mut() {
            flappy.gravity = -5.0;
        }
    }
}

fn clamp(
    mut query: Query<&mut Transform, With<Flappy>>,
    mut state: ResMut<NextState<GamePhase>>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        if transform.translation.y > 384.0 {
            transform.translation.y = 384.0;
        } else if transform.translation.y < -384.0 {
            state.set(GamePhase::GameOver);
        }
    }
}

fn move_walls(
    mut commands: Commands,
    mut query: Query<&mut Transform, With<Obstacle>>,
    delete: Query<Entity, With<Obstacle>>,
    assets: Res<AssetStore>,
    loaded_assets: AssetResource,
    mut rng: ResMut<RandomNumberGenerator>,
) {
    let mut rebuild = false;
    for mut transform in query.iter_mut() {
        transform.translation.x -= 4.0;
        if transform.translation.x < -530.0 {
            rebuild = true;
        }
    }
    if rebuild {
        for entity in delete.iter() {
            commands.entity(entity).despawn();
        }
        build_wall(&mut commands, &assets, &loaded_assets, rng.range(-5..5));
    }
}

fn hit_wall(
    player: Query<&Transform, With<Flappy>>,
    walls: Query<&Transform, With<Obstacle>>,
    mut state: ResMut<NextState<GamePhase>>,
) {
    if let Ok(player) = player.get_single() {
        for wall in walls.iter() {
            let distance = player.translation.distance(wall.translation);
            if distance < 32.0 {
                state.set(GamePhase::GameOver);
            }
        }
    }
}
