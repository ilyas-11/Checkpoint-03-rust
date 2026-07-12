pub fn inv_pyramid(v: String, i: usize) -> Vec<String> {
    let mut res =Vec::new();

    for x in 1..=i{
        let line = format!("{}{}"," ".repeat(x),v.repeat(x));
        res.push(line);
    } 
    for x in (1..i).rev(){
        let line = format!("{}{}"," ".repeat(x),v.repeat(x));
        res.push(line);
    } 
    res
}
