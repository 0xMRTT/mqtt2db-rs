table! {
    mqtt (id) {
        id -> Int4,
        topic -> Varchar,
        payload -> Text,
        time -> Time,
        qos -> Int4,
    }
}
