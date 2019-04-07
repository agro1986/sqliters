use std::process;
use std::io;
use std::io::Write;
use std::num::ParseIntError;

#[derive(Debug, Clone, PartialEq, Eq)]
struct MetaCommandError {
    kind: MetaCommandErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum MetaCommandErrorKind {
    UnrecognizedCommand,
}

impl MetaCommandError {
    pub fn new(kind: MetaCommandErrorKind) -> Self {
        Self { kind }
    }

    pub fn kind(&self) -> &MetaCommandErrorKind {
        &self.kind
    }

    pub fn __description(&self) -> &str {
        match self.kind {
            MetaCommandErrorKind::UnrecognizedCommand => "Unrecognized command",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PrepareStatementError {
    kind: PrepareStatementErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PrepareStatementErrorKind {
    UnrecognizedStatement,
}

impl PrepareStatementError {
    pub fn new(kind: PrepareStatementErrorKind) -> Self {
        Self { kind }
    }

    pub fn kind(&self) -> &PrepareStatementErrorKind {
        &self.kind
    }

    pub fn __description(&self) -> &str {
        match self.kind {
            PrepareStatementErrorKind::UnrecognizedStatement => "Unrecognized keyword",
        }
    }
}

struct Statement {
    kind: StatementKind,
}

enum StatementKind {
    Insert,
    Select,
}

impl Statement {
    pub fn insert() -> Self {
        Self { kind: StatementKind::Insert }
    }

    pub fn select() -> Self {
        Self { kind: StatementKind::Select }
    }

    pub fn kind(&self) -> &StatementKind {
        &self.kind
    }
}


fn prepare_statement(input_buffer: &str) -> Result<Statement, PrepareStatementError> {
    if input_buffer.starts_with("insert") {
        return Ok(Statement::insert());
    }

    if input_buffer == "select" {
        return Ok(Statement::select());
    }

    Err(PrepareStatementError{kind: PrepareStatementErrorKind::UnrecognizedStatement})
}

fn execute_statement(s: &Statement) {
    match s.kind() {
        StatementKind::Select => {
            println!("This is where we would do a select");
        }
        StatementKind::Insert => {
            println!("This is where we would do an insert");
        }
    }
}

fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap();
}

fn do_meta_command(input_buffer: &str) -> Result<(), MetaCommandError> {
    if input_buffer == ".exit" {
        process::exit(libc::EXIT_SUCCESS);
    }

    Err(MetaCommandError::new(MetaCommandErrorKind::UnrecognizedCommand))
}

fn main() {
    loop {
        print_prompt();
        let mut input_buffer = String::new();
        io::stdin().read_line(&mut input_buffer).unwrap();
        let input_buffer = input_buffer.trim();

        if input_buffer.starts_with(".") {
            match do_meta_command(input_buffer) {
                Ok(_) => {
                    continue;
                }
                Err(error) => {
                    let message = error.__description();
                    println!("{}: {}", message, input_buffer);
                    continue;
                }
            }
        }

        let statement = match prepare_statement(input_buffer) {
            Ok(s) => s,
            Err(error) => {
                let message = error.__description();
                println!("{}: {}", message, input_buffer);
                continue;
            }
        };

        execute_statement(&statement);

        println!("Executed.");
    }

}
