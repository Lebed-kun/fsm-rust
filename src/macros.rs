/// Simple macro for initialization of hashmap
#[macro_export]
macro_rules! map {
    ($($key:expr => $val:expr),* $(,)?) => {
        {
            use std::collections::HashMap;
            
            let mut map = HashMap::new();

            $(
                map.insert($key, $val);
            )*

            map
        }
    };
}
