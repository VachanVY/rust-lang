#![allow(warnings)]

#[derive(Clone)]
struct List {
    el: i32,
    next: Box<Option<List>>
}

impl List {
    fn new(val: i32) -> List {
        return List {
            el: val,
            next: Box::new(None)
        }
    }

    fn from(arr: &[i32]) -> List {
        let mut head = List::new(arr[0]);

        let mut current = &mut head;
        for &val in &arr[1..] {
            /* current.next = Box::new(Some(List::new(val)));
            current = (*current.next).as_mut().unwrap(); */
            current.append(val);
        }
        return head
    }

    fn append(&mut self, val: i32) {
        let mut current = self;
        while let Some(ref mut next_node) = *current.next {
            current = next_node;
        }
        current.next = Box::new(Some(List::new(val)));
    }

    fn pop(&mut self) -> Option<i32> {
        let current = &mut self.clone();
        loop {
            if current.next.is_none() {
                let val = current.el;
                *current = current.next.take().unwrap();
                return Some(val)
            }
            // current = (*(current.next))
            if let Some(next_node) = &mut (*current.next) {
                // ERROR WHEN CURRENT = NEXT_NODE
            }
        }
    }


    fn prepend(&mut self, val:i32) {
        // std::mem::replace(dest, src): 
        //    Moves src into the referenced dest, returning the previous dest value
        let old_self = std::mem::replace(self, List::new(val));
        self.next = Box::new(Some(old_self));
    }
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LinkedList([")?;

        let mut current = Some(self);
        while let Some(node) = current {
            write!(f, "{}", node.el)?;
            current = match &*node.next {
                Some(next_node) => Some(next_node), // Move to the next node
                None => None,                       // End of the list
            };
            if current.is_some() {
                write!(f, ", ")?;
            }

        }
        writeln!(f, "])")
    }
}

fn main() {
    let mut list = List::new(42);
    list.append(32);
    list.append(2321);
    list.append(2839);
    list.prepend(69);
    println!("{}", list);

    let mut list = List::from(&[1, 2, 3]);
    list.prepend(0);
    println!("{}", list);
}

// unsafe {
//     let mut current = self;

//     while let Some(ref next_node) = *current.next {
//         if next_node.next.is_none() { // | ... | current | next_node | None |
//             let pop_val = next_node.el;
//             current.next = Box::new(None);
//             return Some(pop_val)
//         }
//         current = &mut next_node;
//     }
// }
// return None