use super::object::Object;

pub struct Food<O: Object> {
    body: O,
    health: u32,
}
