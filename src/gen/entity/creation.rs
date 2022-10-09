use crate::gen::emit_generated_code;
use crate::style_print::log_success;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use sea_orm_cli::{run_migrate_command, MigrateSubcommands, run_generate_command, GenerateSubcommands, DateTimeCrate};
use std::path::Path;

pub(super) fn create_entity(model: &str, entity_src_dir: &Path) {
    let migrate_subcommand = MigrateSubcommands::Fresh;
    run_migrate_command(
        Some(migrate_subcommand),
        migration_dir.to_str().unwrap(),
        false,
    )
    .unwrap();

    let generate_subcommand = GenerateSubcommands::Entity {
        compact_format: true,
        expanded_format: false,
        include_hidden_tables: false,
        tables: None,
        ignore_tables: vec!["seaql_migrations".to_string()],
        max_connections: 1,
        output_dir: entity_src_dir.to_string(),
        database_schema: "".to_string(),
        database_url: "".to_string(),
        with_serde: "".to_string(),
        with_copy_enums: false,
        date_time_crate: DateTimeCrate::Time
    };
 run_generate_command(
        Some(generate_subcommand),
        false
    );
    log_success(&format!(
        "Successfully created `{}` entity file: {}",
        model,
        file_path.display()
    ));
}
