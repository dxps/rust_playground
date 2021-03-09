// This is a generic struct.
// (Note: since x and y are just defined but not used here,
//        they are prefixed with underscore to remove any 'dead code' warning.)
struct Coordinate<T> {
    _x: T,
    _y: T,
}

// This is a basic macro that capitalizes the first letter of a String.
macro_rules! capitalize {
    ($a: expr) => {
        let mut v: Vec<char> = $a.chars().collect();
        v[0] = v[0].to_uppercase().nth(0).unwrap();
        $a = v.into_iter().collect();
    };
}

// defines a struct that can be copied and debugged
#[derive(Clone, Copy)]
struct CopyCoordinate {
    x: i8,
    y: i8,
}

// this function demonstrates that the copy trait on the CopyCoordinate enables us to use the
// CopyCoordinate again after being used here
fn print(point: CopyCoordinate) {
    println!("{} {}", point.x, point.y);
}

fn main() {
    // shows that different data types can be made with this generic struct
    let _one = Coordinate { _x: 50, _y: 50 };
    let _two = Coordinate { _x: 5.6, _y: 5.6 };

    let mut s = String::from("test");
    // fires our macro
    capitalize!(s);

    // makes the s capitalized without us having to define it using let.
    println!("{}", s);

    let test_copycoord = CopyCoordinate { x: 1, y: 2 };
    print(test_copycoord);
    println!("{}", test_copycoord.x)
}
