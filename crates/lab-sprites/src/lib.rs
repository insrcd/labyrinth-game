use bevy::{prelude::*};


use std::{time::Duration, collections::HashMap};
use lab_entities::world;


#[allow(dead_code)]
pub mod stage {
    pub const WORLD: &'static str = "init";
}

mod systems;

pub struct SpritesPlugin;

impl Plugin for SpritesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(SpriteLibrary::new())
            .add_startup_system(crate::systems::load_world_sprites_system.system())
            .add_system(crate::systems::text_despawn.system());
    }
}


#[derive(Clone, Debug)]
pub struct Sprite {
    pub name:  &'static str,
    pub atlas_sprite : u32,
    pub atlas_handle : Handle<TextureAtlas>,
    pub height: u32,
    pub width: u32
}

struct Letter;

pub struct SpriteLibrary {
    library: Box<HashMap<&'static str, Sprite>>
}

impl SpriteLibrary {
    pub fn new () -> SpriteLibrary {
        SpriteLibrary {
            library : Box::new(HashMap::new())
        }
    }

    pub fn add(&mut self,sprite: Sprite){
        self.library.as_mut().insert(sprite.name, sprite);
    }

    pub fn get(&self, name : &str) -> Option<&Sprite> {
        println!("Trying to get {} as a sprite",name);
        self.library.get(name)
    }

    pub fn make_string(&self, st : String, mut location : Vec3) -> Vec<SpriteSheetComponents> {
        let mut sprites = Vec::<SpriteSheetComponents>::new();
        
        for c in st.to_lowercase().chars().into_iter() {
            if c == ' ' {
                *location.x_mut() += 8.;
                continue;
            }
            if let Some(sprite) = self.get(&format!("l_{}", c)){
                sprites.push(sprite.to_components(location));
                *location.x_mut() += 14.;
            }
        }

        sprites
    }

    pub fn write_despawning_text(&self,  
        mut commands :&mut Commands,
        st : String, 
        duration : Duration, 
        mut location : Vec3){
        self.make_string(st, location).into_iter().for_each(move |c| {
            commands.spawn(c).with(world::Despawn).with(Timer::new(duration));
        });
    }
}

impl Sprite {
    pub fn new (name : &'static str, sprite_idx: u32, handle: Handle<TextureAtlas>, width: u32, height: u32) -> Sprite {
         return Sprite {
             name: name.clone(),
             atlas_sprite: sprite_idx,
             atlas_handle: handle,
             width,
             height
         }
    }

    pub fn to_components(&self, loc : Vec3) -> SpriteSheetComponents {
        SpriteSheetComponents {
            translation: Translation::new(loc.x(), loc.y(), loc.z()),
            scale: Scale(1.0),
            draw: Draw { is_visible: true, ..Default::default() },
            sprite: TextureAtlasSprite::new(self.atlas_sprite),
            texture_atlas: self.atlas_handle.clone(),
            ..Default::default()
        }
    }
}
