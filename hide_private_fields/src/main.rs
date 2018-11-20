mod figures {
    pub struct Rectangle {
        width: f64,
        length: f64,
    }

    pub struct Square {
        side: f64,
    }

    impl Rectangle {
        pub fn new(width: f64, length: f64) -> Option<Rectangle> {
            if width > 0. && length > 0. {
                Some(Rectangle { length, width })
            } else {
                None
            }
        }
        pub fn area(&self) -> f64 {
            self.width * self.length
        }
    }

    impl Square {
        pub fn new(side: f64) -> Option<Square> {
            if side > 0. {
                Some(Square { side })
            } else {
                None
            }
        }
        pub fn area(&self) -> f64 {
            self.side * self.side
        }
    }
}

// cargo run --bin hide_private_fields
fn main() {
    use figures::{Rectangle, Square};

    let rect1 = Rectangle::new(3., 5.).unwrap();
    let rect2 = Rectangle::new(4., 6.).unwrap();
    // error: field `width` is private
    //        let rect3 = Rectangle {
    //            width: 3.,
    //            length: 5.,
    //        };

    let sq1 = Square::new(8.).unwrap();
    let sq2 = Square::new(4.).unwrap();

    for r in [&rect1, &rect2].iter() {
        println!("Площадь равна {}", r.area());
    }

    for s in [&sq1, &sq2].iter() {
        println!("Площадь равна {}", s.area());
    }
}
