// @generated automatically by Diesel CLI.

diesel::table! {
    admin_users (user_id) {
        user_id -> Integer,
        name -> Text,
        url -> Text,
        front_deploy -> Text,
        email -> Text,
        password -> Text,
        phone -> Nullable<Text>,
        dni -> Nullable<Text>,
        status -> Text,
        role -> Text,
        created_at -> Text,
        updated_at -> Nullable<Text>,
    }
}

diesel::table! {
    links (id) {
        id -> Integer,
        link -> Text,
        title -> Text,
        date_created -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    admin_users,
    links,
);
