table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_by -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_by -> Nullable<Varchar>,
        updated_at -> Timestamp,
        deleted_by -> Nullable<Varchar>,
        deleted_at -> Nullable<Timestamp>,
    }
}
