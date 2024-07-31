use reedline::{DefaultCompleter, FileBackedHistory, Reedline, Signal};

fn main() {
    // Initialize the REPL with history
    let history = Box::new(
        FileBackedHistory::with_file(50, "history.txt".into())
            .expect("Error configuring history with file"),
    );

    let commands = vec![
        "train".to_string(),
        "predict".to_string(),
        "exit".to_string(),
    ];
    let completer = Box::new(DefaultCompleter::new(commands.clone()));

    let mut line_editor = Reedline::create()
        .with_history(history)
        .with_completer(completer);

    println!("Welcome to the ML REPL. Type 'exit' to quit.");

    loop {
        let sig = line_editor.read_line(">>> ");
        match sig {
            Ok(Signal::Success(buffer)) => {
                handle_command(buffer);
            }
            Ok(Signal::CtrlC) | Ok(Signal::CtrlD) => {
                println!("\nExiting...");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }
}

fn handle_command(command: String) {
    match command.as_str() {
        "train" => {
            println!("Training model...");
            // Call your train function here
        }
        "predict" => {
            println!("Predicting...");
            // Call your predict function here
        }
        "exit" => {
            std::process::exit(0);
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}
