# Update Account Information

Info allowed to be updated include:

- First name: `first_name`
- Last name: `last_name`

Protected endpoint: `PUT /user/{uuid}`

```http
PUT /user/{uuid} HTTP/1.1
Content-Type: application/json

{
  "first_name": "abc",
  "last_name": "abc"
}
```

### Successful Response

```http
HTTP /1.1 200 OK
```
