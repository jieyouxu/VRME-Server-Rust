# Meeting Session, Accounts, Avatars and Authentication

The core abstraction in our VRME project is a `MeetingSession`. Each
`MeetingSession` represents a live meeting duration with participants occuring
at a certain period of time.

We need to design how to handle the creation, maintainenace and graceful
shutdown of `MeetingSession`s and how `MeetingSession` interacts with the
identities of users.

The Oculus Go target hardware platform has very restricted input capabilities.
Namely, it's very difficult to submit input (such as in the form of username and
password). But we still do need to identify and confirm the identities of
meeting participants because this information is critical for handling
participants' representations in the virtual meeting environment.

## User Account, Authentication and Avatars

To minimize the amount of input that the user needs to provide through the
Oculus Go controller (which has very limited input capabilities), we can utilize
an external Account Management website.

- Users wll be able to *register*, *manage* and *delete* their accounts.
    + This will also support the notion of user *avatars*, which is a required
      functionality.

    ```rust
    type Email = String;
    type Uuid = String;
    type HashedPassword = String;
    type Avatar = Vec<u8>;

    struct RegisterUserRequest {
        first_name: String,
        last_name: String,
        password: HashedPassword,
        email: Email,
        avatar: Avatar,
    }

    struct RegisterUserResponse {
        success: bool,
        uuid: Uuid,
    }
    
    struct LoginRequest {
        username: String,
        password: HashedPassword,
    }
    ```

    + By default, `email` address serve as the username.
    + We should also generate a cross-cutting `uuid` for each user.
    + Some routes needed:
        * `POST /account/create`
            - Required payload: `struct RegisterUserRequest`
        * `POST /login`
            - Required payload: `struct LoginRequest`
            - On successful login a `session_token: String` is returned which is
              used as the `Authorization: Bearer <session_token>` header on
              requesting information from protected endpoints.
            - A `session_token` has an expiration date, after which the user is
              required to login again. If a `session_token` is used more
              frequently, it should automatically extend the expiration date
              (which should be capped at a `MAX_VALID_DURATION`).
            - Additionally, the user's `uuid` is also returned.
        * `GET /account/uuid/{email}`:
            - Gets the `uuid` of the user with the `{email}` address.
            - `{email}: String`: user's email address.
            - *Protected*: needs `session_token`.
        * `GET /account/{uuid}`
            - `{uuid}: String`
            - *Protected*: needs `session_token`.
            - Returns user information, including resource endpoint for
              user's `avatar`.
        * `GET /account/{uuid}/avatar`
            - Returns user's avatar in the `application/octet-stream` format.
        * `POST /account/{uuid}/avatar`
            - Upload a new avatar in `application/octet-stream` format.
            - *Protected*: needs `session_token`.
        * `POST /account/{uuid}/delete`
            - Deletes the account and the associated avatar.
            - *Protected*: needs `session_token`.

## Meeting Session and Presentation

Meeting session logic requires user authentication.

These operators should be done on an external website, so that we can avoid
Oculus Go from needing to supply input via the controller.

```rust
struct CreateMeetingRequest {
    creator: Uuid,
    participants: Vec<Uuid>, 
}

type MeetingId = u32;
type Uuid = String;
type PresentationId = String;

struct CreateMeetingResponse {
    success: bool,
    meeting_id: MeetingId,
    participants: Vec<Uuid>,
}

struct GetMeetingInfoResponse {
    presenter: Uuid,
    participants: Uuid,
}
```

- Endpoints:
    + `POST /meeting/new` (website)
        * Create a new meeting session.
        * Required payload: `struct CreateMeetingRequest`.
        * *Protected*: requires `session_token`.
        * On success returns `meeting_id`.
    + `POST /meeting/join` (join via website, should auto connect in Oculus Go)
        * Join a meeting session with the given `meeting_id`.
    + `GET /meeting/{meeting_id}`
        * Get information about the meeting with id `{meeting_id}`.
        * *Protected*: requires `session_token`.
    + `GET /meeting/{meeting_id}/presentation`
        * Get the presentation slides for `{meeting_id}`.
        * *Protected*: requires `session_token`.
        * Returns the presentation via `application/octet-stream`.
    + `POST /meeting/{meeting_id}/presentation`
        * Upload new presentation slides for `{meeting_id}`.
        * *Protected*: requires `session_token`.
        * Upload the presentation via `application/octet-stream`.
    + `POST /meeting/{meeting_id}/leave` (leave via website, should auto
      disconnect in Oculus Go)
        * Leave the meeting with given `{meeting_id}`.
        * Must already be a participant of the particular meeting.
    + `POST /meeting/{meeting_id}/delete` (delete via website, should auto
      disconnect in Oculus Go)
        * Terminates a meeting session.
        * *Protected*: requires `session_token`.
        * Only the meeting's creator can terminate a meeting session.

