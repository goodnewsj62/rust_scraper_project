pub use sea_orm_migration::prelude::*;

mod m20240915_044114_unique_hash;
mod m20240926_000001_create_school_table;
mod m20240915_070941_regen_column;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240915_044114_unique_hash::Migration),
            Box::new(m20240926_000001_create_school_table::Migration),
            Box::new(m20240915_070941_regen_column::Migration),
        ]
    }
}
