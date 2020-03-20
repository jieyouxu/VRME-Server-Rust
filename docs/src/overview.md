# Overview

**API Version**: `1.0.0`

The is the documentation for the Virtual Reality Meeting Environment server, 
which is responsible for hosting API endpoints for three subsystems:

1. Account Management and Authentication
2. Meeting Session and Presentation
3. View State Relay Subsystem

## Path Parameters

When an endpoint has `{parameter_name}` within the URL path, then this
information needs to be provided by the client.

### Example

```http
GET /account/{uuid}
```

**Path parameters**:

| Parameter Name | Type     | Description            | Additional Constraints | Example            |
|----------------|----------|------------------------|------------------------|--------------------|
| `uuid`         | `String` | The `uuid` of an user. |                        | `aaaabbbbccccdddd` |

When the request is made, path parameters must be properly filled in.

```http
GET /account/aaaabbbbccccdddd
```

## Request Payloads 

Some `POST` and `PUT` endpoints require that the client provide additional data
through the *request body*.

All of such endpoints require that the additional data be provided in the JSON
data interchange format as standardized by
[RFC 7159](https://tools.ietf.org/html/rfc7159).

When sending JSON in the request body, the client **must** provide these
headers:

- `Content-Length`: the size of the JSON payload in bytes. 
- `Content-Type`: `application/json` shall be provided as the MIME type of the
  JSON payload.

The endpoints' documentation will specify the required structure and type of the
JSON object in Rust `struct` format.

### Example

```rust
type Email = String;
type Date = String;
type HashedPassword = String;

struct RegisterRequest {
	first_name: String,
	last_name: String,
	date_of_birth: Date,
	email: Email,
	hashed_password: HashedPassword,
}
```

The `Email`, `Date` and `HashedPassword` are type-aliased here because they have
additional constraints as to which values are allowed to occupy their types, and
often is a subset of the aliased type `String`.

- Any value which occupies the `Email` type is required to conform with the
  email address format standardized in
  [RFC 5322](https://tools.ietf.org/html/rfc5322).
- Any value which occupies the `Date` type is required to conform with the
  [RFC 3339](https://tools.ietf.org/html/rfc3339) date format.
- Any value which occupies the `HashedPassword` type should be a *hashed*
  password and not a plain-text password.

A possible value that complies with the required structure and type:

```json
{
	"first_name": "John",
	"last_name": "Doe",
	"email": "no-reply@example.com",
	"hashed_password": "&^D1cxa8d7a89cxhuiyu"
}
```

## Pending Work

- Incoporate the audio subsystem.

## References

- [RFC 7159](https://tools.ietf.org/html/rfc7159)
- [RFC 5322](https://tools.ietf.org/html/rfc5322)
- [RFc 3339](https://tools.ietf.org/html/rfc3339)

