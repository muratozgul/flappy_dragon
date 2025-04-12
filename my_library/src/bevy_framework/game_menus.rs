use crate::{AssetStore, MenuResource};
use bevy::state::state::FreelyMutableState;
use bevy::{app::AppExit, prelude::*};
use trait_set::trait_set;

// trait_set! {
//     pub trait PluginState = FreelyMutableState + Copy + Default;
// }
trait_set! {
    pub trait PluginState = States + FromWorld + FreelyMutableState;
}

#[derive(Component)]
pub(crate) struct MenuElement;

pub(crate) fn setup<T: PluginState>(
    state: Res<State<T>>,
    mut commands: Commands,
    menu_resource: Res<MenuResource<T>>,
    loaded_assets: crate::AssetResource,
    assets: Res<AssetStore>,
) {
    let current_state = state.get();
    let menu_graphic = match current_state {
        current_state if menu_resource.menu_state == *current_state => {
            assets.get_handle("main_menu", &loaded_assets).unwrap()
        }
        current_state if menu_resource.game_end_state == *current_state => {
            assets.get_handle("game_over", &loaded_assets).unwrap()
        }
        _ => panic!("Unknown menu state"),
    };

    commands.spawn(Camera2d::default()).insert(MenuElement);

    commands.spawn((
        Sprite::from(menu_graphic),
        Transform::from_xyz(0.0, 0.0, 1.0),
        MenuElement,
    ));
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
            state.set(menu_state.game_start_state.clone());
        } else if keyboard.just_pressed(KeyCode::KeyQ) {
            exit.send(AppExit::Success);
        }
    } else if current_state == menu_state.game_end_state {
        if keyboard.just_pressed(KeyCode::KeyM) {
            state.set(menu_state.menu_state.clone());
        } else if keyboard.just_pressed(KeyCode::KeyQ) {
            exit.send(AppExit::Success);
        }
    }
}
