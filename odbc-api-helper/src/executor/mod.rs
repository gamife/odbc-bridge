pub mod batch;
pub mod database;
pub mod execute;
pub mod prepare;
pub mod query;
pub mod statement;
pub mod table;

#[derive(Debug)]
pub enum SupportDatabase {
    Dameng,
    Pg,
    Mysql,
}
