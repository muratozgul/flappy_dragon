use bevy::{
    asset::{Asset, LoadedUntypedAsset},
    prelude::*,
    utils::HashMap,
};

pub type LoadedAssets = Assets<LoadedUntypedAsset>;
pub type AssetResource<'w> = Res<'w, LoadedAssets>;

#[derive(Resource, Clone)]
pub struct AssetStore {
    pub(crate) asset_index: HashMap<String, Handle<LoadedUntypedAsset>>,
}

impl AssetStore {
    pub fn get_handle<T: Asset>(
        &self,
        index: &str,
        assets: &LoadedAssets,
    ) -> Option<Handle<T>> {
        if let Some(handle_untyped) = self.asset_index.get(index) {
            if let Some(handle) = assets.get(handle_untyped) {
                return Some(handle.handle.clone().typed::<T>());
            }
            None
        } else {
            None
        }
    }

    pub fn play(
        &self,
        sound_name: &str,
        commands: &mut Commands,
        assets: &LoadedAssets,
    ) {
        let sound_handle: Handle<AudioSource> =
            self.get_handle(sound_name, assets).unwrap();
        commands.spawn((AudioPlayer::new(sound_handle.clone()),));
    }
}
