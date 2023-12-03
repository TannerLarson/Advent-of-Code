use camino::Utf8PathBuf;
use nom::{
    self,
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

/// Path ////////////////////////////////////////////////////////////////////////////////////////////////
fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        // Keep taking while the character is a lower-case letter or a '.' or a '/'
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(i)
}

/// Commands ////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
struct Ls;

fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

#[derive(Debug)]
struct Cd(Utf8PathBuf);

fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}

#[derive(Debug)]
pub enum Command {
    Ls,
    Cd(Utf8PathBuf),
}

impl From<Ls> for Command {
    fn from(_value: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(value: Cd) -> Self {
        Command::Cd(value.0)
    }
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((map(parse_cd, Into::into), map(parse_ls, Into::into)))(i)
}

/// Entry ////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    // parser for file
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, path)| Entry::File(size, path),
    );
    // parser for directory
    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);
    alt((parse_file, parse_dir))(i)
}

/// Line ////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub enum Line {
    Entry(Entry),
    Command(Command),
}

pub fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_entry, Line::Entry),
        map(parse_command, Line::Command),
    ))(i)
}
