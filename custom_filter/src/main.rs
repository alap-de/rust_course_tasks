fn main() {
    let values = vec![1, 2, 3, 4, 5];
    let condition = FilterCondition { value: 2 };
    let result = custom_filter(values, &condition);
    println!("{:?}", result);
}

struct FilterCondition<T> {
    value: T,
}

impl<T: Eq> FilterCondition<T> {
    fn is_match(&self, value: &T) -> bool {
        self.value == *value
    }
}

fn custom_filter<T: Eq>(values: Vec<T>, condition: &FilterCondition<T>) -> Vec<T> {
    let mut result = Vec::new();
    for value in values.into_iter() {
        if condition.is_match(&value) {
            result.push(value);
        }
    }
    result
}
