
#[derive(PartialEq)]
pub enum BeerStyle {
    PaleAle,
    IPA,
    DIPA,
    RyePA,
    Stout,
    Porter,
    Sour,
    KettleSour,
    Wheat,
    Blond,
    Lager,
    PaleLager,
    Pilsner,
    CreamAle,
    FruitBeer,
    RedAle,
    Gose,
    Tripple,
    Double,
    Trappist
}

pub struct Beer {
    pub name : &'static str,
    pub style : BeerStyle,
    pub price: u32,
    pub quality: f32
}

impl Good for Beer {
    fn price(&self) -> u32{
        self.price
    }
    fn quality(&self) -> f32{
        self.quality
    }
}

pub struct Distributor {
    
}
pub struct Server {
    
}

impl Conveyer for Distributor {

}


