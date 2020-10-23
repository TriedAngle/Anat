//
// 0 = {}
// NatNum::Zero
// 1 = { {} }
// NatNum::Mult([NatNum::Zero])
// 2 = { {}, { {} } }
// NatNum::Mult([NatNum::Zero, NatNum::Mult([NatNum::Zero])])
// 3 = { {}, { {} }, { {}, { {} } } }
// NatNum::Mult([NatNum::Zero, NatNum::Mult([NatNum::Zero]), NatNum::Mult([NatNum::Zero, NatNum::Mult([NatNum::Zero])])
#[derive(PartialEq, Debug, Clone)]
pub enum NatNum {
    Zero,
    Mult(Vec<NatNum>),
}

impl NatNum {
    pub fn to_number(&self) -> u32 {
        match self {
            NatNum::Zero => return 0,
            NatNum::Mult(nat_num) => return simple_nat_to_num(nat_num),
        }
    }
}

/// returns the count of the elements
/// doesn't check the 'correctness of the tree' / 'Wohlfundierung'
pub fn simple_nat_to_num(num: &Vec<NatNum>) -> u32 {
    return num.len() as u32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_zero() {
        let zero = NatNum::Zero;
        assert_eq!(zero.to_number(), 0);
    }

    #[test]
    fn create_one() {
        let one = NatNum::Mult(vec![NatNum::Zero]);
        assert_eq!(one.to_number(), 1)
    }

    #[test]
    fn create_two() {
        let two = NatNum::Mult(vec![NatNum::Zero, NatNum::Mult(vec![NatNum::Zero])]);
        assert_eq!(two.to_number(), 2)
    }

    #[test]
    #[rustfmt::skip]
    fn create_three() {
        let three = NatNum::Mult(vec![
            NatNum::Zero,
            NatNum::Mult(vec![
                NatNum::Zero
                ]),
            NatNum::Mult(vec![
                NatNum::Zero,
                NatNum::Mult(vec![
                    NatNum::Zero
                    ])
                ]),
            ]);

        assert_eq!(three.to_number(), 3)
    }
}
