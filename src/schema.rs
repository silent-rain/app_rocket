table! {
    http_logs (id) {
        id -> Integer,
        user_id -> Nullable<Varchar>,
        method -> Varchar,
        path -> Varchar,
        query -> Nullable<Varchar>,
        body -> Nullable<Varchar>,
        remote_addr -> Varchar,
        log_type -> Varchar,
        created -> Datetime,
    }
}

table! {
    token_api_auth (id) {
        id -> Integer,
        user_token_id -> Integer,
        uri -> Varchar,
        expire -> Integer,
        status -> Bool,
        created -> Datetime,
        updated -> Datetime,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Varchar,
        gender -> Bool,
        age -> Integer,
        birth -> Nullable<Varchar>,
        phone -> Varchar,
        email -> Nullable<Varchar>,
        password -> Varchar,
        address -> Nullable<Varchar>,
        avatar -> Nullable<Varchar>,
        status -> Bool,
        created -> Datetime,
        updated -> Datetime,
    }
}

table! {
    user_token (id) {
        id -> Integer,
        user_id -> Nullable<Varchar>,
        token -> Varchar,
        status -> Bool,
        created -> Datetime,
        updated -> Datetime,
    }
}

joinable!(token_api_auth -> user_token (user_token_id));

allow_tables_to_appear_in_same_query!(
    http_logs,
    token_api_auth,
    users,
    user_token,
);
