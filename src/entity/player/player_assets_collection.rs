use bevy::asset::Handle;
use bevy::{prelude::*, math::*};
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct PlayerAssetCollection {

    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 6, rows = 1))]
    pub stop_by_front_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "player/player-walking.png")]
    pub stop_by_front: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 6, rows = 1))]
    pub stop_by_back_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "player/player-walking.png")]
    pub stop_by_back: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 6, rows = 1))]
    pub stop_by_left_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "player/player-walking.png")]
    pub stop_by_left: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 6, rows = 1))]
    pub stop_by_right_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "player/player-walking.png")]
    pub stop_by_right: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 6, rows = 1))]
    pub walking_by_front_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "player/player-walking.png")]
    pub walking_by_front: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 6, rows = 1))]
    pub walking_by_back_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "player/player-walking.png")]
    pub walking_by_back: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 6, rows = 1))]
    pub walking_by_left_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "player/player-walking.png")]
    pub walking_by_left: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 6, rows = 1))]
    pub walking_by_right_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "player/player-walking.png")]
    pub walking_by_right: Handle<Image>,

}