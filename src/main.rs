use std::io::{self, stdout, BufRead, BufReader, Write};
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::path::{Path, PathBuf};

use clap::{AppSettings, Clap};
use console::{self, style, Key, Term};
use rand::prelude::*;

#[derive(Clap)]
#[clap(version = "1.0")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short, long)]
    path: PathBuf,
    #[clap(short, long)]
    quit_with_err: bool,
}

fn main() -> io::Result<()> {
    let opts: Opts = Opts::parse();
    let shell = get_shell_path();
    let prompt_command = get_prompt_command(&shell);
    let mut term = Term::stdout();
    term.clear_screen()?;
    let mut input = read_list(&opts.path);
    let mut buf = String::new();
    while let Ok(size) = input.read_line(&mut buf) {
        if size == 0 {
            break;
        }
        let chars = buf.trim_end_matches('\n').chars();
        let mut stdout = io::stdout();
        let mut prompt = Command::new(&shell)
            .arg("-c")
            .arg(&prompt_command)
            .output()?
            .stdout;
        if prompt.is_empty() {
            prompt = default_prompt();
        }
        term.write_all(&prompt)?;
        chars.for_each(|c| {
            let rand: f64 = rand::thread_rng().gen();
            let duration_millis = ((rand * 2.0 + 0.5) * 50.0) as u64;
            print!("{}", c);
            stdout.flush().unwrap();
            thread::sleep(Duration::from_millis(duration_millis));
        });
        term.write_line("");
        let output = Command::new("sh").arg("-c").arg(&buf).output()?;
        term.write_all(&output.stdout)?;
        buf.clear();
        if !output.status.success() && opts.quit_with_err {
            break;
        }
    }

    term.move_cursor_down(4)?;
    println!("To quit, press 'q' key.");
    let q = Key::Char('q');
    loop {
        let k = term.read_key()?;
        if k == q {
            break;
        }
    }

    Ok(())
}

fn read_list<P: AsRef<Path>>(p: P) -> BufReader<File> {
    let f = File::open(p).expect("cannot open file");
    BufReader::new(f)
}

#[allow(unused)]
fn default_prompt() -> Vec<u8> {
    Vec::from(format!("{}", style("prompt # ").cyan()))
}

#[allow(unused)]
fn show_prompt() {
    format!("{}", String::from_utf8_lossy(&default_prompt()));
}

fn get_shell_path() -> String {
    "fish".into()
}

fn get_prompt_command(_shell: &str) -> String {
    "fish_prompt".into()
}
