use std::fmt;

#[derive(Debug, Clone)]
pub struct Matrix(pub Vec<Vec<i32>>);

impl Matrix {
    pub fn new(slice: &[&[i32]]) -> Self {
        Self(
            slice.iter().map(|r|r.to_vec()).collect(),
        )
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (r,row) in self.0.iter().enumerate() {

            write!(f,"(")?;

            for (i,value) in row.iter().enumerate() {

                if i>0{
                    write!(f," ")?;
                }

                write!(f,"{}",value)?;
            }

            write!(f,")")?;
            write!(f,"{}",r)?;

            if r+1!=self.0.len(){
                writeln!(f)?;
            }
        }

        Ok(())
    }

}
