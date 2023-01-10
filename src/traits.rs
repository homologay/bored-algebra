//! here lie traits for abstractions like Group, Ring, ...

///Group
trait Group::<Vec<T>> where T: Neg, Sub, Unit {
    todo!();
}

///Ring, a pretty one hopefully
trait Ring::<T> where T: Group {
    todo!();
}

///The property of a set with a binary operation "add" having a unit
trait Unit::<Vec<T>> where T: Add {
    todo!();
}

