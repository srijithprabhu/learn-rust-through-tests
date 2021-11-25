use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

impl Display for City {
    // `f` is a buffer, and this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RGB ({0}, {1}, {2}) 0x{0:02X}{1:02X}{2:02X}", self.red, self.green, self.blue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct CityTestCase {
        city: City,
        expected: String
    }

    #[test]
    fn test_city_display() {
        for testcase in [
            CityTestCase {
                city: City { name: "Dublin", lat: 53.347778, lon: -6.259722},
                expected: String::from("Dublin: 53.348°N 6.260°W")
            },
            CityTestCase {
                city: City { name: "Oslo", lat: 59.95, lon: 10.75 },
                expected: String::from("Oslo: 59.950°N 10.750°E")
            },
            CityTestCase {
                city: City { name: "Vancouver", lat: 49.25, lon: -123.1 },
                expected: String::from("Vancouver: 49.250°N 123.100°W")
            }
        ].iter() {
            assert_eq!(testcase.expected, format!("{}", testcase.city));
        }
    }

    struct ColorTestCase {
        color: Color,
        expected_debug: String,
        expected_display: String
    }

    #[test]
    fn test_color_print() {
        for testcase in [
            ColorTestCase {
                color: Color { red: 128, green: 255, blue: 90 },
                expected_debug: String::from("Color { red: 128, green: 255, blue: 90 }"),
                expected_display: String::from("RGB (128, 255, 90) 0x80FF5A")
            },
            ColorTestCase {
                color: Color { red: 0, green: 3, blue: 254 },
                expected_debug: String::from("Color { red: 0, green: 3, blue: 254 }"),
                expected_display: String::from("RGB (0, 3, 254) 0x0003FE")
            },
            ColorTestCase {
                color: Color { red: 0, green: 0, blue: 0 },
                expected_debug: String::from("Color { red: 0, green: 0, blue: 0 }"),
                expected_display: String::from("RGB (0, 0, 0) 0x000000")
            }
        ] {
            assert_eq!(testcase.expected_debug, format!("{:?}", testcase.color));
            assert_eq!(testcase.expected_display, format!("{}", testcase.color));
        }
    }
}
