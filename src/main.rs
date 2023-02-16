use anyhow::{anyhow, Result};
use clap::Parser;
use glob::glob;
use regex::{Captures, Regex};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
use tempfile::tempdir;

#[derive(Parser, Debug)]
struct Args {
    note: PathBuf,
    #[arg(short, long, env = "OBSIDIAN_VAULT")]
    vault: PathBuf,
    #[arg(short, long, default_value = "note.html")]
    output: PathBuf,
    #[arg(short, long, default_value_t = false)]
    stdout: bool,
}

fn replace_links(vault_path: &'_ Path) -> impl FnMut(&Captures) -> String + '_ {
    |captures| {
        let target = captures.name("target").map_or("", |m| m.as_str());

        let path = glob(&format!("{}/**/{}", vault_path.to_string_lossy(), target,))
            .unwrap()
            .next()
            .map(|path| path.unwrap().to_str().unwrap().to_owned());

        let text = captures.name("text").map_or(target, |m| m.as_str());

        if let Some(p) = path {
            format!("[{}]({})", text, p)
        } else {
            text.into()
        }
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
    // Check out the funky deref - reref stuff going on here!
    let vault_path = PathBuf::from(&*shellexpand::full(&*args.vault.to_string_lossy())?);
    let note_path = vault_path.join(args.note);
    let new_note = modify_note(note_path, vault_path)?;
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
        let mut pandoc = Command::new("pandoc")
            .stdin(Stdio::piped())
            .arg("-f")
            .arg("markdown+autolink_bare_uris+tex_math_dollars")
            .arg("-o")
            .arg(args.output.to_str().unwrap())
            .arg("--standalone")
            .arg("--embed-resources")
            .arg("--data-dir")
            .arg(temp_dir.path())
            .arg("--css")
            .arg(css_path.to_str().unwrap())
            .arg("--template")
            .arg(template_path.to_str().unwrap())
            .spawn()?;

        pandoc
            .stdin
            .as_mut()
            .unwrap()
            .write_all(new_note.as_bytes())?;
        let output = pandoc.wait_with_output()?;

        if !output.status.success() {
            return Err(anyhow!("Command executed with failing error code"));
        }
    }
    Ok(())
}
