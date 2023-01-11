///element of S_n
struct NCycle {
    cycles: Vec<u64>,
    n: u64,
}

//or trait, since A_n /subset S_n?

///element of A_n
struct NCycleAlternating {
    cycles: Vec<u64>,
    n: u64,
}

//functions to implement
//
//  sign
//  order
//  inverse
