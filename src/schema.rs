table! {
    user (id) {
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
        created -> Varchar,
        updated -> Varchar,
    }
}
