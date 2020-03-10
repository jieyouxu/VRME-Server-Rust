pub(crate) mod register;

use diesel::Queryable;

/// An user account.
#[derive(Debug, Queryable)]
pub struct Account {
    pub uuid: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub iteration_count: u32,
    pub salt: String,
    pub password_hash: String,
}
