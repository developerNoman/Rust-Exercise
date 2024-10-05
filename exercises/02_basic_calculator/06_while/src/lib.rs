// Rewrite the factorial function using a `while` loop.

fn factorial(mut n:u32)->u32{
    let mut fac=1;
    if(n==0){
       return 1;
    }
    else{
       while(n>=1){
        fac *=n;
        n=n-1;
       }
    }
    fac
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
