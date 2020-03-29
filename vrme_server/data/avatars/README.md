# Avatars Store

This data directory stores the avatars of each user who uploaded an avatar of
their choice with their consent.

Each avatar is named by `{uuid}.png` where `uuid` is the unique id of the user.

When the client requests an avatar of a user who did not upload an avatar or
deleted their avatar, the default avatar `default.png` is returned as a
fallback.

## Default Avatar

The default avatar is taken from
`https://www.iconfinder.com/icons/403017/anonym_avatar_default_head_person_unknown_user_icon`.

The website explicitly grants permission for **free commerical use**, whereas
copyright attribution is hereby given.
