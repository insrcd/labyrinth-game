
use bevy::prelude::*;
use lab_entities::prelude::*;

use crate::*;
use lab_world::{TileComponents, TilePalette};
use std::{rc::Rc, cell::RefCell};

#[derive(Clone, Debug)]
pub struct Blueprint {
    pub name : String,
    pub tiles : Vec<TileComponents>
}

impl Blueprint {
    fn top_right(&self) -> (Vec2, Location) {
        let mut location = Location::default();
        let mut size : Vec2 = Vec2::new(0.,0.);
        
        for tiles in self.tiles.as_slice() {
            if tiles.location.0 > location.0 {
                location.0 = tiles.location.0;
                size = tiles.sprite.size();
            }
            if tiles.location.1 > location.1 {
                location.1 = tiles.location.1;
            }
        }

        (size, location)
    }
}

pub struct MapBuilder {
    pub tile_palette : Rc<RefCell<TilePalette>>,
    pub starting_location : Location,
    pub current_location : Location,
    pub tiles : Vec<TileComponents>,
    pub mobs : Vec<MobComponents>,
    pub blueprints : Vec<Blueprint>
}

impl<'a>  MapBuilder {
    pub fn new(palette: Rc<RefCell<TilePalette>>, starting_location: &Location) -> MapBuilder {
        MapBuilder {
            tile_palette : palette.clone(),
            starting_location : starting_location.clone(),
            current_location : starting_location.clone(),
            tiles: Vec::new(),
            blueprints: Vec::new(),
            mobs: Vec::new()
        }
    }
    pub fn reset_position(&mut self) -> &MapBuilder {
        self.current_location = self.starting_location;

        self
    }
    
    pub fn set_position(&mut self, location : Location) -> &mut MapBuilder{
        self.current_location = location;

        self
    }

    pub fn to_blueprint(&mut self, name : &str) -> Blueprint {
        let bp =Blueprint {
            name: name.to_string(),
            tiles: self.tiles.clone()
        };

        self.blueprints.push(bp.clone());

        self.tiles.clear();

        self.current_location = self.starting_location;

        println!("Adding blueprint loc reset to: {:?}", self.starting_location);

        bp
    }

    pub fn add_tiles_from_blueprint(&mut self, name : &str) -> &mut MapBuilder {
        
        println!("Adding blueprint tiles at {:?}", self.current_location);

        for bp in self.blueprints.iter() {
            if bp.name != name {
                continue;
            }
            
            let offset = bp.top_right();

            // right now just put to the right of the last tile
           
            for tile in bp.tiles.as_slice() {
                let mut c =  tile.clone();

                c.location.0 += self.current_location.0;
                c.location.1 += self.current_location.1;

                self.tiles.push(c);
            }

            
            // add size and offset from top right corner of last blueprint
            self.current_location.0 += (offset.0).x() + (offset.1).0;
            //self.current_location.1 += (offset.1).1 + (offset.0).y();

        }
        self
    }

    pub fn add_tiles_to_area(&mut self, loc : &Location, area: Area, tile_name: String) -> &mut Self {
       
        if let Some(comps) = self.tile_palette.borrow().components.get(&tile_name){
            for x in 0..area.0 as u32 {
                for y in 0..area.1 as u32 {  
                    let mut comp = comps.clone();
                    
                    comp.location = Location(loc.0 + (x * comp.sprite.width) as f32, loc.1 - (y * comp.sprite.height) as f32 , loc.2,  comp.location.3);       
                    println!("Location: {:?}", comp.location);
                    self.tiles.push(comp);            
                }
            }
        } else {
            println!("Cannot find tile definition for {}", tile_name);
        }
    
        self
    }
    pub fn add_tiles(&mut self, pos : RelativePosition, count : u32, tile_name: String) -> &mut Self {
        if let Some(comps) = self.tile_palette.borrow().components.get(&tile_name){
            for _ in 0..count {
                let mut my_comp = comps.clone();

                let loc = self.current_location;
                
                let tile_size_x = comps.sprite.width as f32;
                let tile_size_y = comps.sprite.height as f32;

                println!("Tile Size: {},{}", tile_size_x, tile_size_y);

                let location = match pos {
                    RelativePosition::LeftOf => {                                    
                        Location(loc.0 - tile_size_x, loc.1, loc.2, world::WorldLocation::World)
                    }
                    RelativePosition::RightOf => {
                        Location(loc.0 + tile_size_x, loc.1, loc.2, world::WorldLocation::World)
                    }
                    RelativePosition::Above => {
                        Location(loc.0, loc.1 + tile_size_y, loc.2, world::WorldLocation::World)
                    }
                    RelativePosition::Below => {
                        Location(loc.0, loc.1 - tile_size_y, loc.2, world::WorldLocation::World)
                    },
                    _ => self.current_location
                };

                println!("Adding tile at {:?} last location: {:?}", self.current_location, location);
                
                my_comp.location = location;
                println!("Hardness: {:?}", my_comp.hardness);

                self.tiles.push(my_comp);

                self.current_location = location;
            }
        }

        self
    }

    pub fn add_mobs(&mut self, mut pos : Location, count : u32, mob_name: String) -> &mut Self {
        if let Some(comps) = self.tile_palette.borrow().components.get(&mob_name){
            for instance_num  in 0..count {
                // modify z index because right now sprites are clipping if z = other_z
                pos.2 += instance_num as f32;

                self.mobs.push(MobComponents {
                    interaction: comps.interaction,
                    sprite: comps.sprite.clone(),
                    location: pos,
                    timer : Timer::from_seconds(2.0, false),
                    interactable_type: lab_core::InteractableType::Npc,
                    ..Default::default()
                });
            }
        }
        self
    }

    pub fn iter(&mut self) -> std::slice::Iter<'_, TileComponents> {
        self.tiles.iter()
    }

}