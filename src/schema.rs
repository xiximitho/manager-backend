// @generated automatically by Diesel CLI.

diesel::table! {
    access_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 500]
        token -> Varchar,
        expiry_date -> Timestamp,
    }
}

diesel::table! {
    attendance_records (id) {
        id -> Int4,
        patient_id -> Int4,
        attendant_id -> Int4,
        attendance_date -> Date,
        canceled -> Nullable<Bool>,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    patients (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        date_of_birth -> Date,
        #[max_length = 20]
        contact_number -> Varchar,
        #[max_length = 100]
        email -> Nullable<Varchar>,
        #[max_length = 200]
        address -> Nullable<Varchar>,
        attendant_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        unique_id -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(access_tokens -> users (user_id));
diesel::joinable!(attendance_records -> patients (patient_id));
diesel::joinable!(attendance_records -> users (attendant_id));
diesel::joinable!(patients -> users (attendant_id));

diesel::allow_tables_to_appear_in_same_query!(access_tokens, attendance_records, patients, users,);
