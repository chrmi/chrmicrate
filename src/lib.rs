pub fn chrmi() -> bool {
    println!();
    println!("=-=-=-=-=-=-=-=-=-=-=-=-=-=");
    println!("-- https://www.chrmi.com --");
    println!("=-=-=-=-=-=-=-=-=-=-=-=-=-=");
    println!();
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works(){
        assert!(chrmi());
    }
}
