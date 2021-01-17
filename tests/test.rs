#![deny(unused_mut)]

#[macro_use]
extern crate proofsize_derive;

#[cfg(test)]
mod tests {
    use rug_binserial::*;



    #[derive(ProofSize)]
    pub struct Struct {
        x: i32,
        y: i32
    }

    #[test]
    fn test_struct() {
        
        let record = Struct { x: 51, y: 212 };
        assert_eq!(record.proof_size(), 2);

    }

    
}
