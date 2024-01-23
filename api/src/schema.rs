// @generated automatically by Diesel CLI.

diesel::table! {
    admin (id) {
        id -> Int4,
        #[max_length = 25]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
    }
}

diesel::table! {
    player (id) {
        id -> Int4,
        #[max_length = 25]
        name -> Varchar,
        score -> Int4,
        #[max_length = 1000]
        image_url -> Varchar,
    }
}

diesel::table! {
    soul (id) {
        id -> Int4,
        owner -> Int4,
        #[max_length = 25]
        name -> Varchar,
    }
}

diesel::joinable!(soul -> player (owner));

diesel::allow_tables_to_appear_in_same_query!(
    admin,
    player,
    soul,
);
