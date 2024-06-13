mod checklist;
use checklist::Checklist;

fn main() {
    let mut checklist = Box::new(Checklist::new());
    
    loop {
        println!("---------------------------");
        println!("1 - add todo");
        println!("2 - list todos");
        println!("3 - mark todo complete");
        println!("4 - quit");
        
        let mut input = String::new();
        
        std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

        match input.trim() {
            "1" => {
                println!("enter a name for todo");
                
                let mut name_input = String::new();
                
                std::io::stdin()
                .read_line(&mut name_input)
                .expect("Failed to read input");
                
                checklist.add(name_input);
            },
            "2" => checklist.print_list(),
            "3" => {
                println!("enter a name to mark complete");
                
                let mut id_input: String = String::new();
                
                std::io::stdin()
                .read_line(&mut id_input)
                .expect("Failed to read input");

                let id = id_input.trim().parse::<u32>().expect("Invalid ID");

                checklist.mark_todo_complete(id);
            }
            "4" => break,
            _ => println!("Failed to read input")
        }
    }
}