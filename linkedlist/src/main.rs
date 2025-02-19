#[derive(Debug)]
struct LinkedList {
    head: Option<Box<Cell>>
}
#[derive(Debug)]
struct Cell {
    data: i32,
    next: Option<Box<Cell>>
}

impl LinkedList {
    fn new_list() -> Self {
        Self {
            head: None
        }
    }
    fn insert_head(&mut self, d: i32) {
        let head_cell = Box::new(Cell {
            data: d,
            next: self.head.take(),
        });
        self.head = Some(head_cell);
    }
    fn delete_top(&mut self) {
        if let Some(top) = self.head.take() {
            self.head = top.next;
        }
    }

    fn search_cell(&mut self, d: i32) -> Option<&mut Cell> {
        let mut current = self.head.as_deref_mut();
        while let Some(cell) = current {
            if cell.data == d {
                return Some(cell);
            }
            current = cell.next.as_deref_mut();
        }
        None
    }
}

impl Cell {
    fn insert(&mut self, d: i32) {
        let new_cell = Box::new(Cell {
            data: d,
            next: self.next.take()
        });
        self.next = Some(new_cell);
    }

    fn delete(&mut self) {
        if let Some(delete_cell) = self.next.take() {
            self.next = delete_cell.next;
        }
    }

}


fn main() {
    let mut list = LinkedList::new_list();

    list.insert_head(1);
    list.insert_head(2);
    list.insert_head(3);
    println!("{:?}", list);

    if let Some(cell) = list.search_cell(2) {
        cell.insert(9);
    }
    println!("{:?}", list);

    if let Some(cell) = list.search_cell(2) {
        cell.delete();
    }
    println!("{:?}", list);
    
}
