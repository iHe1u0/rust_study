pub struct List {
    pub data: Option<i32>,
    pub next: Option<Box<List>>,
}

impl List {
    pub fn new() -> List {
        let list = List {
            data: None,
            next: None,
        };
        list
    }

    pub fn push(self: &mut List, value: i32) {
        let node = List {
            data: Some(value),
            next: self.next.take(),
        };
        self.next = Some(Box::new(node));
    }

    pub fn pop(self: &mut List) -> Option<i32> {
        let node = self.next.take()?;
        self.next = node.next;
        Some(node.data?)
    }

    pub fn debug(&self) {
        let mut node = self;

        while let Some(ref next_node) = node.next {
            if !node.data.is_none() {
                println!("{}", node.data.unwrap_or(0));
            }

            node = next_node;
        }
        if !node.data.is_none() {
            println!("{}", node.data.unwrap_or(0));
        }
    }
}
