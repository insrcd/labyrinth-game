use bevy::{prelude::{Translation, Properties}, math::{Vec2, Vec3}, render::camera::Camera, window::Window};
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WorldLocation {
    World,
    Inventory,
    Labyrinth,
    BarRoom
}
#[derive(Clone, Debug, Copy, PartialEq, Properties)]
pub struct Location (pub f32, pub f32, pub f32, 
    #[property(ignore)] pub WorldLocation);


impl Default for Location {
    fn default() -> Self {
        return Location(0.,0.,0.,WorldLocation::World)
    }
    
}
impl From<Location> for Vec3 {
    fn from(x: Location) -> Self {
        Vec3::new(x.0, x.1, x.2)
    }
    
}

impl Location {
    pub fn normalize( window: &Window, 
            cam_transition: &Translation,  
            position : &Vec2) -> Vec2 {

        let camera_offset_x : f32 = cam_transition.x();
        let camera_offset_y : f32 = cam_transition.y() ;
    
        let x_window_offset = window.width;
        let y_window_offset = window.height;
        
        let normalized_x = position.x() + camera_offset_x - (x_window_offset/2) as f32;
        let normalized_y = position.y() + camera_offset_y - (y_window_offset/2) as f32;

        return Vec2::new(normalized_x, normalized_y);
    }
}

impl From<Translation> for Location {
    fn from(t : Translation) -> Self {
        Location (t.0.x(), t.0.y(), t.0.z(), WorldLocation::World)
    }
}
#[derive(Clone, PartialEq)]
pub struct Area(pub f32, pub f32);

#[derive(Copy, Clone, Debug)]
pub struct Visible;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Hardness (pub f32);

impl Default for Hardness {
    fn default() -> Hardness {
        return Hardness(1.)
    }
}
