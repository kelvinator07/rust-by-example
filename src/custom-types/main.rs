// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right corners are in space.
    top_left: Point,
    bottom_right: Point,
}

// Add a function rect_area which calculates the area of a Rectangle (try using nested destructuring).
fn rect_area(rectangle: Rectangle) -> f32 {
    let Rectangle {
        top_left: Point {
            x: top1,
            y: bottom1,
        },
        bottom_right: Point {
            x: top2,
            y: bottom2,
        },
    } = rectangle;
    top1 * bottom2 + top2 * bottom1
}

// Add a function square which takes a Point and a f32 as arguments,
// and returns a Rectangle with its top left corner on the point, and a width and height corresponding to the f32.
fn square(point: Point, num: f32) -> Rectangle {
    Rectangle {
        top_left: point,
        bottom_right: Point { x: num, y: num },
    }
}

fn main() {
    println!("========Custom Types========");
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 30;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 5.4, y: 0.4 };
    let another_point: Point = Point { x: 10.6, y: 0.2 };

    // Access the fields of the point
    println!("Points coordinates: ({} {}) ", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    let bottom_right: Point = Point {
        x: 6.6,
        ..another_point
    };

    // `bottom_right.y` will be the same as `another_point.y` because we used that field from `another_point`
    println!(
        "Second Points coordinates: ({} {}) ",
        bottom_right.x, bottom_right.y
    );

    // Destructure the point using a `let` binding
    let Point { x: top, y: bottom } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: top, y: bottom },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.3);

    // Access the fields of a tuple struct
    println!("Pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("Pair contains {:?} and {:?}", integer, decimal);

    let area = rect_area(_rectangle);

    println!("Area of rectangle is {:.3?}", area);

    println!("Rectangle square is {:?}", square(point, 2.5));

    println!("\n\n========Enums========");

    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted = WebEvent::Paste("my_text".to_owned());
    let clicked = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnLoad;

    inspect(pressed);
    inspect(pasted);
    inspect(clicked);
    inspect(load);
    inspect(unload);

    println!("\n\n========Enum Typed Alias========");

    // We can refer to each variant via its alias, not its long and inconvenient name.
    let x = Operations::Add;
}

// Create an `enum` to classify a web event. Note how both names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`. Each is different and independent.
enum WebEvent {
    // An `enum` variant may either be `unit-like`,
    PageLoad,
    PageUnLoad,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded!"),
        WebEvent::PageUnLoad => println!("Page unloaded!"),
        // Destructure `c` from inside the `enum` variant.
        WebEvent::KeyPress(c) => println!("Pressed '{}'.", c),
        WebEvent::Paste(s) => println!("Pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => println!("Clicked at x={}, y={}.", x, y),
    }
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}
