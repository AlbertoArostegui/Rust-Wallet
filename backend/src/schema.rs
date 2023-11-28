// @generated automatically by Diesel CLI.

diesel::table! {
    users (email) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        hashed_password -> Varchar,
        #[max_length = 255]
        salt -> Varchar,
        #[max_length = 255]
        public_key -> Varchar,
        #[max_length = 255]
        address -> Varchar,
    }
}
