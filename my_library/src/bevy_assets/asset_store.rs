use bevy::{
    asset::{Asset, LoadedUntypedAsset},
    prelude::*,
    utils::HashMap,
};

pub type LoadedAssets = Assets<LoadedUntypedAsset>;
pub type AssetResource<'w> = Res<'w, LoadedAssets>;

#[derive(Resource)]
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
}
