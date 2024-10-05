// Rewrite the factorial function using a `for` loop.
fn factorial(mut n:u32)->u32{
    let mut fac=1;
        for mut i in 1..=n {
            fac *=i;
            i+=1;
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
