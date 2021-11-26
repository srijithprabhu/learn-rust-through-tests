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
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn calculate_area(&self) -> f32 {
        return match self {
            Rectangle {
                top_left: Point{
                    x:a,
                    y:b
                },
                bottom_right: Point{
                    x:c,
                    y:d
                }
            } => (c - a) * (b - d)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct RectangleTestCase {
        rectangle: Rectangle,
        expected_area: f32
    }

    #[test]
    fn test_calculate_rectangle_area() {
        for test_case in [
            RectangleTestCase {
                rectangle: Rectangle {
                    top_left: Point{
                        x: 0f32,
                        y: 0f32
                    },
                    bottom_right: Point {
                        x: 1f32,
                        y: -1f32
                    }
                },
                expected_area: 1f32
            }
        ] {
            assert_eq!(test_case.expected_area, test_case.rectangle.calculate_area())
        }
    }
}
