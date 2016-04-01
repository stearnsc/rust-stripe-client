use super::money::Money;

struct Balance {
    available: Vec<Money>,
    livemode: bool,
    pending: Vec<Money>
}
