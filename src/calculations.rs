use ta::indicators::{ExponentialMovingAverage, RelativeStrengthIndex};
use ta::Next;
// calculate ris
pub fn calculate_rsi() {
    let mut rsi = RelativeStrengthIndex::new(3).unwrap();

    println!("{}", rsi.next(10.0));
}
