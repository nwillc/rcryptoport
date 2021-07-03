use lazy_static::lazy_static;
use rust_decimal::Decimal;

lazy_static! {
    static ref ONE_HUNDRED: Decimal = Decimal::from(100);
}

pub fn percent_change(prior: Decimal, current: Decimal) -> Decimal {
    (current - prior) / prior * *ONE_HUNDRED
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    #[test]
    fn test_percent_change() {
        let change = super::percent_change(Decimal::from(200), Decimal::from(250));
        assert_eq!(Decimal::from(25), change);
    }
}
