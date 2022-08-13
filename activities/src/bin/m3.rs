// Topic: Basic macro repetitions
//
// Requirements:
//   * Create a macro to generate hashmaps.
//   * The macro must be able to accept multiple key/value pairs.
//   * Print out the generated hashmap using the `dbg!` macro to ensure it works.

macro_rules! hashmap {
    (
        $($key:tt : $value:tt),+
        $(,)?
    ) => {{
        let mut hashmap = ::std::collections::HashMap::new();
        $(
            hashmap.insert($key, $value);
        )+
        hashmap
    }};
}


fn main() {
    let hashmap = hashmap!{
        1: "a",
        2: "b",
        3: "c",
    };
    dbg!(hashmap);
}
