pub fn next_prime(nbr: usize) -> usize {
    if nbr <=2{
        return 2;
    }
    let mut n =nbr;
    loop{
        if is_prime(n){
            return n;
        }
        n+=1;
    }
}
fn is_prime(n : usize)-> bool{
    if n<2{
        return false;
    }
    if n==2{
        return true;
    }
    if n %2==0{
        return false;
    }
    let mut  i = 3;
    while i*i<=n{
        if n%i==0{
            return false;
        }
        i +=2;
    }
    true
}
