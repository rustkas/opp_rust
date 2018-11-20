struct Rectangle {
    width: f64,
    length: f64,
}

struct Square {
    side: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.length
    }
}

impl Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Rectangle {
    fn new(width: f64, length: f64) -> Option<Rectangle> {
        if width > 0. && length > 0. {
            Some(Rectangle { length, width })
        } else {
            None
        }
    }
    // ...
}

impl Square {
    fn new(side: f64) -> Option<Square> {
        if side > 0. {
            Some(Square { side })
        } else {
            None
        }
    }
    // ...
}

// cargo run --bin add_constructor
fn main() {
    let rect1 = Rectangle::new(3., 5.).unwrap();
    let rect2 = Rectangle::new(4., 6.).unwrap();;
    // этот вызов приводит к панике
    let rect3 = Rectangle::new(-4., 6.).unwrap();;

    let sq1 = Square::new(8.).unwrap();
    let sq2 = Square::new(4.).unwrap();

    for r in [&rect1, &rect2, &rect3].iter() {
        println!("Площадь равна {}", r.area());
    }

    for s in [&sq1, &sq2].iter() {
        println!("Площадь равна {}", s.area());
    }
    //    let rect1 = Rectangle { width: 3., length: 5. };
    //    let rect2 = Rectangle { width: 4., length: 6. };
    //
    //    let sq1 = Square { side: 8. };
    //    let sq2 = Square { side: 4. };
    //
    //    let rects = [&rect1, &rect2];
    //    let squares = [&sq1, &sq2];
    //
    //    for r in rects.iter() {
    //        println!("Площадь равна {}", r.area());
    //    }
    //
    //    for s in squares.iter() {
    //        println!("Площадь равна {}", s.area());
    //    }
}
