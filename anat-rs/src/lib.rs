// # Definition
// NatNums are defined by:
// 0 = {}
// n = (n-1) U {n-1} 
// or an easier but kinda wrong definition:
// n = {{}, {n-1}}
//
// ## examples:
//
// 0 = {}
// NatNum::Zero
// 1 = { {} }
// NatNum::Mult([NatNum::Zero])
// 2 = { {}, { {} } }
// NatNum::Mult([NatNum::Zero, NatNum::Mult([NatNum::Zero])])
// 3 = { {}, { {} }, { {}, { {} } } }
// NatNum::Mult([NatNum::Zero, NatNum::Mult([NatNum::Zero]), NatNum::Mult([NatNum::Zero, NatNum::Mult([NatNum::Zero])])
//
// TODO: maybe change Vec<NatNum> to an array when const generics hit stable, to avoid heap allocations and potential runtime costs.
//
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

    /// a recursive adder for two NatNumbers / Nat Trees
    pub fn add_rec(&self, other: &NatNum) -> NatNum {
        // check if one of the numbers represents 0, if so return the other.
        // if both are 0 num2 is always returned but this doesn't matter.
        match &self {
            NatNum::Zero => return other.clone(),
            NatNum::Mult(tree1) => {
                match other {
                    NatNum::Zero => return self.clone(),
                    NatNum::Mult(tree2) => {
                        // compare the 'size' of both trees (their actual value)
                        // to do the cheaper computation
                        if tree1.len() >= tree2.len() {
                            let mut tree_data = tree1.clone();
                            recursive_nat_tree_increment(&mut tree_data, tree2.len() as u32);
                            return NatNum::Mult(tree_data)
                        } else {
                            let mut tree_data = tree2.clone();
                            recursive_nat_tree_increment(&mut tree_data, tree1.len() as u32);
                            return NatNum::Mult(tree_data)
                        }
                    }
                }
            }
        }
    }

    // TODO: Write a Tree to Number parser.
    pub fn to_number_checked(&self) -> u32 {
        unimplemented!()
    }

    pub fn to_string(&self) -> String {
        match self {
            NatNum::Zero => String::from("{}"),
            NatNum::Mult(tree) => {
                let mut graph = String::from("{}\n");
                recursive_nat_tree_to_string_simple(tree, &mut graph, 0);
                graph
            }
        }
    }
}

impl From<u32> for NatNum {
    fn from(num: u32) -> NatNum {
        if num == 0 {
            return NatNum::Zero
        }
        let mut nat_num_data: Vec<NatNum> = Vec::new();
        populate_nat_tree(&mut nat_num_data, num);
        let nat_num = NatNum::Mult(nat_num_data);

        nat_num
    }
}


/// returns the count of the elements
/// doesn't check the 'correctness of the tree' / 'Wohlfundierung'
fn simple_nat_to_num(num: &Vec<NatNum>) -> u32 {
    return num.len() as u32;
}


/// recursively populates a vector with a tree like structure of NatNumbers
/// curr: remaining iterations for each tree / subtree
/// it works by subtracting -1 from 'curr' in each iteration
/// than it looks for these cases:
/// curr = 0 => stop iteration e.g tree is fully populated
/// curr = 1 => Add a NatNum::Zero
/// curr = _ => Add a new subtree and recursively populates it (same function)
fn populate_nat_tree(tree: &mut Vec<NatNum>, curr: u32) {
    match curr {
        0 => return,
        1 => {
            tree.insert(0,NatNum::Zero);
            populate_nat_tree(tree, curr - 1);
        },
        _ => {
            let mut sub_tree_data = Vec::new();
            populate_nat_tree(&mut sub_tree_data, curr - 1);
            let sub_tree = NatNum::Mult(sub_tree_data);
            tree.insert(0,sub_tree);
            populate_nat_tree(tree, curr - 1)
        },
    }
}

// copies itself and pushes itself into itself as a new subtree 
fn recursive_nat_tree_increment(tree: &mut Vec<NatNum>, rem: u32) {
    if rem == 0 {
        return;
    }
    let new_subtree = tree.clone();
    let end_num = NatNum::Mult(new_subtree);
    tree.push(end_num);
    recursive_nat_tree_increment(tree, rem - 1)
}

// creates a string from the tree with intonations for each Subtree
// tree: vector of a natnum
// graph: String that should be manipulated
// level: current subtree level, used for intonation
fn recursive_nat_tree_to_string_simple(tree: &Vec<NatNum>, graph: &mut String, level: u32) {
    for x in tree {
        match x {
            NatNum::Zero => {
                graph.push_str(&format!("{}{{}}\n", simple_graph_intonation(level)));
            }
            NatNum::Mult(subtree) => {
                graph.push_str(&format!("{}{{}}\n", simple_graph_intonation(level)));
                recursive_nat_tree_to_string_simple(subtree, graph, level + 1)
            }
        }
    }
}

