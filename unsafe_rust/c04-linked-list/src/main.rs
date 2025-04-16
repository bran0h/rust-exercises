use std::ops::Range;

struct Node {
    pub first: u64,
    pub rest: LinkedList,
}

struct LinkedList(*const Node);

impl LinkedList {
    fn range(range: Range<u64>) -> Self {
        let mut list = LinkedList(std::ptr::null());
        for value in range.rev() {
            let node = Node {
                first: value,
                rest: list,
            };
            list = LinkedList(Box::into_raw(Box::new(node)));
        }

        list
    }

    fn sum(&self) -> u64 {
        if self.0.is_null() {
            0
        } else {
            let node = unsafe { &*self.0 };
            node.first + node.rest.sum()
        }
    }

    fn reverse(&self) -> Self {
        let mut reversed = LinkedList(std::ptr::null());
        for item in self.into_iter() {
            let node = unsafe { &*item.0 };
            let new_node = Node {
                first: node.first,
                rest: reversed,
            };
            reversed = LinkedList(Box::into_raw(Box::new(new_node)));
        }
        reversed
    }
}

struct LinkedListIter<'a> {
    next: Option<&'a LinkedList>,
}

impl<'a> Iterator for LinkedListIter<'a> {
    type Item = &'a LinkedList;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next;
        self.next = match current {
            Some(list) if !list.0.is_null() => {
                let node = unsafe { &*list.0 };
                Some(&node.rest)
            }
            _ => None,
        };
        current
    }
}

impl<'a> IntoIterator for &'a LinkedList {
    type Item = &'a LinkedList;
    type IntoIter = LinkedListIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListIter { next: Some(self) }
    }
}

impl Drop for LinkedList {
    fn drop(&mut self) {
        // Don't drop anything if the pointer is null
        let head = self.0;
        if head.is_null() {
            return;
        }
        let mut list = unsafe { (*self.0).rest.0 };
        while !list.is_null() {
            let node = unsafe { Box::from_raw(list as *mut Node) };
            list = node.rest.0;
            // The node is dropped here
        }
    }
}

fn main() {
    let list = LinkedList::range(1..3);
    let mut size = std::mem::size_of_val(&list);
    for item in (&list).into_iter() {
        size += std::mem::size_of_val(&item);
        println!("Deep size of list: {}", size);
    }
    println!("Deep size of list: {}", size);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum() {
        let list = LinkedList::range(1..10);
        assert_eq!(list.sum(), 45);
    }

    #[test]
    fn test_drop() {
        let list = LinkedList::range(1..10);
        let mut size = std::mem::size_of_val(&list);
        for item in (&list).into_iter() {
            size += std::mem::size_of_val(&item);
            println!("Deep size of list: {}", size);
        }
        println!("Deep size of list: {}", size);
        // List is dropped here
    }

    #[test]
    fn test_reverse() {
        let list = LinkedList::range(1..10);
        let reversed = list.reverse();
        let expected = [9, 8, 7, 6, 5, 4, 3, 2, 1];
        let mut i = 0;
        for item in (&reversed).into_iter() {
            if !item.0.is_null() {
                let node = unsafe { &*item.0 };
                assert_eq!(node.first, expected[i]);
                i += 1;
            }
        }
        // Check the sum of the reversed list
        assert_eq!(reversed.sum(), 45);
    }
}
