#[derive(Debug, Clone, Copy)]
pub struct ChessPosition {
    pub rank: i32,
    pub file: i32,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank<8&&rank>=0&&file<8&&file>=0{
            Some(Self{rank,file})
        }else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Queen {
    pub position: ChessPosition,
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self{position}
    }

    pub fn can_attack(self, other: Self) -> bool {
        let s=self.position;
        let o= other.position;
         s.rank==o.rank|| s.file==o.file|| 
            (s.rank as isize - o.rank as isize).abs()==(s.file as isize - o.file as isize).abs()

        
    }
}