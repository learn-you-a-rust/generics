
// this def is in the std lib
enum Option<T> {
    Some(T),
    None,
}

struct Point<T> {
    x: T,
    y: T,
}

struct Cpoint<T, U> {
    x: T,
    y: U,
}

// method definition for struct, implementing methods on Points of generic type;
// the declaration impl<T> specifies that we’re implementing methods on the type
// Point<T>.
impl<T> Point<T> {
    // returns a reference to the data in the field x
    fn x(&self) -> &T {
        &self.x
    }
}

// this implements methods only for certain types of Points
impl<T> Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// this combines two Cpoint<T, U>s;
// the type parameters in the method signature don't have to match those
// in the struct definition.
// the generic parameters T and U are declared after impl, because they go with the struct definition. 
// the generic parameters V and W are declared after fn mixup, because they’re only relevant to the method.
impl<T, U> Cpoint<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    // function generics
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // struct generics
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
