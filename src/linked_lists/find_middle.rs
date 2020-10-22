
// use super::list::List;
// use std::cell::{Ref};

// impl<T> List<T> {
//     pub fn peek_middle(&self) -> Option<Ref<T>> {
//         if self.len() <= 1 {
//             return self.peek_first();
//         }

//         return None;
//     }
// }

// pub fn find_middle(list: &List<i32>) -> Ref<> {
//      if list.len() <= 1 {
//          return head;
//      }

//      let mut slow = Some(head);
//      let mut fast = Some(head);

//      while fast.unwrap().get_next() != None {
//          fast = fast.unwrap().get_next();
//          slow = slow.unwrap().get_next();

//          if fast.unwrap().get_next() == None {
//              break;
//          }

//          fast = fast.unwrap().get_next();
//      }

//      return slow.unwrap();
// }

// #[cfg(test)]
// mod tests {
//     use super::Node as Node;
//     use super::find_middle as compute;

//     #[test]
//     fn find_middle_of_single_linked_list() {
//         let input = Node::new(1, None);
//         let expected = &1;
//         let result = compute(input).get_data();
//         assert_eq!(expected, result);
//     }

//     #[test]
//     fn find_middle_of_small_odd_sized_linked_list() {
//         let third = Node::new(3, None);
//         let second = Node::new(2, Some(third));
//         let input = Node::new(1, Some(second));
        
//         let expected = &2;
//         let result = compute(input).get_data();
//         assert_eq!(expected, result);
//     }

//     #[test]
//     fn find_middle_of_medium_odd_sized_linked_list() {
//         let fifth = Node::new(5, None);
//         let fourth = Node::new(4, Some(fifth));
//         let third = Node::new(3, Some(fourth));
//         let second = Node::new(2, Some(third));
//         let input = Node::new(1, Some(second));

//         let expected = &3;
//         let result = compute(input).get_data();
//         assert_eq!(expected, result);
//     }

//     #[test]
//     fn find_middle_of_small_even_sized_linked_list() {
//         let second = Node::new(2, None);
//         let input = Node::new(1, Some(second));

//         let expected = &2;
//         let result = compute(input).get_data();
//         assert_eq!(expected, result);
//     }

//     #[test]
//     fn find_middle_of_big_even_sized_linked_list() {
//         let sixth = Node::new(6, None);
//         let fifth = Node::new(5, Some(sixth));
//         let fourth = Node::new(4, Some(fifth));
//         let third = Node::new(3, Some(fourth));
//         let second = Node::new(2, Some(third));
//         let input = Node::new(1, Some(second));

//         let expected = &4;
//         let result = compute(input).get_data();
//         assert_eq!(expected, result);
//     }
// }