use anyhow::{Result, anyhow};
use clap::Parser;
use glob::glob;
use regex::{Captures, Regex};
use tempfile::tempdir;
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio}, io::Write,
};

#[derive(Parser, Debug)]
struct Args {
    note: PathBuf,
    vault: PathBuf,
    #[arg(short, long, default_value = "note.html")]
    output: PathBuf,
    #[arg(short, long, default_value_t = false)]
    stdout: bool,
}

fn replace_links(vault_path: &'_ Path) -> impl FnMut(&Captures) -> String + '_ {
    |captures| {
        let text = captures.name("text").map_or("", |m| m.as_str());
        let target = captures.name("target").map_or("", |m| m.as_str());
        let path = match glob(&format!("{}/**/{}", vault_path.to_string_lossy(), target))
            .unwrap()
            .next()
        {
            Some(path) => path.unwrap().to_str().unwrap().to_owned(),
            None => String::from(""),
        };
        format!("[{}]({})", text, path)
    }
}

#[allow(unused)]
fn modify_note(note_path: PathBuf, vault_path: PathBuf) -> Result<String> {
    let note = fs::read_to_string(note_path)?;
    let re = Regex::new(r"\[\[(?P<target>[^\|\]]+)\|?(?P<text>[^\|\]]+)?\]\]")?;
    Ok(re
        .replace_all(&note, replace_links(&vault_path))
        .into_owned())
}

fn main() -> Result<()> {
    let args = Args::parse();
    let new_note = modify_note(args.note, args.vault)?;
    let temp_dir = tempdir()?;

    let template = include_bytes!("../resources/template.html");
    let template_path = temp_dir.path().join("template.html");
    // fs::create_dir(temp_dir.path().join("templates"))?;
    fs::File::create(&template_path)?.write_all(template)?;

    let css = include_bytes!("../resources/output.css");
    let css_path = temp_dir.path().join("output.css");
    fs::File::create(&css_path)?.write_all(css)?;
    
    if args.stdout {
        println!("{}", new_note);
    } else {
        let mut pandoc = Command::new("pandoc").stdin(Stdio::piped())
            .arg("-f")
            .arg("markdown")
            .arg("-o")
            .arg(args.output.to_str().unwrap())
            .arg("--standalone")
            .arg("--embed-resources")
            .arg("--data-dir")
            .arg(temp_dir.path())
            .arg("--css")
            .arg(css_path.to_str().unwrap())
            .arg("--template")
            .arg(template_path.to_str().unwrap()).spawn()?;

        pandoc.stdin.as_mut().unwrap().write_all(new_note.as_bytes())?;
        let output = pandoc.wait_with_output()?;

        if !output.status.success() {
            return Err(anyhow!("Command executed with failing error code"));
        }
    }
    Ok(())
}
