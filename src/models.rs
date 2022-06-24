#[derive(Queryable)]
pub struct Mqtt {
    pub id: i32,
    pub topic: String,
    pub payload: String,
    pub time: String,
    pub qos: i32,
}