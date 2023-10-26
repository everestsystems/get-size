use serde_json::{Map, Number, Value};

use crate::{GetSize, GetSizeTracker};

impl GetSize for Value {
    fn get_heap_size<T: GetSizeTracker>(&self, tracker: &mut T) -> usize {
        match self {
            Value::Null | Value::Bool(_) | Value::Number(_) => 0,
            Value::String(string) => string.get_heap_size(tracker),
            Value::Array(vec) => vec.get_heap_size(tracker),
            Value::Object(map) => map.get_heap_size(tracker),
        }
    }
}

impl GetSize for Map<String, Value> {
    fn get_heap_size<T: GetSizeTracker>(&self, tracker: &mut T) -> usize {
        self.iter()
            .map(|(key, value)| key.get_heap_size(tracker) + value.get_heap_size(tracker))
            .sum::<usize>()
    }
}

impl GetSize for Number {}

#[cfg(test)]
mod test_serde_json_types {
    use std::mem;

    use serde_json::json;

    use super::*;

    use crate::NoTracker;

    #[test]
    fn test_serde_json_number() {
        let test = json!(1);
        assert_eq!(test.get_heap_size(&mut NoTracker), 0);

        let test = json!(1.0);
        assert_eq!(test.get_heap_size(&mut NoTracker), 0);
    }

    #[test]
    fn test_serde_json_string() {
        let test = json!("Hello");
        assert_eq!(test.get_heap_size(&mut NoTracker), 5);
    }

    #[test]
    fn test_serde_json_array() {
        let test = json!(["Hello", "World"]);
        assert_eq!(
            test.get_heap_size(&mut NoTracker),
            mem::size_of::<Value>() * 2 + 10
        );
    }

    #[test]
    fn test_serde_json_map() {
        let mut test = Map::new();
        assert_eq!(test.get_heap_size(&mut NoTracker), 0);

        test.insert("a".to_string(), Value::Number(Number::from(1i8)));
        assert_eq!(test.get_heap_size(&mut NoTracker), 1);

        test.insert("b".to_string(), Value::Number(Number::from(2i8)));
        test.insert("c".to_string(), Value::Number(Number::from(3i8)));
        assert_eq!(test.get_heap_size(&mut NoTracker), 3);
    }
}