// example:
//
// |-> {}
// |-> {}
// |   |-> {}
fn simple_graph_intonation(level: u32) -> String {
    let mut intonation = String::new();
    intonation.push_str("|");
    for _ in 0..level {
        intonation.push_str("  |");
    }
    intonation.push_str("->");
    intonation
}

// TESTS
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_zero() {
        let num = NatNum::Zero;
        assert_eq!(num.to_number(), 0);
    }

    #[test]
    fn create_one() {
        let num = NatNum::Mult(vec![NatNum::Zero]);
        assert_eq!(num.to_number(), 1)
    }

    #[test]
    fn create_two() {
        let num = NatNum::Mult(vec![NatNum::Zero, NatNum::Mult(vec![NatNum::Zero])]);
        assert_eq!(num.to_number(), 2)
    }

    #[test]
    #[rustfmt::skip]
    fn create_three() {
        let num = NatNum::Mult(vec![
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

        assert_eq!(num.to_number(), 3)
    }

    #[test]
    fn from_zero() {
        let num = NatNum::from(0);
        assert_eq!(num, NatNum::Zero);
        assert_eq!(num.to_number(), 0);
    }

    #[test]
    fn from_one() {
        let num = NatNum::from(1);
        assert_eq!(num, NatNum::Mult(vec![NatNum::Zero]));
        assert_eq!(num.to_number(), 1);
    }

    #[test]
    #[rustfmt::skip]
    fn from_two() {
        let num = NatNum::from(2);
        assert_eq!(num, NatNum::Mult(vec![
            NatNum::Zero,
            NatNum::Mult(vec![
                NatNum::Zero
                ])
            ]));
        assert_eq!(num.to_number(), 2);
    }

    #[test]
    #[rustfmt::skip]
    fn from_three() {
        let num = NatNum::from(3);
        assert_eq!(num, NatNum::Mult(vec![
            NatNum::Zero,
            NatNum::Mult(vec![
                NatNum::Zero
                ]),
            NatNum::Mult(vec![
                NatNum::Zero,
                NatNum::Mult(vec![
                    NatNum::Zero
                    ])
            ])
        ]));
    }

    #[test]
    #[rustfmt::skip]
    fn from_four() {
        let num = NatNum::from(4);
        assert_eq!(num, NatNum::Mult(vec![
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
            NatNum::Mult(vec![
                NatNum::Zero,
                NatNum::Mult(vec![
                    NatNum::Zero
                    ]),
                NatNum::Mult(vec![
                    NatNum::Zero,
                    NatNum::Mult(vec![
                        NatNum::Zero
                        ])
                ])
            ])
        ]));
    }

    // this crashes my pc :( (i7 5930k & 16GB ram)
    // #[test]
    // fn stress_test_creation() {
    //     let num = NatNum::from(100_000);
    // }

    #[test]
    fn add_zeros() {
        let num1 = NatNum::from(0);
        let num2 = NatNum::from(0);
        let num3 = NatNum::from(2);

        assert_eq!(num1.add_rec(&num2), NatNum::from(0));
        assert_eq!(num1.add_rec(&num3), NatNum::from(2));
        assert_eq!(num3.add_rec(&num1), NatNum::from(2));
    }

    #[test]
    fn add_non_zero() {
        let num1 = NatNum::from(1);
        let num2 = NatNum::from(2);
        let num3 = NatNum::from(3);

        assert_eq!(num1.add_rec(&num2), NatNum::from(3));
        assert_eq!(num2.add_rec(&num2), NatNum::from(4));
        assert_eq!(num3.add_rec(&num2), NatNum::from(5));
    }
    
    #[test]
    fn prints() {
        let num0 = NatNum::from(0);
        let num1 = NatNum::from(1);
        let num4 = NatNum::from(4);
        assert_eq!(num0.to_string(), "{}");
        assert_eq!(num1.to_string(), "{}\n|->{}\n");
        assert_eq!(num4.to_string(), "{}\n|->{}\n|->{}\n|  |->{}\n|->{}\n|  |->{}\n|  |->{}\n|  |  |->{}\n|->{}\n|  |->{}\n|  |->{}\n|  |  |->{}\n|  |->{}\n|  |  |->{}\n|  |  |->{}\n|  |  |  |->{}\n");
        
    }
}