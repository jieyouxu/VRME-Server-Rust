# Password Security

## Client-side

When users create an account, they need to supply a password `password` on the
client side.

The client shall hash the `password` using a *strong* hash such as
[SHA-256](https://tools.ietf.org/html/rfc4634) so we do not transfer the
plaintext password over the internet where the destination may be compromised.

We require that the password have a maximum size of \\( 128 \\) characters.

The output of the strong hash function is `client_hash`, which is
\\( 32 \\) bytes long.

We require that the `client_hash` be
[Base64](https://tools.ietf.org/html/rfc4648) encoded to give
`base64_encoded_client_hash`.

- Since each Base64 character can encode \\( 6 \\) bits

	\\[
		\log_2(64) = 6
	\\]

- \\( 32 \\) bytes or \\( 256 \\) bits can be represented by \\( 43 \\) base64
  characters.

	\\[
		\left\lceil \frac{4 (32)}{ 3 } \right\rceil = 43\ \text{base64 characters} 
	\\]

- This needs to be round up to the nearest multiple of \\( 4 \\) where the
  unused characters will be padded via `'='`. Hence a length of \\( 44 \\) is
  required.

The client-side must **not** log or store the password which is entered by the
user in plaintext form. The client also should not store the password hash.

## Server-side

We require that the client-side send a Base64-encoded
`base64_encoded_client_hash` which is required to be exactly \\( 44 \\) Base64
characters long.

When the server receives the password hashed by the client
`base64_encoded_client_hash`, we will use
[PBKDF2](https://tools.ietf.org/html/rfc2898) and
[HMAC-SHA-256](https://www.ietf.org/rfc/rfc2104.txt) to hash the
`base64_encoded_client_hash` for secure storage with securely-generated `salt`.

### Implementation Detail

- We use a **secure random number generator** to generate a \\( 16 \\) byte
  `salt`.
- We feed the `salt` and `password_hashed_1` into **PBKDF2**.
- We initialize **PBKDF2** with **HMAC-SHA-256** as the core hash function.
- We perform `100,000` iterations (`iteration_count = 100_000`).
- We take `32` bytes (`256` bits) of the output of **PBKDF2** as the final
  `password_hash_final`.
- We store the `iteration_count`, `salt` and `password_hash_final` into a
  persistent local database.

## References

- [rand](https://docs.rs/rand/0.7.3/rand/)
- [RFC 4648: Base64 encoding](https://tools.ietf.org/html/rfc4648)
- [RFC 4634: SHA and HMAC-SHA](https://tools.ietf.org/html/rfc4634)
- [RFC 2898: PKCS #5](https://tools.ietf.org/html/rfc2898)
- [Serious Security: How to store your users’ passwords safely](https://nakedsecurity.sophos.com/2013/11/20/serious-security-how-to-store-your-users-passwords-safely/)
