// @generated automatically by Diesel CLI.

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
    }
}
