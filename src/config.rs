use anyhow::Result;
use clap::{command, value_parser, Arg, ArgAction};
use git2::Repository;
use std::{env, fmt, net::Ipv4Addr, path::PathBuf};
use walkdir::{DirEntry, WalkDir};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    pub bind: Ipv4Addr,
    pub port: u16,
    pub path: PathBuf,
    pub repo: Vec<Repo>,
    pub mode: Mode,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Repo {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug)]
pub enum Mode {
    Update,
    Server,
}

#[allow(dead_code)]
impl Config {
    pub fn new() -> Result<Self> {
        let matches = command!()
            .arg(
                Arg::new("bind")
                    .long("bind")
                    .value_name("BIND")
                    .default_value("127.0.0.1")
                    .value_parser(value_parser!(Ipv4Addr))
                    .help("bind to interface"),
            )
            .arg(
                Arg::new("port")
                    .long("port")
                    .value_name("PORT")
                    .default_value("8080")
                    .value_parser(value_parser!(u16).range(1..))
                    .help("port to listen on"),
            )
            .arg(
                Arg::new("update")
                    .long("update")
                    .conflicts_with_all(["bind", "port"])
                    .action(ArgAction::SetTrue)
                    .help("update git repositories to rogit.db"),
            )
            .arg(
                &Arg::new("path")
                    .value_name("ROGIT-PATH")
                    .required(false)
                    .value_parser(value_parser!(PathBuf)),
            )
            .get_matches();
        let current_dir = env::current_dir().unwrap();
        let path = matches.get_one::<PathBuf>("path").unwrap_or(&current_dir);

        let mode = if matches.get_flag("update") {
            Mode::Update
        } else {
            git2::opts::enable_caching(false);
            Mode::Server
        };
        git2::opts::strict_hash_verification(false);
        unsafe {
            git2::opts::set_mwindow_file_limit(64)?;
        }
        let bind = matches.get_one::<Ipv4Addr>("bind").unwrap();
        let port = matches.get_one::<u16>("port").unwrap();

        let repo: Vec<Repo> = Vec::new();

        Ok(Self {
            bind: bind.clone(),
            port: port.clone(),
            path: path.clone(),
            repo,
            mode,
        })
    }

    pub fn scan(&mut self) {
        let mut repos: Vec<Repo> = Vec::new();
        for entry in WalkDir::new(&self.path)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_entry(|e| Self::is_repo(e))
            .filter_map(|r| r.inspect_err(|e| eprintln!("walkdir error: {}", e)).ok())
        {
            let link = entry.into_path();
            let name = link.file_name().unwrap().to_string_lossy().to_string();
            let path = std::fs::read_link(&link).unwrap();
            println!("[scan] sync: {}\t-> {}", name, path.display());
            repos.push(Repo { name, path });
        }
        if repos.is_empty() {
            println!("[scan] no git repository found!");
        } else {
            self.repo = repos
        };
    }

    fn is_repo(entry: &DirEntry) -> bool {
        if !entry.path_is_symlink() {
            return false;
        }
        // TODO skip hidden file
        Repository::open_bare(entry.path())
            .inspect_err(|e| eprintln!("[scan] skip: {}", e.message()))
            .is_ok()
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "path: {}", self.path.display())?;
        for r in &self.repo {
            writeln!(f, "* {}\t->{}", r.name, r.path.display())?;
        }
        match self.mode {
            Mode::Update => write!(f, "mode: update"),
            Mode::Server => write!(f, "mode: server {}:{}", self.bind, self.port),
        }
    }
}
