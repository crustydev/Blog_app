table! {
    article (id) {
        id -> Int4,
        title -> Varchar,
        description -> Varchar,
        content -> Varchar,
        created_at -> Varchar,
        updated_at -> Varchar,
        unique_id -> Varchar,
        blogger_id -> Int4,
    }
}

table! {
    blogger (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

table! {
    comment (id) {
        id -> Int4,
        content -> Varchar,
        created_at -> Varchar,
        updated_at -> Varchar,
        unique_id -> Varchar,
        blogger_id -> Int4,
        article_id -> Int4,
    }
}

table! {
    reply (id) {
        id -> Int4,
        content -> Varchar,
        created_at -> Varchar,
        updated_at -> Varchar,
        blogger_id -> Int4,
        comment_id -> Int4,
    }
}

joinable!(comment -> article (article_id));
joinable!(reply -> comment (comment_id));

allow_tables_to_appear_in_same_query!(
    article,
    blogger,
    comment,
    reply,
);
