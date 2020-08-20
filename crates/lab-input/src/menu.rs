
#[derive(Clone, Debug)]
pub struct Menu {
    title : String,
    items : Vec<MenuItem>,
    current_selection : Option<MenuItem>
}

impl Default for Menu {
    fn default() -> Menu {
        Menu {
            title: Default::default(),
            items: Vec::new(),
            current_selection: Option::None
        }
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct MenuItem {
    
}
