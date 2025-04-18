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
    #[default]
    Loading,
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
    .add_plugins(GameStatePlugin::new(
        GamePhase::MainMenu,
        GamePhase::Flapping,
        GamePhase::GameOver,
    ))
    .add_plugins(
        AssetManager::new()
            .add_image("dragon", "flappy_dragon.png")?
            .add_image("wall", "wall.png")?
            .add_sound("flap", "dragonflap.ogg")?
            .add_sound("crash", "crash.ogg")?
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
    commands.spawn(Camera2d::default()).insert(FlappyElement);
    spawn_image!(
        assets,
        commands,
        "dragon",
        -490.0,
        0.0,
        1.0,
        &loaded_assets,
        Flappy { gravity: 0.0 },
        FlappyElement
    );

    build_wall(&mut commands, &assets, rng.range(-5..5), &loaded_assets);
}

fn build_wall(
    commands: &mut Commands,
    assets: &AssetStore,
    gap_y: i32,
    loaded_assets: &LoadedAssets,
) {
    for y in -12..=12 {
        if y < gap_y - 4 || y > gap_y + 4 {
            spawn_image!(
                assets,
                commands,
                "wall",
                512.0,
                y as f32 * 32.0,
                1.0,
                &loaded_assets,
                Obstacle,
                FlappyElement
            );
        }
    }
}

fn gravity(mut query: Query<(&mut Flappy, &mut Transform)>) {
    if let Ok((mut flappy, mut transform)) = query.get_single_mut() {
        flappy.gravity += 0.1;
        transform.translation.y -= flappy.gravity;
    }
}

fn flap(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Flappy>,
    assets: Res<AssetStore>,
    loaded: Res<LoadedAssets>,
    mut commands: Commands,
) {
    if keyboard.pressed(KeyCode::Space) {
        if let Ok(mut flappy) = query.get_single_mut() {
            flappy.gravity = -5.0;
            assets.play("flap", &mut commands, &loaded);
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
        build_wall(&mut commands, &assets, rng.range(-5..5), &loaded_assets);
    }
}

fn hit_wall(
    player: Query<&Transform, With<Flappy>>,
    walls: Query<&Transform, With<Obstacle>>,
    mut state: ResMut<NextState<GamePhase>>,
    assets: Res<AssetStore>,
    loaded_assets: Res<LoadedAssets>,
    mut commands: Commands,
) {
    if let Ok(player) = player.get_single() {
        for wall in walls.iter() {
            let distance = player.translation.distance(wall.translation);
            if distance < 32.0 {
                assets.play("crash", &mut commands, &loaded_assets);
                state.set(GamePhase::GameOver);
            }
        }
    }
}
