# Accounts, Authentication and Authorization

Authentication and authorization for the VRME API uses JSON Web Tokens (JWT) as
standardized by [RFC 7519](https://tools.ietf.org/html/rfc7519).

## Protected API Endpoints

Protected API endpoints require that the `Authorization` header be filled in
with a valid JWT token `<jwt-token>` on requests by the client.

```http
Authorization: Bearer <jwt-token>
```

## Typical Flow

1. User registration:
	- Endpoint: `POST /register`
	- See: [`/register`](./registration.md)
2. Login:
	- Endpoint: `POST /login`
	- See: [`/login`](./login.md)

## Failed Authentication 

Attempting to authenticate with invalid credentials will return `401
Unauthorized`.

```http
HTTP/1.1 401 Unauthorized
{
	"message": "Invalid credentials"
}
```

## References

- [RFC 7519](https://tools.ietf.org/html/rfc7519)
- [Keats/jsonwebtoken](https://github.com/Keats/jsonwebtoken)
- [jwt.io](https://jwt.io/)

