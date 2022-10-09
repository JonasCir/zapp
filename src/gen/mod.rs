use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fs, io};
use syn::File;

mod entity;
mod graphql;

pub mod migration;

pub(self) fn read_dir<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    Ok(fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_file() {
                Some(entry.file_name().to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect())
}

pub(self) fn to_upper_camel(s: &str) -> String {
    s.to_case(Case::UpperCamel)
}

pub(self) fn emit_generated_code(
    mutation_dir: &Path,
    file_name: &str,
    file_content_tokens: &TokenStream,
) -> PathBuf {
    // formatting and pretty printing
    let syntax_tree = syn::parse_str::<File>(&file_content_tokens.to_string()).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);

    let file_path = mutation_dir.join(file_name);
    dbg!(&file_path);
    let mut file = fs::File::create(&file_path).unwrap();

    file.write_all(formatted.as_bytes()).unwrap();
    file_path
}
