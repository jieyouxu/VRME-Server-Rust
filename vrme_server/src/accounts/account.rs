//! An user account.

use tokio_pg_mapper_derive::PostgresMapper;

/// Newtype declarartion on `chrono::NaiveDate` which does not take into account timezone
/// information.
pub type Date = chrono::NaiveDate;

/// An user account.
///
/// - `Email` must be a valid [RFC 2822](https://tools.ietf.org/html/rfc2822)
///   email address.
/// - `password_hash` requirements:
/// 	* Must be hashed client-side with a **strong** hash function such as
/// 	  `SHA-256`.
/// 	* Must be hashed to exactly **32** bytes long (take the first 32 bytes).
/// 	* Must be Base64-encoded to **43** base64 characters long.
#[derive(Debug, PostgresMapper)]
#[pg_mapper(table = "accounts")]
pub struct Account {
	pub user_id: u32,
	pub email: String,
	pub first_name: String,
	pub last_name: String,
	pub iteration_count: u32,
	pub salt: String,
	pub password_hash: String,
	pub created_at: Date,
}
