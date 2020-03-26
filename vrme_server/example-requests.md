# Example Requests

Using `httpie` as the tool:

## Register an Account

- Base password: `123456`
- Apply `HMAC-SHA-256` hash algorithm: `8d969eef6ecad3c29a3a629280e686cf0c3f5d5a86aff3ca12020c923adc6c92`
- Apply `base64` encoding: `sO31+7iOCyh+r76az9YIwqaxoOzty+fUTNHNWJA0w+I=`

```bash
http -v POST http://localhost:8080/register \ 
	first_name="John" \
	last_name="Doe" \
	email="example@example.com" \
	hashed_password="sO31+7iOCyh+r76az9YIwqaxoOzty+fUTNHNWJA0w+I="
```

## Login

- Use same `hashed_password`: `sO31+7iOCyh+r76az9YIwqaxoOzty+fUTNHNWJA0w+I=`
- Use same `email`: `example@example.com`

```bash
http -v POST http://localhost:8080/login \
	email="example@example.com" \
	hashed_password="sO31+7iOCyh+r76az9YIwqaxoOzty+fUTNHNWJA0w+I="
```

## Get UUID with Email

- Use same `email`: `example@example.com`

```bash
http -v GET http://localhost:8080/accounts/uuid \
	email="example@example.com"
```
