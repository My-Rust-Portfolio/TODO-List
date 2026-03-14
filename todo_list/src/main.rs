use clap::{Parser, Subcommand};

#[derive(Debug)]
struct Task {
    title: String,
    description: String,
    completed: bool,
    index: usize,
}

#[derive(Debug, Subcommand)]
enum Commands {
    List,
    Add { title: String, description: String },
    Complete { index: usize },
}

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)] // Tells clap to parse the subcommand into this field
    command: Commands,
}

fn main() {
    let args = Args::parse();  // Fills from CLI args
    println!("Parsed: {:?}", args.command);

    let mut tasks: Vec<Task> = Vec::new();

    match args.command {
        Commands::Add { title, description } => {
            let task = Task { title, description, completed: false, index: tasks.len() };
            tasks.push(task);
            println!("Added: {:?}", tasks.last().unwrap());
        }
        Commands::List => {
            for task in &tasks {
                println!("{:?}", task);
            }
        }
        Commands::Complete { index} => {
            if (index >= tasks.len()) {
                eprintln!("Error: No task with index {}", index);
                return;
            }

            tasks.iter_mut().find(|t| t.index == index).map(|t| t.completed = true);
            println!("Completed task with index: {}", index);
        }
    }
}


