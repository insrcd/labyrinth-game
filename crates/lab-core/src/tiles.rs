use crate::{prelude::*};
use std::fmt::Debug;
use std::collections::HashMap;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ParsableState {
  string_value: Option<String>,
  bool_value: Option<bool>,
  int_value: Option<i32>
}

pub struct StateParseErr {
  message: &'static str
}

impl StateParseErr {
  fn new(message: &'static str) -> Self {
    StateParseErr {
      message: message
    }
  }
}

impl Debug for StateParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("State Error: {}", self.message))
    }
  }

#[derive(Clone, Default)]
pub struct Items {
    pub items: HashMap<WorldHandle<Item>, Entity>
}

impl Items {
  pub fn add(&mut self, components : ItemComponents, entity: Entity) -> &Items {
    self.items.insert(components.handle, entity);

    self
  }

  pub fn make(&mut self, commands : &mut Commands) -> WorldHandle<Item> {
    let handle:WorldHandle<Item> = WorldHandle::default();

    commands.spawn(ItemComponents {
      handle: handle,
      ..Default::default()
    }).for_current_entity(|f|{
      self.items.insert(handle, f);
    });

    handle
  }
}
/// Component for tracking tile state
/// e.g. val true_or_false : bool = tile_state.get("is_open".into()).into()
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectState {
    pub values : HashMap<String, ParsableState> 
}

impl Default for ObjectState {
    fn default() -> Self {
        ObjectState {
          values: HashMap::new()
        }
    }
}

impl ObjectState { 
  pub fn set_int(&mut self, key: String, value: i32){
    self.values.insert(key, ParsableState {int_value: Some(value), ..Default::default()});
  } 
  pub fn set_bool(&mut self, key: String, value: bool){
    self.values.insert(key, ParsableState {bool_value: Some(value), ..Default::default()});
  } 
  pub fn set_string(&mut self, key: String, value: String){
    self.values.insert(key, ParsableState {string_value: Some(value), ..Default::default()});
  } 
  
  pub fn get (&self, key : String) -> Result<ParsableState, StateParseErr> {
    match self.values.get(&key) {
      Some(state) => { Ok( state.clone() ) }
      None => Err(StateParseErr::new("Key does not exist in tile state" ))
    }
  }

  pub fn merge(&self, _other : Self){
    todo!()
  }
}

impl Into<bool> for ParsableState {
    fn into(self) -> bool {
      if self.bool_value == Some(true) {
        true
      } else {
        false
      }
    }
}

impl Into<Option<String>> for ParsableState {
  fn into(self) -> Option<String> {
    if let Some(value) = self.string_value {
      Some(value.clone())
    } else {
      None
    }
  }
}

impl Into<Option<i32>> for ParsableState {
  fn into(self) -> Option<i32> {
    if let Some(value) = self.int_value {
      Some(value)
    } else {
      None
    }
  }
}

impl Into<i32> for ParsableState {
  fn into(self) -> i32 {
    if let Some(value) = self.int_value {
      value
    } else {
      0
    }
  }
}
impl Into<Option<u32>> for ParsableState {
  fn into(self) -> Option<u32> {
    if let Some(value) = self.int_value {
      if value >= 0 {
        Some(value as u32)
      } else {
        None
      }
    } else {
      None
    }
  }
}
