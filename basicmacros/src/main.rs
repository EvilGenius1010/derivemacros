#[macro_use]
mod dec_macro;
use customderivemacros::SerializeNovice;

#[derive(Debug,SerializeNovice)]
struct Rect{
    width: u32,
    height: u32,
}

// #[derive(SerializeNovice)]
// enum TestEnum{
//     Zero,
//     Positive,
//     Negative
// }


// #[derive(SerializeNovice)]
// union ABC {
//     a1:u32
// }

struct Square{
    side: u32,
}

trait Shape {
    fn area(&self) -> u32;
    fn perimeter(&self) -> u32;
}

impl Shape for Rect{
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}


impl Shape for Square{
    fn area(&self) -> u32 {
        self.side * self.side
    }
    fn perimeter(&self) -> u32 {
        4 * self.side
    }
}

/// this allows us to take any input which implements given trait Shape
fn get_area_and_perimeter(s: &impl Shape) -> (u32, u32) {
    (s.area(), s.perimeter())
}

impl std::fmt::Display for Rect{
    /// struct Position {
    ///     longitude: f32,
    ///     latitude: f32,
    /// }
    ///
    /// impl fmt::Display for Position {
    ///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         write!(f, "({}, {})", self.longitude, self.latitude)
    ///     }
    /// }

fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Width: {}", self.width)
    }
}

fn main() {
    let rect = Rect { width: 30, height: 50 };
    let square = Square { side: 40 };

    println!("Rectangle is {:?}", rect);

    println!("Rectangle area: {}", rect.area());
    println!("Square area: {}", square.area());

    let (rect_area, rect_perimeter) = get_area_and_perimeter(&rect);
    let (square_area, square_perimeter) = get_area_and_perimeter(&square);

    let vector1 = vector![1, 2, 3, 4, 5];
    println!("Vector: {:?}", vector1);
    rect.hello_from_macro();
}
