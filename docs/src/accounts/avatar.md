# User Avatar

Upon account creation, the user receives a default avatar.

The user will need to update the avatar through the endpoint
`POST /user/{uuid}/avatar`.

Size defaults to `512x512` in `PNG` format.

## Get Avatar

`GET /user/{uuid}/avatar`

```http
GET /user/{uuid}/avatar
```

### Successful Response

```http
HTTP/1.1 200 OK
Content-Type: image/png

89 50 4E 47 0D 0A 1A 0A
```

## Update Avatar

Protected endpoint: `PUT /user/{uuid}/avatar`

```http
PUT /user/{uuid}/avatar HTTP/1.1
Content-Type: image/png
Authorization: Bearer eyJ1dWlkIjoiMTIzZTQ1NjctZTg5Yi0xMmQzLWE0NTYtNDI2NjU1NDQwMDAwIiwgImF1dGgtdG9rZW4iOiJiYXNlNjQtZW5jb2RlZCJ9

89 50 4E 47 0D 0A 1A 0A
```

### Successful Response

```http
HTTP/1.1 200 OK
```

## Delete Avatar

Resets user avatar to default avatar.

Protected endpoint: `DELETE /user/{uuid}/avatar`

```http
DELETE /user/{uuid}/avatar HTTP/1.1
Authorization: Bearer eyJ1dWlkIjoiMTIzZTQ1NjctZTg5Yi0xMmQzLWE0NTYtNDI2NjU1NDQwMDAwIiwgImF1dGgtdG9rZW4iOiJiYXNlNjQtZW5jb2RlZCJ9
```

### Successful Response

```http
HTTP/1.1 200 OK
```