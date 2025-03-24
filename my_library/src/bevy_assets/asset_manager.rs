use bevy::{prelude::*, app::Plugin, ecs::system::Resource};

use super::AssetStore;

#[derive(Clone)]
pub enum AssetType {
    Image,
}

#[derive(Resource, Clone)]
pub struct AssetManager {
    pub(crate) asset_list: Vec<(String, String, AssetType)>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            asset_list: Vec::new(),
        }
    }

    pub fn add_image<S: ToString>(
        mut self,
        tag: S,
        filename: S,
    ) -> anyhow::Result<Self> {
        let filename = filename.to_string();

        #[cfg(not(target_arch = "wasm32"))]
        {
            let current_directory = std::env::current_dir()?;
            let assets = current_directory.join("assets");
            let new_image = assets.join(&filename);
            if !new_image.exists() {
                return Err(anyhow::Error::msg(format!(
                    "{} not found in assets directory",
                    &filename
                )));
            }
        }

        self.asset_list.push((
            tag.to_string(), filename, AssetType::Image
        ));
        Ok(self)
    }
}

impl Plugin for AssetManager {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.clone());
        app.add_systems(Startup, setup);
    }
}

fn setup(
    asset_resource: Res<AssetManager>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut assets = AssetStore {
        asset_index: bevy::utils::HashMap::new(),
    };
    asset_resource.asset_list.iter().for_each(
        |(tag, filename, asset_type)| {
            match asset_type {
                _ => {
                    assets
                        .asset_index
                        .insert(tag.clone(), asset_server.load_untyped(filename));
                }
            }
        }
    );
    commands.remove_resource::<AssetManager>();
    commands.insert_resource(assets);
}