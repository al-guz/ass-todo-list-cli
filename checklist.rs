struct ChecklistItem {
    complete: bool,
    id: u32,
    name: String
}

pub struct Checklist {
    todo_list: Vec<ChecklistItem>,
}

impl Checklist {
    pub fn print_list(&self) {
        if self.todo_list.is_empty() {return}
        println!("-------------------------------------");
        for item in self.todo_list.iter() {
            println!("----------------");
            println!("id: {} name: {} isCompleted: {}", item.id, item.name, item.complete);
        }
    }

    pub fn mark_todo_complete(&mut self, id:u32) -> Option<()> {
        for items in &mut self.todo_list {
            if items.id == id {
                println!("marked {} complete", items.name);
                items.complete = true;
                Some(());
            }
        }
        None
    }
    
    pub fn add(&mut self, name: String) {
        println!("added {}", name);
        let checklist_item = ChecklistItem {
            complete: false,
            id: self.todo_list.len() as u32 + 1,
            name: name
        };
        self.todo_list.push(checklist_item);
    }

    pub fn new() -> Self {
        Self {
            todo_list: vec![]
        }
    }
}