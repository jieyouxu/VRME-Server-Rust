# Get UUID of User Given Email

Available at `GET /user/email`:

```http
GET /user/email HTTP/1.1
Content-Type: application/json

{
  "email": "no-reply@example.com"
}
```

## Successful Response

```http
HTTP/1.1 200 OK
Content-Type: application/json

{
  "uuid": "123e4567-e89b-12d3-a456-426655440000"
}
```
