use diesel::sql_types::Time;

#[derive(Queryable)]
pub struct Mqtt {
    pub id: i32,
    pub topic: String,
    pub payload: String,
    pub time: Time,
    pub qos: i32,
}