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
       for r in &self.0{
        let line = r.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
        writeln!(f,"({})",line)?;
       }
       Ok(())
    }

}
