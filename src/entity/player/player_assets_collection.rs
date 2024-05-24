use bevy::asset::Handle;
use bevy::{prelude::*, math::*};
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct PlayerAssetCollection {

    #[asset(texture_atlas_layout(tile_size_x = 64., tile_size_y = 64., columns = 1, rows = 1))]
    pub stop_by_front_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "player/player.png")]
    pub stop_by_front: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 64., tile_size_y = 64., columns = 1, rows = 1))]
    pub stop_by_back_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "player/player.png")]
    pub stop_by_back: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 64., tile_size_y = 64., columns = 1, rows = 1))]
    pub stop_by_left_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "player/player.png")]
    pub stop_by_left: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 64., tile_size_y = 64., columns = 1, rows = 1))]
    pub stop_by_right_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "player/player.png")]
    pub stop_by_right: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 64., tile_size_y = 64., columns = 6, rows = 8))]
    pub walking_layout: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "player/player.png")]
    pub walking: Handle<Image>,

}