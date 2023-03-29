
use super::super::hand::HandValueType;


const DEFAULT_IS_ROLLOVER: bool = false;
const DEFAULT_ROLLOVER_AMOUNT: HandValueType = 0;


#[derive(Copy, Clone)]
pub struct Rollover {
    pub is_rollover: bool,
    pub rollover_amount: HandValueType,
}
impl Rollover {
    pub fn new() -> Self {
        Self {
            is_rollover: DEFAULT_IS_ROLLOVER,
            rollover_amount: DEFAULT_ROLLOVER_AMOUNT
        }
    }
}
