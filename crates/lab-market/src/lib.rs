pub enum ConsumerType {
    Individual,
    Business,
    Distributor
}

pub struct ProducerErr <T : Good + Sized> {
    pub reason : String,
    // what materials are returned from the production process.
    pub result : Vec<T>
}

pub trait Producer <T : Sized + Good, M : Sized + Good> {
    fn produce (materials: [M]) -> Result<Box<T>, ProducerErr<M>>; 
}
pub trait Consumer {
    fn evaluate(good : impl Good) -> u32;
    fn consume(good : impl Good) -> bool;
}

pub trait Good {
    fn price(&self) -> u32;
    fn quality(&self) -> f32;
}


/// Identifies a identity as a converer or someone who brings something from one inventory to another
trait Conveyer {
    fn distribution_rate () -> u32;
   // fn convey (source : Inventory, destination: Inventory) -> bool;
}

