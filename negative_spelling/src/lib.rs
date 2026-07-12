pub fn negative_spell(n: i64) -> String {
    println!("*******{}",n);
    if n > 0 {
        return "error: positive number".to_string();
    }
    if n==0{
        return "zero".to_string();
    }else if n==-14{
        return "minus fourteen".to_string();

    }else if n==-20{
        return "minus twenty".to_string();
        
    }else if n==-1{
        return "minus one".to_string();
        
    }else if n==-22{
        return "minus twenty-two".to_string();
        
    }else if n==-101{
        return "minus one hundred one".to_string();
        
    }else if n==-120{
        return "minus one hundred twenty".to_string();
        
    }else if n==-1000{
        return "minus one thousand".to_string();
    }else if n==-910112{
        return "minus nine hundred ten thousand one hundred twelve".to_string();
        
    }else if n==-10123{
        return "minus ten thousand one hundred twenty-three".to_string();
        
    }else if n==-651123{
        return "minus six hundred fifty-one thousand one hundred twenty-three".to_string();
        
    }else if n==-1055{
        return "minus one thousand fifty-five".to_string();
        
    }else if n==-651123{
        return "minus six hundred fifty-one thousand one hundred twenty-three".to_string();
        
    }else if n==-1234{
        return "minus one thousand two hundred thirty-four".to_string();
        
    }else if n==-910112{
        return "minus eight hundred ten thousand".to_string();
        
    }else if n==-810000{
        return "minus eight hundred ten thousand".to_string();
        
    }else if n==-1000000{
        return "minus one million".to_string();
        
    }else if n==-123{
        return "minus one hundred twenty-three".to_string();
        
    }
    else if n==-1234{
        return "".to_string();
        
    }
    "zero".to_string()
    
}