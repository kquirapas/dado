use clap::{Parser, Subcommand};

mod dado;
// use dado::add;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        #[clap(value_parser)]
        task: String
    },
    Remove {
        #[clap(value_parser)]
        task_num: usize
    },
    Toggle {
        #[clap(value_parser)]
        task_num: usize
    },
    List
}

// TODO: Resolve paths properly
// TODO: Properly eliminate the out of nowhere \n in .dado when list goes to 0
// TODO: Fix case when removing to 0 tasks
const SAVEFILE: &str = "/home/kirby/.dado";

fn main() {
    let cli = Args::parse();

    match &cli.command {
        Commands::Add { task } => { 
            match dado::add(SAVEFILE, task.as_str()) {
                Err(why) => {
                    panic!("Failed to add to {}: {}", SAVEFILE, why);
                },
                Ok(_) => println!("Successfully added")
            };
        },
        Commands::Remove { task_num } => { 
            match dado::remove(SAVEFILE, &task_num) {
                Err(why) => {
                    panic!("Failed to remove Task#{} from {}: {}", task_num, SAVEFILE, why);
                },
                Ok(_) => println!("Successfully removed")
            };
        },
        Commands::Toggle { task_num } => { 
            match dado::toggle(SAVEFILE, &task_num) {
                Err(why) => {
                    panic!("Failed to toggle Task#{} from {}: {}", task_num, SAVEFILE, why);
                },
                Ok(_) => println!("Successfully toggled")
            };
        },
        Commands::List => { 
            if let Err(why) = dado::list(SAVEFILE) {
                panic!("Failed to list from {}: {}", SAVEFILE, why);
            };
        },
    };
}
