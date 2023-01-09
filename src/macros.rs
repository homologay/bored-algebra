#![allow(unused_macros)]

//idea: a macro like finite_group!((a, b),
//                                  (abab, aB))
//
//                                  makes a group with presentation 
//
//                                  < a, b | abab, ab^{-1} >
//
//      (so there's not so much annoying bs like \< and such)
//
//      i think the argument given above could be expr?
//
//      look up rust reference

macro_rules! finite_group {
    //ident -- variable/function names
    ($func_name:ident) => {
        println!("you called {:?}()", stringify!($funct_name));
    };
}

//some available designators:
//
//  -block
//  -expr       expressions
//  -ident      var/fn names
//  -item
//  -literal    literal consts
//  -pat        pattern
//  -path
//  -stmt       statement
//  -tt         token tree
//  -ty         type
//  -vis        visibility qualifier
