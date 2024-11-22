use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "sdl")]
#[command(about = "A CLI for the SDL programming language", long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Init { dir: Option<String> },
    Build,
    Run { file: Option<String> },
    Lex { file: Option<String> },
    Parse { file: Option<String> },
    Tree { file: Option<String> },
}

fn main() {
    let cli = CLI::parse();

    match cli.command {
        Commands::Init { dir } => init_command(dir),
        Commands::Build => build_command(),
        Commands::Run { file } => run_command(file),
        Commands::Lex { file } => lex_command(file),
        Commands::Parse { file } => parse_command(file),
        Commands::Tree { file } => tree_command(file),
    }
}

fn init_command(dir: Option<String>) {
    println!("Initializing project in: {:?}", dir.unwrap_or_else(|| ".".to_string()));
}

fn build_command() {
    println!("Building SDL project...");
}

fn run_command(file: Option<String>) {
    println!("Running file: {:?}", file.unwrap_or_else(|| "main.sdl".to_string()));
}

fn lex_command(file: Option<String>) {
    println!("Lexing file: {:?}", file);
}

fn parse_command(file: Option<String>) {
    println!("Parsing file: {:?}", file);
}

fn tree_command(file: Option<String>) {
    println!("Generating AST tree for file: {:?}", file);
}