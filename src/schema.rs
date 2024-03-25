// @generated automatically by Diesel CLI.

diesel::table! {
    avatar (id) {
        id -> Int4,
        image_id -> Int4,
        user_id -> Uuid,
    }
}

diesel::table! {
    image (id) {
        id -> Int4,
        path -> Text,
        #[max_length = 100]
        filename -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    loginfo (id) {
        id -> Int4,
        title -> Varchar,
        depth -> Float4,
        start_datetime -> Timestamp,
        end_datetime -> Timestamp,
        water_temperature -> Nullable<Float4>,
        vawe_power -> Nullable<Float4>,
        side_view -> Nullable<Float4>,
        start_pressure -> Int4,
        end_pressure -> Int4,
        description -> Nullable<Varchar>,
        user_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 50]
        surname -> Nullable<Varchar>,
        #[max_length = 50]
        patronymic -> Nullable<Varchar>,
        #[max_length = 10]
        role -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        date_of_birth -> Timestamp,
        #[max_length = 100]
        password -> Varchar,
        is_verified -> Bool,
        id -> Uuid,
        avatar_id -> Nullable<Int4>,
    }
}

diesel::joinable!(avatar -> image (image_id));
diesel::joinable!(loginfo -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    avatar,
    image,
    loginfo,
    users,
);
