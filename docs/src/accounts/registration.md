# User Registration

Users need to create an account to use the VRME API.

## Create an Account by Registering

```http
POST /register
```

### Description

Creates a new user account, if one does not exist already.

### URL Structure

```
/register
```

### HTTP Verb

```http
POST
```

### Request Headers

Required:

- `Content-Type` must be `application/json`.
- `Content-Length` must be specified.

### Request Body

```rust
struct RegistrationRequest {
	first_name: String,
	last_name: String,
	email: String,
	hashed_password: String,
}
```

Remarks:

- `Email` must be a valid [RFC 2822](https://tools.ietf.org/html/rfc2822)
  email address.
- See requirements for `HashedPassword` at
  [Remark: Password Security](./password.md).
	* Must be hashed client-side with a **strong** hash function such as
	  `SHA-256`.
	* Must be hashed to exactly \\( 32 \\) bytes long.
	* Must be Base64-encoded to \\( 44 \\) Base64 characters long.

### Responses

#### Success: `201 Created`

An account with the provided information is successfully created.

```http
HTTP/1.1 201 Created
Content-Type: application/json

{
  "email": "no-reply@example.com",
  "uuid": "123e4567-e89b-12d3-a456-426655440000",
  "auth_token": "BASE64_ENCODED_AUTH_TOKEN"
}
```

#### Failure: `400 Bad Request`

If the request lacks information, `400 Bad Request` is returned with:

```rust
struct MissingRequiredFieldsResponse {
	cause: String,
	message: String,
}
```

```http
HTTP/1.1 400 Bad Request
Content-Type: application/json

{
	"cause": "missing-fields",
	"message": "missing required fields"
}
```

If the request contains malformed JSON, `400 Bad Request` is returned with:

```rust
struct MalformedJsonResponse {
	cause: String,
	message: String,
}
```

```http
HTTP/1.1 400 Bad Request
Content-Type: application/json

{
	"cause": "malformed-request",
	"message": "malformed JSON"
}
```

#### Failure: `409 Conflict`

```rust
struct AccountExistsReponse {
	cause: String,
	message: String,
}
```

```http
HTTP/1.1 409 Conflict
Content-Type: application/json

{
	"cause": "account-exists",
	"message": "an account with the provided email already exists, login instead"
}
```

This is returned when there is an existing account with the supplied `email`.
The client is recommended to prompt the user to login instead.