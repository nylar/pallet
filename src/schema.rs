table! {
    krate (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
    }
}

table! {
    krateowner (krate_id, owner_id) {
        krate_id -> Int4,
        owner_id -> Int4,
    }
}

table! {
    owner (id) {
        id -> Int4,
        login -> Text,
        name -> Nullable<Text>,
    }
}

table! {
    token (id) {
        id -> Int4,
        owner_id -> Int4,
        name -> Text,
        api_token -> Text,
        created_at -> Timestamp,
    }
}

table! {
    version (id) {
        id -> Int4,
        krate_id -> Int4,
        vers -> Text,
        yanked -> Bool,
    }
}

joinable!(krateowner -> krate (krate_id));
joinable!(krateowner -> owner (owner_id));
joinable!(token -> owner (owner_id));
joinable!(version -> krate (krate_id));

allow_tables_to_appear_in_same_query!(krate, krateowner, owner, token, version,);
