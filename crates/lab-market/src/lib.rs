

pub enum ConsumerType {
    Individual,
    Business,
    Distributor
}

pub struct ProducerErr <T : Good + Sized> {
    pub reason : String,
    pub result : Vec<T>
}

pub trait Producer <T : Sized + Good, M : Sized + Good> {
    fn produce (materials: [M]) -> Result<Box<T>, ProducerErr<M>>; 
}
pub trait Consumer {
    fn evaluate(good : impl Good) -> u32;
    fn consume(good : impl Good) -> bool;
}

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
pub trait Good {
    fn price(&self) -> u32;
    fn quality(&self) -> f32;
}

impl Good for Beer {
    fn price(&self) -> u32{
        self.price
    }
    fn quality(&self) -> f32{
        self.quality
    }
}

/// Identifies a identity as a converer or someone who brings something from one inventory to another
pub struct Conveyer {

}