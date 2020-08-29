use crate::{Zoomable, prelude::*};
use std::fmt::Debug;
use std::{sync::{Arc, Mutex}, collections::HashMap};

#[derive(Default, Clone, Debug)]
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
/// Component for tracking tile state
/// e.g. val true_or_false : bool = tile_state.get("is_open".into()).into()
#[derive(Clone, Debug)]
pub struct ObjectState {
    pub values : Arc<Mutex<HashMap<String, ParsableState>>>    
}

impl Default for ObjectState {
    fn default() -> Self {
        ObjectState {
          values: Arc::new(Mutex::new(HashMap::new()))
        }
    }
}

impl ObjectState { 
  pub fn set_int(&mut self, key: String, value: i32){
    self.values.lock().expect("could not get lock").insert(key, ParsableState {int_value: Some(value), ..Default::default()});
  } 
  pub fn set_bool(&mut self, key: String, value: bool){
    self.values.lock().expect("could not get lock").insert(key, ParsableState {bool_value: Some(value), ..Default::default()});
  } 
  pub fn set_string(&mut self, key: String, value: String){
    self.values.lock().expect("could not get lock").insert(key, ParsableState {string_value: Some(value), ..Default::default()});
  } 
  
  pub fn get (&self, key : String) -> Result<ParsableState, StateParseErr> {
    if let Ok(map) = Arc::clone(&self.values).lock() {
      match map.get(&key) {
          Some(state) => { Ok( state.clone() ) }
          None => Err(StateParseErr::new("Key does not exist in tile state" ))
      }
    } else {
      Err(StateParseErr::new( "Could not get lock on state" ))
    }
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
