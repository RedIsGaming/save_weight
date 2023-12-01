use chrono::*;

#[derive(Debug, Clone, Copy)]
pub struct Weight {
    pub id: u32,
    pub weight: f32,
    pub date: DateTime<Utc>
}

impl Weight {
    pub fn new(id: u32, weight: f32, date: DateTime<Utc>) -> Self {
        Self {
            id,
            weight,
            date
        }
    }
}
