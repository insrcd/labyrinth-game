
use amethyst::core::transform::Transform;
use amethyst::renderer::{
    SpriteRender, SpriteSheet,
    Texture, Transparent, formats::texture::ImageFormat
};

use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::prelude::*;

use amethyst::window::ScreenDimensions;

mod components;

struct EnterRoomState;

pub fn load_texture<N>(name: N, world: &World) -> Handle<Texture>
where
    N: Into<String>,
{
    let loader = world.read_resource::<Loader>();
    loader.load(
        name,
        ImageFormat::default(),
        (),
        &world.read_resource::<AssetStorage<Texture>>(),
    )
}

impl SimpleState for EnterRoomState {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        // ...

        let sprite_sheet = load_sprite_sheet(texture_handle);
        let sprite_sheet_handle = {
            let loader = data.world.read_resource::<Loader>();
            loader.load_from_data(
                sprite_sheet,
                (),
                &data.world.read_resource::<AssetStorage<SpriteSheet>>(),
            )
        };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
