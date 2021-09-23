use std::io::{Read, Write, Seek, SeekFrom};

use json::JsonValue;
use structopt::StructOpt;

use crate::utils::Result;

mod path;

use path::Path;

#[derive(StructOpt)] pub struct Cd { pub key: String }
#[derive(StructOpt)] pub struct CatExcept { pub except: Vec<String> }
#[derive(StructOpt)] pub struct Edit { pub editor: String }
#[derive(StructOpt)] pub struct Save { pub filename: String }

#[derive(StructOpt)]
pub enum Command
{
    Cat,
    CatExcept(CatExcept),
    Cd(Cd),
    Discard,
    Edit(Edit),
    Exit,
    Ls,
    Save(Save)
}

pub fn shell(mut json: JsonValue) -> Result
{
    let mut buf = String::new();
    let mut cwd = Path::new();
    let mut unsaved = false;

    ctrlc::set_handler(move || {
        eprintln!();
        print!("? ");
        std::io::stdout().flush().unwrap();
    }).unwrap_or(());

    loop {
        buf.clear();
        print!("{}> ", itertools::join(cwd.clone(), "/"));
        std::io::stdout().flush()?;
        buf += "petfall ";
        std::io::stdin().read_line(&mut buf)?;

        let command = match Command::from_iter_safe(buf.strip_suffix("\n").unwrap_or("petfall").split(' ')) {
            Ok(cmd) => cmd,
            Err(_) => { eprintln!("petfall: unrecognized command"); continue },
        };

        let cwjson = path::reach(&json, &cwd);

        match command {
            Command::Cat => println!("{}", json::stringify_pretty(cwjson.clone(), 4)),
            Command::CatExcept(cat) =>
                if let Err(err) = path::cat_except(&cwjson, &cat.except) {
                    eprintln!("Error: {}", err);
                }
            Command::Cd(cd) => {
                if let Err(err) = path::advance(&cwjson, &mut cwd, &cd.key) {
                    eprintln!("Error: {}", err);
                }
            },
            Command::Discard => unsaved = false,
            Command::Edit(edit) => {
                let mut file = tempfile::NamedTempFile::new()?;
                writeln!(file, "{}", json::stringify_pretty(cwjson.clone(), 4))?;
                if let Err(err) = std::process::Command::new(edit.editor).arg(file.path()).status() {
                    eprintln!("An error occured! {}", err);
                }
                file.seek(SeekFrom::Start(0))?;
                buf.clear();
                file.read_to_string(&mut buf)?;
                match json::parse(&buf) {
                    Ok(newval) => { path::edit(&mut json, &cwd, newval).unwrap(); unsaved = true },
                    Err(_) => eprintln!("Failed to parse resulting JSON!")
                }
            }
            Command::Exit => {
                if unsaved {
                    eprintln!("There are unsaved changes, first use the save or discard command");
                }
                else {
                    break;
                }
            },
            Command::Ls => {
                println!("{}", itertools::join(path::ls(&cwjson), " "));
            },
            Command::Save(save) => {
                match std::fs::File::create(save.filename) {
                    Ok(mut file) => { writeln!(file, "{}", json::stringify_pretty(json.clone(), 4))?; unsaved = false },
                    Err(_) => eprintln!("Failed to create new json file!")
                }
            }
        }
    }

    Ok(())
}
