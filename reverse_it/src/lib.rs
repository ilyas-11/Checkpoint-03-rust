pub fn reverse_it(v: i32) -> String {
let res= v.abs().to_string();
let rev:String=res.chars().rev().collect();
if v<0{
    return format!("-{}{}",rev,res);
}
format!("{}{}",rev,res)
}
