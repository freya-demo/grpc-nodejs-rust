use std::ops::Bound;

use crate::compiled_protos;

impl From<compiled_protos::base::Range> for (Bound<usize>, Bound<usize>) {
    fn from(value: compiled_protos::base::Range) -> Self {
        (
            Bound::Included(value.start as usize),
            match value.end {
                Some(v) => Bound::Excluded(v as usize),
                None => Bound::Unbounded,
            },
        )
    }
}
