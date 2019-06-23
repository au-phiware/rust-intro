




























// A syntax within a syntax: JSON
use serde_json::json;

let john = json!({
    "name": "John Doe",
    "age": 43,
    "phones": [
        "+44 1234567",
        "+44 2345678",
    ],
});

let john: serde_json::Value = john;













// A DSL parser at compile time: SQL
#![feature(plugin)]
#![plugin(postgres_macros)]

let query = sql!("SELECT * FROM users WHERE name = $1");
let bad_query = sql!("SELECT * FORM users WEHRE name = $1");
// error: Invalid syntax at position 10: syntax error at or near "FORM"


















// Declarative
let v = vec![1, 2, 3];

// Imperative
let mut v = Vec::new();
v.push(1);
v.push(2);
v.push(3);








