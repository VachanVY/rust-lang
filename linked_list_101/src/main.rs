#![allow(warnings)]

#[derive(Clone, Debug)]
struct List {
    el: Option<i32>,
    next: Box<Option<List>>
}

impl List {
    fn new(val: i32) -> List {
        return List {
            el: Some(val),
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
        if self.el.is_none() {
            self.el = Some(val);
            return;
        }
        let mut current = self;
        while let Some(ref mut next_node) = *current.next {
            current = next_node;
        }
        current.next = Box::new(Some(List::new(val)));
    }

    fn pop(&mut self) -> Option<i32> {
        match &mut*self.next {
            None => {
                let pop_val = self.el;
                self.el = None;
                return pop_val;
            },

            // we check the next element
            Some(n) => {
                if n.next.is_none() {
                    let pop_val = n.el;
                    // if the next element is the terminal element
                    // we take the self.next
                    // that will change the current.next to None
                    self.next.take();
                    return pop_val;
                }else{
                    // if the next element is not the terminal element
                    return n.pop()
                }
            }
        }
    }

    fn prepend(&mut self, val:i32) {
        if self.el.is_none() {
            self.el = Some(val);
            return;
        }
        // std::mem::replace(dest, src): 
        //    Moves src into the referenced dest, returning the previous dest value
        let old_self = std::mem::replace(self, List::new(val));
        self.next = Box::new(Some(old_self));
    }

    fn popf(&mut self) -> Option<i32> {
        let pop_val = self.el; // Store the value to return
        if let Some(next_node) = self.next.take() {
            // Update the current node with the values of the next node
            self.el = next_node.el;
            self.next = next_node.next;
        } else {
            // If there's no next node, this is the last node
            self.el = None;
        }
        return pop_val;
    }
    
    
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LinkedList([")?;

        let mut current = Some(self);
        while let Some(node) = current {
            if !node.el.is_none(){
                write!(f, "{}", node.el.unwrap())?;
            }
                current = match &*node.next {
                    Some(next_node) => Some(next_node), // Move to the next node
                    None => None,                       // End of the list
                };
                if current.is_some() {
                    write!(f, ", ")?;
                }

        }
        write!(f, "])")
    }
}

fn main() {
    let mut list = List::new(42); 
    println!("{}", list); // [42]
    println!("POP: {:?}", list.pop()); // 42
    println!("{}", list); // []
    println!("{:?}", list.pop()); // poping from empty linked list // does nothing
    println!("{}", list); // []
    list.prepend(420);
    println!("{}", list); // [420]
    println!("POP: {:?}", list.pop()); // 420
    println!("{}", list); // []
    println!("POPF: {:?}", list.popf());
    println!("{}", list); // []
    

    list.append(32);
    list.append(2321);
    list.append(2839);
    list.prepend(69);
    println!("{}", list); // [69, 32, 2321, 2839]
    println!("POPF: {:?}", list.popf()); // 69
    println!("{}", list); // [32, 2321, 2839]
    println!("POP: {:?}", list.pop()); // 2839
    println!("{}", list); // [32, 2321]

    let mut list = List::from(&[1, 2, 3]);
    println!("{}", list);
    list.prepend(0);
    println!("{}", list);
    while let Some(val) = list.pop() {
        println!("POP: {}", val);
    }

    let mut list = List::from(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    println!("{}", list);
    list.prepend(-1);
    println!("{}", list);
    while let Some(val) = list.popf() {
        println!("POPF: {}", val);
    }
}
