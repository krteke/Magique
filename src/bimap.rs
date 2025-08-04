// use std::collections::HashMap;

// pub struct Bimap {
//     forward: HashMap<[u8; 4], [u8; 4]>,
//     backward: HashMap<[u8; 4], [u8; 4]>,
// }

// impl Bimap {
//     pub fn new() -> Bimap {
//         Bimap {
//             forward: HashMap::new(),
//             backward: HashMap::new(),
//         }
//     }

//     pub fn insert(&mut self, key: &[u8; 4], value: &[u8; 4]) -> &mut Self {
//         self.forward.insert(*key, *value);
//         self.backward.insert(*value, *key);

//         self
//     }

//     pub fn get_by_left(&self, key: &[u8; 4]) -> Option<[u8; 4]> {
//         self.forward.get(key).copied()
//     }

//     pub fn get_by_right(&self, value: &[u8; 4]) -> Option<[u8; 4]> {
//         self.backward.get(value).copied()
//     }
// }

// // #[cfg(test)]
// // mod tests {
// //     use super::*;
// // }
