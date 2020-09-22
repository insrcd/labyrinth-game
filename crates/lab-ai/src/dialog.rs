use lab_world::TextChangeEvent;
use lab_core::prelude::*;
use lab_entities::prelude::*;

#[derive(Default)]
struct DialogState {
  dialog_events : EventReader<DialogEvent>
}
#[derive(Clone, Debug)]
pub struct DialogEvent {
    pub text : String,
    pub entity: Entity
}

/*
pub fn dialog_system (
  mut text: ResMut<Events<TextChangeEvent>>,
  windows: Res<Windows>,
  dialog_state: Res<DialogState>,
  
  mut player_query : Query<(Entity, &Player, &Transform)>,
  mut m_query: Query<(Entity, &NonPlayer, &Transform, &Named)>,
  mut camera_query: Query<(&Camera, &Transform)>
) {
  let window = windows.iter().last().unwrap();
  let c_trans = camera_query.iter()
      .into_iter()
      .filter_map(|(c,t)| if c.name == Some("Camera2d".to_string()) { Some(t) } else {None} )
      .last();
       
  if let Some(t) = player_query.iter().into_iter().last() {
      for (_entity, dialog, translation) in &mut query.iter() {  
          let distance = Vec2::new((*t.2).x(), (*t.2).y()) - Vec2::new(translation.x(), translation.y());
          if distance.x().abs() + distance.y().abs() < 100. { 
              if let Ok(named) = &mut m_query.get::<Named>(dialog.entity) {     
                  text.send(TextChangeEvent { text: format!("{} says: \"{}\"", named.0, dialog.text).to_string(), name: "log".to_string()});
              }
          }

      }
  }  
}*/
