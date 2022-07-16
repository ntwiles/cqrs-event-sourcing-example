use serde::Serialize;

pub trait EventData: Copy + Serialize {}
