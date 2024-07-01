use crate::compiled_protos::counter::CounterState;

impl From<i32> for CounterState {
    fn from(value: i32) -> Self {
        Self { counter: value }
    }
}
