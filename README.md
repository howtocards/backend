# HowToCards


## REST API

### Authentication

You should create token.<br/>
Send token in `Authorization` header.<br/>

Example:
```
Authorization: bearer 0dsi9fjs9dfj89s8djf
```

Where `0dsi9fjs9dfj89s8djf` is your token.

If route requires authentication, and request not provides it:

`400 Bad Request` is returns.

```json
{
  "ok": false,
  "error": "{ERROR_KIND}"
}
```

Where `ERROR_KIND` is:
- `invalid_token`
- `unknown_token`
- `missing_header`

### `POST /account`

Register new account.

**Receives**:

```json
{
  "email": "string",
  "password": "string"
}
```

If account registered successfully returns `200 OK`.<br/>
Otherwise returns `400 Bad Request`. <br/>


### `POST /account/session`

Login with credentials.

**Receieve**:

```json
{
  "email": "string",
  "password": "string"
}
```


**Response**:

```json
{
  "token": "string"
}
```

**Errors**:

`400 Bad Request`

```json
{
  "ok": false,
  "error": "{ERROR_KIND}"
}
```

`{ERROR_KIND}` can be:
- `email_not_found`
- `invalid_password`


### `GET /account/session`

Get info about current session.

**authentication required**

**Response**:

```json
{
  "email": "string"
}
```
