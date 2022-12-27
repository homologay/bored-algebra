
struct SymmetricGroup {
    todo!();
}

struct AlternatingGroup {
    todo!();
}


//maybe define in a categorical way, with diagrams, etc, as well as a set way. 
trait Group {
    todo!();
}

//was thinking of making morphisms just functions O -> O, but they need extra structure :<
struct Category<O, M> {
    objects: O,
    morphisms: M,
}

//need to implement category rules 
//
//  - can compose morphisms, composition is associative
//  - each object has id morphism
//  - composing with id does nothing
//
//  so should Category be a trait?? should all of this be traits?? probably, working at high levels
//  of abstractino here....

//just a vec with order forgotten and some methods .. is this a Good IDea?? who knows.. 
//Also, make a poset sometime. 
struct Set<E> {
    elements: Vec<E>,
}

    
    

