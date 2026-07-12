
#[derive(Debug, PartialEq, Eq)]
pub struct Outfit {
    pub jacket: Jacket,
    pub hat: Hat,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Jacket{
    Black,
    White,
    Flowers,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Hat{
    Snapback,
    Baseball,
    Fedora,
}
pub fn choose_outfit(formality_level: Option<u32>, invitation_message: Result<&str, &str>) -> Outfit {
    let mut outfit =Outfit{
        jacket: Jacket::Black,
        hat: Hat::Snapback,
    };
    match formality_level{
        None=>outfit.jacket=Jacket::Flowers,
        Some(0)=>outfit.jacket=Jacket::Black,
        Some(_)=>outfit.jacket=Jacket::White,

    }
    match invitation_message{
        Ok(_)=>outfit.hat=Hat::Fedora,
        Err(_)=>{
            if formality_level.is_none(){
                return Outfit{
                    jacket:Jacket::Flowers,
                    hat:Hat::Baseball,
                }
            }
            outfit.hat=Hat::Snapback
        }
    }
    outfit
}
