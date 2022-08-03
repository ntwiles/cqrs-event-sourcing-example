pub trait Replay<T, U> {
    fn replay(events: Vec<T>) -> U;
}
