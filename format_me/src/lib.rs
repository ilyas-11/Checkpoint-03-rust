use std::fmt;

pub struct Park {
    pub name : Option<String>,
    pub park_type : ParkType,
    pub address : Option<String>,
    pub cap : Option<String>,
    pub state : Option<String>,
}

pub enum ParkType {
    Garden,
    Forest,
    Playground,
}

impl fmt::Display for Park {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
         let name= match &self.name{
            Some(s)=>s,
            None => "No name",
         };
         let address= match &self.address{
            Some(s)=>s,
            None => "No address",
         };
         let cap= match &self.cap{
            Some(s)=>s,
            None => "No cap",
         };
         let state= match &self.state{
            Some(s)=>s,
            None => "No state",
         };
         write!(f,"{} - {}, {}, {} - {}" , self.park_type,name,address,cap,state)
    }
}

impl fmt::Display for ParkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParkType::Garden=>write!(f,"garden"),
            ParkType::Forest=>write!(f,"forest"),
            ParkType::Playground=>write!(f,"playground"),
        }
    }
}
