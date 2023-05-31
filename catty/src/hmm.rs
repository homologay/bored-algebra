use frunk_core::{hlist, hlist::HList, poly_fn, Coprod, HList};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mapping_types_following_rules() {
        let H: HList = hlist![420, "hello worlwd", 32.43, 2.91, "5"];
        let mapped = H.map(poly_fn![
           |n1: usize| -> usize { n1 / 10 },
           ['a] |s: &'a str| -> &'a str { "bye" },
           |f1: f32| -> f32 { f1 + 0.1 },
           |f2: f32| -> f32 { f2 - 1.0 },
           ['b] |s2: &'b str| -> &'b str {"6"},
        ]);
        assert_eq!(mapped, hlist![42, "bye", 32.53, 1.91, "6"]);
    }

    #[test]
    fn mapping_types_breaking_rules() {
        let G: HList = hlist![3, 3.1];
        let mapped = G.map(poly_fn![|n: usize| -> usize { n }, |f: f32| -> usize { 1 },]);
        assert_eq!(mapped, hlist![3, 1]);
    }
}
