use mysql::*;

pub fn database_connection() -> Pool {
    let url: &str = "mysql://root:password@localhost/rust_dev_api_crud";
    Pool::new(url).expect("Failed to create pool")
}
