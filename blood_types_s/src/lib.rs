#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum Antigen {
     A,
     AB,
     B,
     O,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum RhFactor {
     Positive,
     Negative,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}
impl BloodType {
    pub fn can_receive_from(self, other: Self) -> bool {
        if self.rh_factor== RhFactor::Negative&&other.rh_factor==RhFactor::Positive{
            return false;
        }
        match self.antigen{
            Antigen::O=>other.antigen==Antigen::O,
            Antigen::A=>{other.antigen==Antigen::A||other.antigen==Antigen::O},
            Antigen::B=>{other.antigen==Antigen::B||other.antigen==Antigen::O},
            Antigen::AB=>true,
        }
    }

    pub fn donors(self) -> Vec<Self> {
    let all = vec![
        BloodType{antigen:Antigen::A,rh_factor:RhFactor::Positive},
        BloodType{antigen:Antigen::A,rh_factor:RhFactor::Negative},
        BloodType{antigen:Antigen::B,rh_factor:RhFactor::Positive},
        BloodType{antigen:Antigen::B,rh_factor:RhFactor::Negative},
        BloodType{antigen:Antigen::O,rh_factor:RhFactor::Positive},
        BloodType{antigen:Antigen::O,rh_factor:RhFactor::Negative},
        BloodType{antigen:Antigen::AB,rh_factor:RhFactor::Positive},
        BloodType{antigen:Antigen::AB,rh_factor:RhFactor::Negative},
    ];
    let mut res=Vec::new();
    for x in all{
        if self.can_receive_from(x){
            res.push(x);
        }
    }
    res
    }

    pub fn recipients(self) -> Vec<Self> {
        let all = vec![
        BloodType{antigen:Antigen::A,rh_factor:RhFactor::Positive},
        BloodType{antigen:Antigen::A,rh_factor:RhFactor::Negative},
        BloodType{antigen:Antigen::B,rh_factor:RhFactor::Positive},
        BloodType{antigen:Antigen::B,rh_factor:RhFactor::Negative},
        BloodType{antigen:Antigen::O,rh_factor:RhFactor::Positive},
        BloodType{antigen:Antigen::O,rh_factor:RhFactor::Negative},
        BloodType{antigen:Antigen::AB,rh_factor:RhFactor::Positive},
        BloodType{antigen:Antigen::AB,rh_factor:RhFactor::Negative},
    ];
    let mut res=Vec::new();
    for x in all{
        if x.can_receive_from(self){
            res.push(x);
        }
    }
    res
    }
}

