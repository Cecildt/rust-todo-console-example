use std::io;

struct Todo {
    id: i16,
    title: String,
    completed: bool,
    deleted: bool,
}

fn add_todo(todos: &mut Vec<Todo>, title: &str) {
    // The size of the vector + 1 makes a decent enough id
    let new_id = todos.len() as i16 + 1;
    todos.push(Todo {
        id: new_id,
        title: title.to_string(),
        completed: false,
        deleted: false,
    });
}

fn remove_todo(todos: &mut Vec<Todo>, todo_id: i16) {
    if let Some(todo) = todos.iter_mut()
                            .find(|todo| todo.id == todo_id) {
        todo.deleted = true;
    }
}

fn mark_done(todos: &mut Vec<Todo>, todo_id: i16) {
    if let Some(todo) = todos.iter_mut()
                            .find(|todo| todo.id == todo_id) {
        todo.completed = true;
    }
}

fn print_todos(todos: &Vec<Todo>) {
    println!("\n\nTodo List:\n — — — — — — — — — -");
    for todo in todos {
        if !todo.deleted {
            let done = if todo.completed { "✔" } else { " " };
            println!("[{}] {} {}", done, todo.id, todo.title);
        }
    }
}

fn invalid_command(command: &str) {
    println!("Invalid command: {}", command);
}

fn main() {
    let mut todos: Vec<Todo> = Vec::new();
    // Print the Todo list on start up
    print_todos(&todos);

    // Loop over input lines forever and ever
    loop {
        // Assign input lines to the `command` variable
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("failed to read line");

        // Split up the input string by spaces (or any whitespace)
        let command_parts: Vec<&str> = command.split_whitespace().collect();

// Now match the size of the vector holding the separate 
        // words in the command
        match command_parts.len() {
            // If 0 we can’t really do much
            0 => invalid_command(&command),
            // If the length is 1 it’ a `list` command, 
            // or an invalid command
            1 => match command_parts[0] {
                "list" => print_todos(&todos),
                // Remember `_`is catch all
                _ => invalid_command(&command),
            },
            // If the length is bigger than 1 we look for 
            // `add x x x x`, `remove x` or `done x`
            _ => {
                // Match the first word in the command
                match command_parts[0] {
                    // If add, let’s join up all words except the
                    // first one as our todo title
                    // `[1..]` means from index 1 in the vector 
                    // to the end
                    "add" => add_todo(&mut todos,
                        &command_parts[1..].join(" ")),
                    // For remove and done we want to send in a
                    // todo_id, so we parse the string as an integer
                    // parse returns a `Result` that is either 
                    // `Ok` or `Err`, so we can handle it much as 
                    // an `Option` return
                    "remove" => if let Ok(num) = command_parts[1].parse::<i16>() {
                        remove_todo(&mut todos, num)
                    },
                    "done" => if let Ok(num) = command_parts[1].parse::<i16>() {
                        mark_done(&mut todos, num)
                    },
                    _ => invalid_command(&command),
                }
            },
        }

        // At the end of each loop print the list
        print_todos(&todos);
    }
}
