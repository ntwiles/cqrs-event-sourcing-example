use serde::Serialize;

pub trait EventData: Copy + Serialize {
    fn kind(&self) -> String;
}
