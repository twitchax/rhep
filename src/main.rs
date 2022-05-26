use std::{str::FromStr, io::{Read, Write}, fs::OpenOptions, path::{PathBuf, Path}};
use aho_corasick::{AhoCorasickBuilder, MatchKind};
use anyhow::Context;
use rayon::prelude::*;
use clap::Parser;
use glob::glob;
use simple_logger::SimpleLogger;
use log::{info, warn, error, LevelFilter};

type Res<T> = anyhow::Result<T>;
type Err = anyhow::Error;
type Void = Res<()>;

/// A tool to easily find / replace text for config preparation purposes..
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The [glob](https://github.com/rust-lang-nursery/glob) of the files you want to interact with.
    #[clap(short, long)]
    glob: String,

    /// The substring to find which indicates the start of a "sentinel" (The starting token for a replacement).
    #[clap(short, long, default_value = "{{")]
    start_sentinel: String,

    /// The substring to find which indicates the start of a "sentinel" (The starting token for a replacement).
    #[clap(short, long, default_value = "}}")]
    end_sentinel: String,

    /// The set of replacements to make.
    #[clap(short, long)]
    replacements: Vec<Replacement>,

    /// The directory to write the results to.
    #[clap(short, long)]
    destination: PathBuf,

    /// Suppress log output.
    #[clap(short, long)]
    quiet: bool
}

#[derive(Debug)]
struct Replacement {
    key: String,
    value: String,
}

impl FromStr for Replacement {
    type Err = Err;

    fn from_str(s: &str) -> Res<Self> {
        let mut parts = s.splitn(2, '=');
        let key = parts.next().ok_or_else(|| Err::msg("There was no `key` part to this replacement."))?.to_string();
        let value = parts.next().ok_or_else(|| Err::msg("There was no `value` part to this replacement."))?.to_string();
        Ok(Replacement { key, value })
    }
}

fn main() -> Void {
    let args = Args::parse();

    let log_level = if args.quiet { LevelFilter::Warn } else { LevelFilter::Info };
    SimpleLogger::new().with_level(log_level).with_colors(true).without_timestamps().init().unwrap();

    let patterns = args.replacements.iter().map(|r| format!("{}{}{}", args.start_sentinel, r.key, args.end_sentinel)).collect::<Vec<_>>();
    let replacements = args.replacements.into_iter().map(|r| r.value).collect::<Vec<_>>();

    glob(&args.glob)?.par_bridge().filter_map(|path_result| {
        match path_result {
            Ok(path) => {
                if path.is_file() {
                    Some(path)
                } else {
                    warn!("{}, skipping (not file) ...", path.display());
                    None
                }
            }
            Err(e) => {
                error!("{}, skipping (glob error: `{}`) ...", e.path().display(), e.error());
                None
            },
        }
    }).for_each(|path| {
        match process_file(&args.destination, &path, &patterns, &replacements) {
            Ok(new_path) => info!("{} => {} ...", path.display(), new_path.display()),
            Err(e) => error!("{}, skipping ({}) ...", path.display(), e.to_string()),
        };
    });

    info!("Done!");

    Ok(())
}

fn process_file(destination: impl AsRef<Path>, path: impl AsRef<Path>, patterns: &[String], replacements: &[String]) -> Res<PathBuf> {
    let destination = destination.as_ref();
    let path = path.as_ref();

    let mut file = std::fs::File::open(&path).context("open error")?;

    let metadata = file.metadata().context("metadata error")?;

    let size = metadata.len().try_into().context("size error")?;
    
    let mut contents = String::with_capacity(size);
    let _ = file.read_to_string(&mut contents).context("read error")?;
    drop(file);

    let ac = AhoCorasickBuilder::new()
        .match_kind(MatchKind::LeftmostFirst)
        .build(patterns);
    let result = ac.replace_all(&contents, replacements);
    
    let new_path = destination.join(path);
    let prefix = new_path.parent().context("could not find parent error")?;
    std::fs::create_dir_all(prefix).context("could not create parent directories error")?;
    let mut new_file = OpenOptions::new().create(true).write(true).truncate(true).open(&new_path).context("create error")?;

    new_file.write_all(result.as_bytes()).context("write error")?;

    new_file.flush().context("flush error")?;

    Ok(new_path)
}