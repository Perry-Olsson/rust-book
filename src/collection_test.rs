use std::collections::HashMap;

pub fn test_vector() {
    let mut my_vec = vec![1, 2, 3];
    my_vec.push(1123);
    println!("{my_vec:?}")
}

pub fn test_vector_2() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = v[0];

    v.push(6);

    println!("The first element is: {first}");
}

pub fn test_hash_map() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

impl SpreadsheetCell {
    fn print(&self) {
        match self {
            Self::Int(num) => println!("{}", num),
            Self::Float(num) => println!("{}", num),
            Self::Text(val) => println!("{}", val)
        }
    }
}

pub fn spreadsheet_cell() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let row_arr = [
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    
    for cell in row {
        cell.print()
    }

    for cell in row_arr {
        cell.print()
    }
}