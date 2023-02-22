pub struct Cache {
    pub ttl: u16
}

impl Cache {
    pub fn new() -> Self {
        Cache {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
