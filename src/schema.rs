// Generated-like schema using Diesel's macro (matches your table).
// If you prefer infer_schema via migrations, keep this aligned.

diesel::table! {
    authentication (username) {
        username -> Text,
        created -> Timestamptz,
        privilege -> Text,
        password -> Text,
    }
}

