//! Hashed password newtype.

use actix_web::web;
use rand;
use ring::pbkdf2;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

/// Hashed password type.
#[derive(Debug, Serialize, Deserialize)]
pub struct HashedPassword {
	/// Number of iterations of `PBKDF2`.
	pub iteration_count: u32,
	/// 16-byte `salt` generated from a secure random number generator.
	pub salt: Vec<u8>,
	/// 32-bytes of the output of `PBKDF2`.
	pub hash: Vec<u8>,
}

impl HashedPassword {
	/// Construct a new `HashedPassword`.
	///
	/// Uses the `PBKDF2` algorithm:
	///
	/// - See `PBKDF2_ALGORITHM` and `PBKDF2_ITERATIONS` for the exact algorithm and number of
	///   iterations used to compute the final hash.
	pub async fn new(
		client_hash: &[u8; HASHED_PASSWORD_LEN],
	) -> Result<Self, String> {
		use rand::RngCore;

		let client_hash = client_hash.clone();

		web::block(move || -> Result<Self, String> {
			let mut salt = [0u8; SALT_LEN];
			rand::thread_rng()
				.try_fill_bytes(&mut salt)
				.map_err(|e| e.to_string())?;

			let mut hash = [0u8; HASHED_PASSWORD_LEN];

			pbkdf2::derive(
				PBKDF2_ALGORITHM,
				NonZeroU32::new(PBKDF2_ITERATIONS as u32).unwrap(),
				&salt,
				&client_hash,
				&mut hash,
			);

			Ok(Self {
				iteration_count: PBKDF2_ITERATIONS as u32,
				salt: salt.to_vec(),
				hash: hash.to_vec(),
			})
		})
		.await
		.map_err(|e| e.to_string())
	}
}

/// Length of the extracted hashed password in bytes. This is for the raw hashed password bytes that
/// is not Base64-encoded.
pub(crate) const HASHED_PASSWORD_LEN: usize = 32;

/// Length of the randomly generated salt in bytes.
const SALT_LEN: usize = 16;

/// We use the `PBKDF2` algorithm to compute the password hash in a secure fashion, with the core
/// hash function being `HMAC-SHA-256`.
///
/// # References
///
/// - [`ring::pbkdf2`](https://briansmith.org/rustdoc/ring/pbkdf2/index.html).
static PBKDF2_ALGORITHM: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;

/// Number of `PBKDF2` iterations to perform. The more iterations, the more difficult to try to
/// compute a rainbow table to try to reverse the hash. However, more iterations also take more CPU
/// cycles to compute.
///
/// We default to use `100_000` iterations which gives a reasonably large number of iterations to
/// hinder a possible attacker.
///
/// As computing power increases, the number of iterations should also be increased to ensure the
/// difficulity (in computing time) for an potential adversay to try to compute a rainbow table for
/// the `PBKDF2` + `HMAC-SHA-256` combination.
const PBKDF2_ITERATIONS: usize = 100_000;
