//! An user account.

/// Newtype declarartion on `chrono::Date<chrono::Utc>`.
pub type Date = chrono::Date<chrono::Utc>;

/// An user account.
///
/// - `Email` must be a valid [RFC 2822](https://tools.ietf.org/html/rfc2822)
///   email address.
/// - `password_hash` requirements:
/// 	* Must be hashed client-side with a **strong** hash function such as
/// 	  `SHA-256`.
/// 	* Must be hashed to exactly \\( 32 \\) bytes long.
/// 	* Must be Base64-encoded to \\( 43 \\) Base64 characters long.
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
