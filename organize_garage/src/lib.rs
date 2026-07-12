use std::ops::Add;

#[derive(Debug, PartialEq, Eq)]
pub struct Garage<T>{
    pub left : Option<T>,
    pub right : Option<T>,
}

impl<T> Garage<T> where T: Add<Output = T> + Copy,{
    pub fn move_to_right(&mut self){
        match (self.left,self.right){
            (Some(x),Some(y))=>{
                self.right = Some(x+y);
                self.left = None;
            }
            (Some(x),None)=>{
                self.right = Some(x);
                self.left = None;
            }
            _=>{}
        }
    }
    pub fn move_to_left(&mut self){
        match (self.right,self.right){
            (Some(x),Some(y))=>{
                self.left = Some(x+y);
                self.right = None;
            }
            (Some(x),None)=>{
                self.left = Some(x);
                self.right = None;
            }
            _=>{}
        }
    }
}