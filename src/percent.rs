use lazy_static::lazy_static;
use rust_decimal::Decimal;

lazy_static! {
    static ref ONE_HUNDRED: Decimal = Decimal::from(100);
}

pub fn percent_change(prior: Decimal, current: Decimal) -> Decimal {
    (current - prior) / prior * *ONE_HUNDRED
}
