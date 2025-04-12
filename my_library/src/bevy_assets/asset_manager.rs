use crate::AssetStore;
use bevy::prelude::*;

#[derive(Clone)]
pub enum AssetType {
    Image,
    Sound,
}

#[derive(Resource, Clone)]
pub struct AssetManager {
    pub(crate) asset_list: Vec<(String, String, AssetType)>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            asset_list: vec![
                (
                    "main_menu".to_string(),
                    "main_menu.png".to_string(),
                    AssetType::Image,
                ),
                (
                    "game_over".to_string(),
                    "game_over.png".to_string(),
                    AssetType::Image,
                ),
            ],
        }
    }

    fn asset_exists(filename: &str) -> anyhow::Result<()> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let current_directory = std::env::current_dir()?;
            let assets = current_directory.join("assets");
            let new_image = assets.join(filename);
            if !new_image.exists() {
                return Err(anyhow::Error::msg(format!(
                    "{} not found in assets directory",
                    &filename
                )));
            }
        }
        Ok(())
    }

    pub fn add_image<S: ToString>(
        mut self,
        tag: S,
        filename: S,
    ) -> anyhow::Result<Self> {
        let filename = filename.to_string();
        AssetManager::asset_exists(&filename)?;
        self.asset_list
            .push((tag.to_string(), filename, AssetType::Image));
        Ok(self)
    }

    pub fn add_sound<S: ToString>(
        mut self,
        tag: S,
        filename: S,
    ) -> anyhow::Result<Self> {
        let filename = filename.to_string();
        AssetManager::asset_exists(&filename)?;
        self.asset_list
            .push((tag.to_string(), filename, AssetType::Sound));
        Ok(self)
    }
}

impl Plugin for AssetManager {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.clone());
    }
}

pub(crate) fn setup_asset_store(
    asset_resource: &AssetManager,
    commands: &mut Commands,
    asset_server: &AssetServer,
) -> AssetStore {
    let mut assets = AssetStore {
        asset_index: bevy::utils::HashMap::new(),
    };
    asset_resource
        .asset_list
        .iter()
        .for_each(|(tag, filename, asset_type)| match asset_type {
            _ => {
                assets
                    .asset_index
                    .insert(tag.clone(), asset_server.load_untyped(filename));
            }
        });
    commands.remove_resource::<AssetManager>();
    commands.insert_resource(assets.clone());
    assets
}
