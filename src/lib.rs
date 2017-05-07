
use std::sync::Arc;
// use std::iter::FromIterator;

#[macro_export]
macro_rules! list { ($val:expr) => { Arc::new(($val)) } }

// struct Cons {}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum List<A> {
    Cons(A, ArcList<A>),
    Nil,
}

pub type ArcList<T> = Arc<List<T>>;

use List::*;
impl<A: Clone> List<A> {
    fn single(a: A) -> Self {
        Cons(a, Arc::from(Nil))
    }

    fn append(self, a: A) -> Self {
        Cons(a, Arc::from(self))
    }

    fn head_option(self) -> Option<A> {
        match self {
            Cons(h, _) => Some(h),
            _ => None,
        }
    }

    fn init(self) -> List<A> {
        match self {
            Cons(_, t) => t.as_ref().clone(),
            Nil => Nil,
        }

    }

    // pub fn fold_right<B, F: FnMut(A, B) -> B>(self, z: B, f: F) -> B {
    //     match self {
    //         Nil => z,
    //         Cons(h, a) => a.as_ref().map(|x| f(h, x.fold_right(z, f))),
    //     }
    // }

    // pub fn map<B, F: FnMut(&A) -> B>(self, f: &F) -> List<B> {
    //     match self {
    //         Cons(h, t) => Cons(f(&h), Arc::new(t.as_ref().map(f))),
    //         Nil => Nil,
    //     }
    // }
    //
    // pub fn foreach<F: FnMut(&A) -> ()>(self, f: &mut F) -> () {
    //     match self {
    //         Cons(h, t) => {
    //             Cons(f(&h), Arc::new(t.as_ref().map(f)));
    //
    //         }
    //         Nil => (),
    //     }
    // }
}

// impl<T> FromIterator<T> for ArcList<T> {
//     fn from_iter<I>(mut iter: I) -> ArcList<T>
//         where I: Iterator<Item = T> + 'static
//     {
//         #[inline]
//         list!({
//                   match iter.next() {
//                       Some(val) => Cons(val, FromIterator::from_iter(iter)),
//                       None => Nil,
//                   }
//               })
//     }
// }


// impl<'a, T> IntoIterator for &'a ArcList<T> {
//     type Item = T;
//     type IntoIter = IntoIter<Self::Item>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         let v = Vec::new();
//         self.foreach(v.push);
//         v.into
//
//     }
//
//     // fn next(&mut self) -> Option<T> {
//     //     let (value, rest) = match ****self {
//     //         Cons(ref value, ref rest) => (value, rest),
//     //         Nil => return None,
//     //     };
//     //
//     //     *self = rest;
//     //     Some(value)
//     // }
// }




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}

    // #[test]
    // fn create_an_empty_list() {
    //     assert!(Nil == Nil);
    // }
    //
    // fn create_single_element() {
    //
    //     assert!(Cons(5, Arc::new(Nil)) == List::single(5))
    // }

}
