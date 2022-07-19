table! {
    req_rsp_logs (id) {
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

allow_tables_to_appear_in_same_query!(
    req_rsp_logs,
    users,
);
