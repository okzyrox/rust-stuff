
pub struct Task {
    pub name: String,
    pub description: String,
    pub completed: bool,
}

pub struct TaskList {
    pub tasks: Vec<Task>,
}

impl TaskList {
    pub fn new() -> TaskList {
        TaskList { tasks: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn list_tasks(&self) {
        println!("List of tasks: ");
        if self.is_empty() {
            println!("There are no tasks!");
            return;
        }
        println!("Completed:");
        for task in &self.tasks {
            if task.completed {
                task.print();
            }
            
        }

        println!("Not completed:");
        for task in &self.tasks {
            if !task.completed {
                task.print();
            }
        }
    }

    pub fn new_task(&mut self, name: String, description: String) {
        let task = Task::new(name, description);
        self.tasks.push(task);
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn complete_task(&mut self, name: String) {
        if self.is_empty() {
            println!("There are no tasks to complete!");
            return;
        }
        for task in &mut self.tasks {
            if task.name == name {
                task.complete();
            }
        }
    }

    pub fn uncomplete_task(&mut self, name: String) {
        if self.is_empty() {
            println!("There are no tasks to uncomplete!");
            return;
        }
        for task in &mut self.tasks {
            if task.name == name {
                task.uncomplete();
            }
        }
    }

    pub fn delete_task(&mut self, name: String) {
        // this looks really cool ngl
        // a function that filters and only keeps based on a specified parameter
        // this is sick
        if self.is_empty() {
            println!("There are no tasks to delete!");
            return;
        }
        self.tasks.retain(|task| task.name != name);
    }
}

impl Task {
    pub fn new(name: String, description: String) -> Task {
        Task { name, description, completed: false }
    }
    pub fn print(&self) {
        println!("<Task> name: {}, description: {}, completed: {}", self.name, self.description, self.completed);
    }

    pub fn complete(&mut self) {
        self.completed = true;
        println!("Task marked as completed: {}", self.name);
    }

    pub fn uncomplete(&mut self) {
        self.completed = false;
        println!("Task marked as uncompleted: {}", self.name);
    }
}

pub fn run () {
    let mut task_list = TaskList::new();
    let mut running = true;

    while running {
        let mut option = String::new();
        println!("Enter option (list, new, delete, complete, uncomplete, exit): ");
        std::io::stdin().read_line(&mut option).expect("Failed to read line");
        match option.trim() {
            "list" => {
                task_list.list_tasks();
                println!("")
            }
            "new" => {
                let mut input = String::new();
                println!("Enter task name: ");
                std::io::stdin().read_line(&mut input).expect("Failed to read line");
                if input.trim() == "" {
                    println!("You must enter a task name to create");
                    continue;
                }
                task_list.new_task(input.trim().to_string(), "This is a new task".to_string());
                println!("")
            }
            "complete" => {
                let mut input = String::new();
                println!("Enter task name: ");
                std::io::stdin().read_line(&mut input).expect("Failed to read line");
                if input.trim() == "" {
                    println!("You must enter a task name to complete");
                    continue;
                }
                task_list.complete_task(input.trim().to_string());
                println!("")
            }
            "uncomplete" => {
                let mut input = String::new();
                println!("Enter task name: ");
                std::io::stdin().read_line(&mut input).expect("Failed to read line");
                if input.trim() == "" {
                    println!("You must enter a task name to uncomplete");
                    continue;
                }
                task_list.uncomplete_task(input.trim().to_string());
                println!("")
            }
            "delete" => {
                let mut input = String::new();
                println!("Enter task name: ");
                std::io::stdin().read_line(&mut input).expect("Failed to read line");
                if input.trim() == "" {
                    println!("You must enter a task name to delete");
                    continue;
                }
                task_list.delete_task(input.trim().to_string());
                println!("")
            }
            "exit" => {
                running = false;
            }
            _ => println!("Invalid option")
        }
    }
}
