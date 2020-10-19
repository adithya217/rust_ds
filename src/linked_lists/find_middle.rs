
#[derive(PartialEq, Debug)]
pub struct Node<'a> {
    data: i32,
    next: Option<&'a Node<'a>>
}

pub fn find_middle<'a>(head: &'a Node) -> &'a Node<'a> {
     if head.next == None {
         return head;
     }

     let mut slow = Some(head);
     let mut fast = Some(head);

     while fast.unwrap().next != None {
         fast = fast.unwrap().next;
         slow = slow.unwrap().next;

         if fast.unwrap().next == None {
             break;
         }

         fast = fast.unwrap().next;
     }

     return slow.unwrap();
}

#[cfg(test)]
mod tests {
    use super::Node as Node;
    use super::find_middle as compute;

    #[test]
    fn find_middle_of_single_linked_list() {
        let input = Node {
            data: 1,
            next: None
        };
        let expected = &input;
        assert_eq!(expected, compute(&input));
    }

    #[test]
    fn find_middle_of_small_odd_sized_linked_list() {
        let input = Node {
            data: 1,
            next: Some(&Node {
                data: 2,
                next: Some(&Node {
                    data: 3,
                    next: None
                })
            })
        };
        let expected = input.next.unwrap();
        assert_eq!(expected, compute(&input));
    }

    #[test]
    fn find_middle_of_small_medium_sized_linked_list() {
        let input = Node {
            data: 1,
            next: Some(&Node {
                data: 2,
                next: Some(&Node {
                    data: 3,
                    next: Some(&Node {
                        data: 4,
                        next: Some(&Node {
                            data: 5,
                            next: None
                        })
                    })
                })
            })
        };
        let expected = input.next.unwrap().next.unwrap();
        assert_eq!(expected, compute(&input));
    }

    #[test]
    fn find_middle_of_small_even_sized_linked_list() {
        let input = Node {
            data: 1,
            next: Some(&Node {
                data: 2,
                next: None
            })
        };
        let expected = input.next.unwrap();
        assert_eq!(expected, compute(&input));
    }

    #[test]
    fn find_middle_of_big_even_sized_linked_list() {
        let input = Node {
            data: 1,
            next: Some(&Node {
                data: 2,
                next: Some(&Node {
                    data: 3,
                    next: Some(&Node {
                        data: 4,
                        next: Some(&Node {
                            data: 5,
                            next: Some(&Node {
                                data: 6,
                                next: None
                            })
                        })
                    })
                })
            })
        };
        let expected = input.next.unwrap().next.unwrap().next.unwrap();
        assert_eq!(expected, compute(&input));
    }
}