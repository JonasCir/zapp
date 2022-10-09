use crate::gen::migration::{creation::create_migration, registration::register_migration};
use std::path::Path;

mod creation;
mod registration;

pub fn handle_gen_migration(model: &str, gen_path: &Path) {
    let migration_src_dir = gen_path.join("migration").join("src");

    create_migration(model, &migration_src_dir);
    register_migration(model, &migration_src_dir);
}
