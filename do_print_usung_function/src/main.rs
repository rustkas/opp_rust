use std::fmt;
use std::fmt::Display;

pub trait Area {
    fn area(&self) -> f64;
}

pub struct Rectangle {
    width: f64,
    length: f64,
}

pub struct Square {
    rectangle: Rectangle,
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.length
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.rectangle.area()
    }
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
            Some(Square {
                rectangle: Rectangle {
                    length: side,
                    width: side,
                },
            })
        } else {
            None
        }
    }
    pub fn area(&self) -> f64 {
        self.rectangle.area()
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "прямоугольник({}, {})",
            self.width, self.length
        )
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "квадрат({}, {})",
            self.rectangle.width, self.rectangle.length
        )
    }
}

pub trait Figure: Area + fmt::Display {}
impl Figure for Rectangle {}

impl Figure for Square {}

fn print_figures_and_areas(figures: &[&Figure]) {
    for f in figures.iter() {
        println!("Площадь {} равна {}", f, f.area());
    }
}

fn print_areas(figures: &[&Area]) {
    for f in figures.iter() {
        println!("Площадь равна {}", f.area());
    }
}
//fn print_figures_and_areas1(figures: &[&(Area + Display)]) {
//    for f in figures.iter() {
//        println!("Площадь равна {}", f.area());
//    }
//}

fn print_figures_and_areas2(figures_with_area: &[&Area], figures_with_display: &[&Display]) {
    for (f_a, f_d) in figures_with_area.iter().zip(figures_with_display.iter()) {
        println!("Площадь {} равна {}", f_d, f_a.area());
    }
}
// cargo run --bin do_print_usung_function
fn main() {
    let rect1 = Rectangle::new(3., 5.).unwrap();
    let rect2 = Rectangle::new(4., 6.).unwrap();

    let sq1 = Square::new(8.).unwrap();
    let sq2 = Square::new(4.).unwrap();

    // let figures_with_display: [&Display; 4] = [&rect1, &rect2, &sq1, &sq2];

    print_areas(&[&rect1, &rect2, &sq1, &sq2]);
    print_figures_and_areas2(&[&rect1, &rect2, &sq1, &sq2], &[&rect1, &rect2, &sq1, &sq2]);
    print_figures_and_areas(&[&rect1, &rect2, &sq1, &sq2]);
}
