# User Registration

Users need to create an account to use the VRME API.


## Create an Account by Registering

```
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
	email: Email,
	hashed_password: HashedPassword,
}
```

Remarks:

- `Email` must be a valid [RFC 2822](https://tools.ietf.org/html/rfc2822)
  email address.
- See requirements for `HashedPassword` at
  [Remark: Password Security](./password.md).

### Responses 

#### Success: `201 Created`

An account with the provided information is successfully created.

```http
HTTP/1.1 201 Created
```

#### Failure: `400 Bad Request`

If the request lacks information, `400 Bad Request` is returned with:

```rust
struct MissingRequiredFieldsResponse {
	message: String,
}
```

```http
HTTP/1.1 400 Bad Request
Content-Type: application/json

{
	"message": "missing required fields"
}
```

If the request contains malformed JSON, `400 Bad Request` is returned with:

```rust
struct MalformedJsonResponse {
	message: String,
}
```

```http
HTTP/1.1 400 Bad Request
Content-Type: application/json

{
	"message": "malformed JSON"
}
```

#### Failure: `409 Conflict`

```rust
struct AccountExistsReponse {
	message: String,
}
```

```http
HTTP/1.1 409 Conflict
Content-Type: application/json

{
	"message": "an account with the provided email address already exists"
}
```

This is returned when there is an existing account with the supplied `email`.
The client is recommended to prompt the user to login instead.

