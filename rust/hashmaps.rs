
/// simple hashmap macro
///     usage: hashmap!['A' => 0, 'C' => 0, 'G' => 0, 'T' => 0];
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}
