use super::{MenuAssets, MenuResource};
use bevy::{app::AppExit, color::palettes::css::WHITE, prelude::*, state::state::FreelyMutableState};
use trait_set::trait_set;

trait_set! {
    pub trait PluginState = FreelyMutableState + FromWorld + Copy;
}

#[derive(Component)]
pub(crate) struct MenuElement;

#[derive(Component)]
pub(crate) struct DebugText;

pub(crate) fn setup<T: PluginState>(
    state: Res<State<T>>,
    mut commands: Commands,
    menu_assets: Res<MenuAssets>,
    menu_resource: Res<MenuResource<T>>,
) {
    let current_state = state.get();
    let menu_graphic = match current_state {
        current_state if menu_resource.menu_state == *current_state =>
            menu_assets.main_menu.clone(),
        current_state if menu_resource.game_end_state == *current_state =>
            menu_assets.game_over.clone(),
        _ => panic!("Unknown menu state"),
    };

    commands
        .spawn(Camera2d::default())
        .insert(MenuElement);

    commands
        .spawn(Sprite::from(menu_graphic))
        .insert(Transform::from_xyz(0.0, 0.0, 1.0))
        .insert(MenuElement);

    setup_debug(commands);
}

pub(crate) fn run<T: PluginState>(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
    current_state: Res<State<T>>,
    mut state: ResMut<NextState<T>>,
    menu_state: Res<MenuResource<T>>,
) {
    let current_state = current_state.get().clone();
    if current_state == menu_state.menu_state {
        if keyboard.just_pressed(KeyCode::KeyP) {
            state.set(menu_state.game_start_state);
        }
        else if keyboard.just_pressed(KeyCode::KeyQ) {
            exit.send(AppExit::Success);
        }
    }
    else if current_state == menu_state.game_end_state {
        if keyboard.just_pressed(KeyCode::KeyM) {
            state.set(menu_state.menu_state);
        }
        else if keyboard.just_pressed(KeyCode::KeyQ) {
            exit.send(AppExit::Success);
        }
    }
}

pub(crate) fn setup_debug(
    mut commands: Commands,
) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            right: Val::Px(10.0),
            top: Val::Px(10.0),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("Murat"),
                TextColor(WHITE.into()),
            )).insert(DebugText);
        });
}

pub(crate) fn update_debug_text<T: PluginState>(
    game_state: Res<State<T>>,
    mut query: Query<&mut Text, With<DebugText>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        text.0 = format!("State: {:?}", game_state.get());
    }
}