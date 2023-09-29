use self::order_number_classifier::OrderNumberClassifier;

pub fn ans() -> u128 {
    let mut onc = OrderNumberClassifier::default();
    while onc.get_order() < 100 {
        onc.next();
    }
    onc.get_non_bouncy()
}

mod order_number_classifier;

#[cfg(test)]
mod tests;
