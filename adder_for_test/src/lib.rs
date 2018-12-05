#[derive(Debug, PartialEq)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
    pub fn panic() {
        panic!("Bazah");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn comparison() {
        let rec1 = Rectangle { length: 1, width: 2 };
        let rec2 = Rectangle { length: 1, width: 2 };
        let rec3 = Rectangle { length: 1, width: 3 };

        assert_eq!(rec2, rec1, "Equal");
        assert_ne!(rec3, rec1, "Not equal");
    }

    #[test]
    #[should_panic(expected = "Bazah")]
    fn panic() {
        Rectangle::panic();
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two doesn't equal four"))
        }
    }
}
