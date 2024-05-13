use std::marker::PhantomData;
use ac_library::{MapMonoid, Monoid};

impl Monoid for Node<usize> {
    type S = Self;
    fn identity() -> Self::S {
        Node(0)
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        todo!()
    }
}

impl MapMonoid for Map<usize> {
    type M = Node<usize>;
    type F = usize;
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        // *f.max(g)
        todo!()
    }
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        // Node(x.0.max(*f))
        todo!()
    }
    fn identity_map() -> Self::F {
        // 0
        todo!()
    }
}

#[derive(Clone)]
struct Node<T: Default + Clone>(T);
#[derive(Clone)]
struct Map<T: Default + Clone>(PhantomData<T>);
