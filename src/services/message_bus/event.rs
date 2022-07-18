use serde::Serialize;

pub trait EventData: Copy + Serialize {
    // Used for human readable mongodb entries.
    fn kind(&self) -> String;
}
