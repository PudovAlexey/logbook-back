// @generated automatically by Diesel CLI.

diesel::table! {
    avatar (id) {
        id -> Int4,
        image_id -> Int4,
        user_id -> Uuid,
    }
}

diesel::table! {
    dive_site (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Varchar>,
        latitude -> Numeric,
        longitude -> Numeric,
        is_verified -> Bool,
        depth_from -> Float4,
        depth_to -> Float4,
        level -> Int4,
        image_id -> Int4,
    }
}

diesel::table! {
    image (id) {
        id -> Int4,
        path -> Text,
        #[max_length = 100]
        filename -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    log_image (id) {
        id -> Int4,
        image_id -> Int4,
        logbook_id -> Int4,
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
        image_id -> Nullable<Int4>,
        site_id -> Int4,
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
diesel::joinable!(dive_site -> image (image_id));
diesel::joinable!(log_image -> image (image_id));
diesel::joinable!(loginfo -> dive_site (site_id));
diesel::joinable!(loginfo -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    avatar,
    dive_site,
    image,
    log_image,
    loginfo,
    users,
);
