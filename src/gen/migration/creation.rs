use crate::style_print::log_success;
use sea_orm_cli::run_migrate_generate;
use std::path::Path;

pub(super) fn create_migration(model: &str, migration_src_dir: &Path) {
    // FIXME(@JonasCir) blocked on https://github.com/SeaQL/sea-orm/issues/1047
    //  this also should somehow use the data time from the outside
    run_migrate_generate(migration_src_dir.as_os_str().to_str().unwrap(), model).unwrap();
    log_success(&format!(
        "Successfully created migration file for model `{}`: {}",
        model,
        migration_src_dir.display()
    ));
}
