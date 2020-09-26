use std::{fs};
use defaults::*;

use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Defaults, PartialEq, Debug, Clone, Copy,Serialize,Deserialize)]
#[def = "String"]
pub enum AttributeType {
  String,
  UInt,
  Int,
  Bool,
  Float
}

#[derive(PartialEq, Debug, Clone, Serialize,Deserialize)]
pub struct Attribute {  
  attr_type: AttributeType,
  name: String,
  value: String
}

impl Into<String> for Attribute {
    fn into(self) -> String {
        self.value
    }
}
impl Into<u32> for Attribute {
  fn into(self) -> u32 {
      self.value.parse().unwrap()
  }
}
impl Into<bool> for Attribute {
  fn into(self) -> bool {
      self.value.parse().unwrap()
  }
}

impl Into<f32> for Attribute {
  fn into(self) -> f32 {
      self.value.parse().unwrap()
  }
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ItemDefinition {
  name: String,
  description: String,
  attributes: Vec<Attribute>
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MobDefinition {
  name: String,
  description: String,
  attributes: Vec<Attribute>,
  script: String
}

trait ItemReader {
  fn read_next(&mut self) -> ItemDefinition;
  fn has_next(&self) -> bool;
  fn reset(&mut self);
}

//#[derive()]
struct JsonFileReader<T> {
  items: Vec<T>,
  _filename : String,
  current: i32,
  count: usize,
 // _pd : &'a PhantomData<T>
}

impl <T> Default for JsonFileReader<T> {
    fn default() -> Self {
        JsonFileReader {
          //_pd: &PhantomData{},
          items: Default::default(),
          _filename: Default::default(),
          current: -1 as i32,
          count: 0          
        }
    }
}

impl <T> JsonFileReader <T> where T : DeserializeOwned + Clone {
  #[allow(dead_code)]
  fn new(filename : &str) -> JsonFileReader<T> {   
    let contents = fs::read_to_string(filename)
      .expect("Could not read file");
    let items : Vec<T> = serde_json::from_str(&contents[..]).unwrap();
    let len = items.len();
    JsonFileReader {
      _filename: filename.into(),
      items: items,
      current: -1,
      count: len,
      ..Default::default()
    }
  }
}

impl ItemReader for JsonFileReader<ItemDefinition> {

    fn read_next(&mut self) -> ItemDefinition {
       if self.has_next() {
        self.current+=1;
        
        self.items[self.current as usize].clone()
       } else {
         panic!("Nothing left.");
       }
    }

    fn has_next(&self) -> bool {
        self.current < self.count as i32 -1
    }

    fn reset(&mut self){
      self.current = -1;
    }
}

impl Iterator for JsonFileReader<ItemDefinition> {
    type Item=ItemDefinition;

    fn next(&mut self) -> Option<Self::Item> {
      if self.has_next() {
        Some(self.read_next())
      } else {
        None
      }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn serialize() {
        let item = ItemDefinition {
          name: "Test".into(),
          ..Default::default()
        };

        let vec = vec![item];

        fs::write("debug.json", serde_json::to_string(&vec).unwrap()).unwrap();

        let mut reader = JsonFileReader::new("debug.json");

        assert_eq!(true, reader.has_next());
        assert_eq!(reader.read_next().name, "Test");

        // test iteration

        reader.reset();

        let mut count = 0;
        for _ in reader {
          count+=1;
        }

        assert_eq!(1, count);

        fs::remove_file("debug.json").unwrap();
    }
    #[test]
    fn item_attribtues() {
      let mut item = ItemDefinition {
        name: "Test".into(),
        ..Default::default()
      };

      item.attributes.push(Attribute { name:"one".into(), value: "1".into(), attr_type: AttributeType::Int});

      let vec = vec![item];

      let data = serde_json::to_string_pretty(&vec).unwrap();
      println!("{}",data);
      
      let item_vec : Vec<ItemDefinition> = serde_json::from_str(&data).unwrap();

      assert_eq!(item_vec[0].attributes[0].name, "one");
      let coerced_int : u32 = item_vec[0].attributes[0].clone().into();
      assert_eq!(coerced_int, 1);
    }
}