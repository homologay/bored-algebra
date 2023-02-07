//base object and morphism traits that are extended in group, ring, algebra, module

///An object in a category
pub trait Object {}

///A morphism between objects in a category
pub trait Morphism<A, B>: Fn(A) -> B
where
    A: Object,
    B: Object,
{
    ///Returns the identity morphism $1$, so $f\circ 1 =
    fn ident() -> Self;
}

fn compose<A, B, C, G, F>(g: G, f: F) -> (impl Fn(A) -> C)
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x)) //'move' captures environment of closure
}
