# Accounts, Authentication and Authorization

Authentication and authorization for the VRME API uses `auth_token`s issued
by the server.

## Protected API Endpoints

Protected API endpoints require that the `Authorization` header be filled in
with a valid authentication payload sent by the user:

```json
{"email":"no-reply@example.com", "auth-token":"base64-encoded"}
```

```http
POST /logout HTTP/1.1
Authorization: Bearer {"email":"no-reply@example.com", "auth-token":"base64-encoded"}
```

## Typical Flow

1. User registration:
	- Endpoint: `POST /register`
	- See: [`/register`](./registration.md)
2. Login:
	- Endpoint: `POST /login`
	- See: [`/login`](./login.md)
	- On successful login, the client is granted an `auth-token`.
3. Access protected endpoints:
  - Access protected endpoint: e.g. `POST /logout`
  - Requires the user to send the authentication payload in the HTTP
    `Authorization: Bearer <AUTH_PAYLOAD>`:
  
  ```json
  {"email":"no-reply@example.com", "auth-token":"base64-encoded"}
  ```

## Failed Authentication 

Attempting to authenticate with invalid credentials will return `401
Unauthorized`.

```http
HTTP/1.1 401 Unauthorized
{
	"message": "invalid credentials"
}
```
