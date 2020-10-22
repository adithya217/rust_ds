
use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

pub type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct Node<T> {
    data: T,
    next: Link<T>,
    prev: Link<T>
}

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize
}

trait ILink<T> {
    fn create_raw(data: T) -> Rc<RefCell<Node<T>>>;
    fn peek(&self) -> Option<Ref<T>>;
    fn peek_mut(&mut self) -> Option<RefMut<T>>;
    fn borrow(&self) -> Link<T>;
}

impl<T> ILink<T> for Link<T> {
    fn create_raw(data: T) -> Rc<RefCell<Node<T>>> {
        let node = Node {
            data: data,
            next: None,
            prev: None
        };

        Rc::new(RefCell::new(node))
    }

    fn peek(&self) -> Option<Ref<T>> {
        self.as_ref().map(|rc_cell| {
            Ref::map(rc_cell.borrow(), |node| &node.data)
        })
    }

    fn peek_mut(&mut self) -> Option<RefMut<T>> {
        self.as_ref().map(|rc_cell| {
            RefMut::map(rc_cell.borrow_mut(), |node| &mut node.data)
        })
    }

    fn borrow(&self) -> Link<T> {
        self.as_ref().map(|rc_cell| {
            Rc::clone(rc_cell)
        })
    }
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List {
            size: 0,
            head: None,
            tail: None
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn peek_first(&self) -> Option<Ref<T>> {
        self.head.peek()
    }

    pub fn peek_last(&self) -> Option<Ref<T>> {
        self.tail.peek()
    }

    pub fn append(&mut self, data: T) {
        let new_tail = Link::create_raw(data);
        self.tail = match self.tail.take() {
            None => {
                self.head = Some(Rc::clone(&new_tail));
                Some(new_tail)
            },
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(Rc::clone(&new_tail));
                new_tail.borrow_mut().prev = Some(old_tail);
                Some(new_tail)
            }
        };
        self.size += 1;
    }

    pub fn prepend(&mut self, data: T) {
        let new_head = Link::create_raw(data);
        self.head = match self.head.take() {
            None => {
                self.tail = Some(Rc::clone(&new_head));
                Some(new_head)
            },
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::clone(&new_head));
                new_head.borrow_mut().next = Some(old_head);
                Some(new_head)
            }
        };
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                None => {
                    self.tail.take();
                    self.size = 0;
                },
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                    self.size -= 1;
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().data
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                None => {
                    self.head.take();
                    self.size = 0;
                },
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                    self.size -= 1;
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().data
        })
    }

    pub fn reverse(&mut self) {
        if self.size <= 1 {
            return;
        }

        let head = self.head.take();
        let tail = self.tail.take();

        let mut curr = head.borrow();
        while curr.is_some() {
            curr = {
                let mut curr_node = curr.as_ref().unwrap().borrow_mut();
                let prev = curr_node.prev.take();
                let next = curr_node.next.take();
                
                match (prev, next) {
                    (Some(prev_node), Some(next_node)) => {
                        // This case occurs when in middle of the list
                        curr_node.prev = Some(Rc::clone(&next_node));
                        curr_node.next = Some(Rc::clone(&prev_node));
                        Some(Rc::clone(&next_node))
                    },
                    (None, Some(next_node)) => {
                        // This case occurs when at start of the list
                        curr_node.prev = Some(Rc::clone(&next_node));
                        curr_node.next = None;
                        Some(Rc::clone(&next_node))
                    },
                    (Some(prev_node), None) => {
                        // This case occurs when at end of the list
                        curr_node.prev = None;
                        curr_node.next = Some(Rc::clone(&prev_node));
                        None
                    },
                    (None, None) => {
                        // This case occurs when list is empty
                        None
                    }
                }
            }
        }

        self.head = tail;
        self.tail = head;
    }
}