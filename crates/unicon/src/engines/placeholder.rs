use crate::traits::TableStatic;

pub trait PlaceholderFuncStdStatic: TableStatic {}

// Note: `Placeholder` is just used to go from 1 to >1 traits to test Rusts type system with generic traits.
// - Instead of implementing an actual DB just emulate what the trait type structure may be.
pub trait PlaceholderString {
    fn get_struct(&self) -> String {
        "placeholder".to_string()
    }
}


pub trait PlaceholderFuncStd: PlaceholderString {
    fn get_vals(&self) -> Vec<(String, String)> {
        Self::get_struct(self);

        vec![
            ("key1".to_string(), "val1".to_string())
        ]
    }
}