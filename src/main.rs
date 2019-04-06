use std::process;
use std::io;
use std::io::Write;

fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap();
}

fn main() {
    loop {
        print_prompt();
        let mut input_buffer = String::new();
        io::stdin().read_line(&mut input_buffer).unwrap();
        let input_buffer = input_buffer.trim();
        if input_buffer == ".exit" {
            process::exit(libc::EXIT_SUCCESS);
        } else {
            println!("Unrecognized command '{}'.", input_buffer);


        }

    }

}
