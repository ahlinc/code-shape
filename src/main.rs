use anyhow::{anyhow, Result};
use cli::{Arguments, Cli};
use std::{
    fs,
    path::{Path, PathBuf},
};
use tree_sitter::{Language, Parser, Query, QueryCursor, Tree};
use tree_sitter_config::Config;
use tree_sitter_loader::{self, LanguageConfiguration, Loader};

fn main() -> Result<()> {
    let cli = Cli::new();
    print_file_items(&cli.args)?;
    Ok(())
}

mod cli {
    use super::*;
    use clap::{Args, Parser};

    #[derive(Parser)]
    #[command(author, version, about, long_about = None)]
    pub struct Cli {
        #[command(flatten)]
        pub args: Arguments,
    }

    #[derive(Args)]
    pub struct Arguments {
        #[arg(long)]
        pub scope: Option<String>,
        pub file_path: PathBuf,
    }

    impl Cli {
        pub fn new() -> Self {
            Self::parse()
        }
    }
}

fn print_file_items(args: &Arguments) -> Result<()> {
    let (language, name) = if let Some(ref scope) = args.scope {
        load_language_for_scope(scope.as_str())?
    } else {
        load_language_for_path(args.file_path.as_path())?
    };
    let query = load_language_query(name, language)?;
    let (tree, text) = parse_file(language, args.file_path.as_path())?;
    println!("{:#?}", tree);

    let mut cursor = QueryCursor::new();
    for (m, capture_index) in cursor.captures(&query, tree.root_node(), text.as_slice()) {
        let pattern_index = m.pattern_index;
        let capture = m.captures[capture_index];
        let capture_name = &query.capture_names()[capture_index as usize];
        let capture_text = capture.node.utf8_text(&text).unwrap_or("");

        println!(
            "<Query match id: {m_id
            }, index: {pattern_index}:{capture_index:<2
            }, capture: {capture_name:20
            }, text: \"{capture_text
            }\">",
            m_id = m.id(),
        );
    }

    Ok(())
}

type LanguageName = String;

fn load_language_for_path(path: &Path) -> Result<(Language, LanguageName)> {
    let config = Config::load()?;
    let mut loader = Loader::new()?;
    loader.find_all_languages(&config.get()?)?;
    let Some((language, config)) = loader.language_configuration_for_file_name(path)? else {
        return Err(anyhow!("Language for path wasn't found"));
    };
    let name = language_name_from_config(config)?;
    Ok((language, name))
}

fn load_language_for_scope(scope: &str) -> Result<(Language, LanguageName)> {
    let config = Config::load()?;
    let mut loader = Loader::new()?;
    loader.find_all_languages(&config.get()?)?;
    let Some((language, config)) = loader.language_configuration_for_scope(scope)? else {
        return Err(anyhow!("Language for path wasn't found"));
    };
    let name = language_name_from_config(config)?;
    Ok((language, name))
}

fn language_name_from_config(lanuguage_config: &LanguageConfiguration) -> Result<String> {
    // println!(
    //     "LC: {:?}, {:?}",
    //     lanuguage_config.scope, lanuguage_config.root_path
    // );
    lanuguage_config
        .scope
        .clone()
        .map(|scope| {
            // println!("scope: {scope}");
            scope
                .strip_prefix("source.")
                // .map_or(scope.clone(), |s| s.to_owned())
                .map(|s| s.to_owned())
                .unwrap_or(scope)
        })
        .or_else(|| {
            lanuguage_config
                .root_path
                .file_stem()
                .map(|s| s.to_str().map(|s| s.to_owned()))
                .flatten()
        })
        .ok_or(anyhow!("Language for path wasn't found"))
}

fn parse_file(language: Language, path: &Path) -> Result<(Tree, Vec<u8>)> {
    let mut parser = Parser::new();
    parser.set_language(unsafe { std::mem::transmute(language) })?;
    let text = fs::read(path)?;
    let tree = parser.parse(&text, None).expect("Can't parse the file");
    Ok((tree, text))
}

fn xdg_config_dir() -> Result<PathBuf> {
    let xdg_path = dirs::config_dir()
        .ok_or(anyhow!("Cannot determine config directory"))?
        .join(env!("CARGO_PKG_NAME"));
    Ok(xdg_path)
}

fn language_queries_dir() -> Result<PathBuf> {
    Ok(xdg_config_dir()?.join("languages"))
}

fn load_language_query(name: LanguageName, language: Language) -> Result<Query> {
    let dir = language_queries_dir()?;
    let mut query_path = dir.join(name);
    query_path.set_extension("scm");
    Ok(Query::new(language, &fs::read_to_string(query_path)?)?)
}
