table! {
    accounts (uuid) {
        uuid -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        iteration_count -> Int4,
        salt -> Bpchar,
        password_hash -> Bpchar,
    }
}
