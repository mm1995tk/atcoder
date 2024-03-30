
// /// bit全探索のイテレータ
// struct BitPatternIterator<'a, T> {
//     vec: &'a Vec<T>,
//     current: usize,
//     max: usize,
// }

// impl<'a, T> BitPatternIterator<'a, T> {
//     fn new(vec: &'a Vec<T>) -> Self {
//         let n = vec.len();
//         Self {
//             vec,
//             current: 0,
//             max: 1 << n,
//         }
//     }
// }

// impl<'a, T> Iterator for BitPatternIterator<'a, T> {
//     type Item = Vec<&'a T>;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.current >= self.max {
//             return None;
//         }

//         let mut tmp: Vec<&'a T> = vec![];
//         for i in 0..self.max {
//             if self.current & (1 << i) != 0 {
//                 tmp.push(&self.vec[i]);
//             }
//         }

//         self.current += 1;

//         Some(tmp)
//     }
// }