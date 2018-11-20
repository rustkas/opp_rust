// Обобщение через перечисление
//
// Самый простой способ обобщить фигуры — использовать перечисление:

pub struct Rectangle {
    width: f64,
    length: f64,
}

pub struct Square {
    side: f64,
}

pub enum Figure {
    Rect(Rectangle),
    Sq(Square),
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

impl Figure {
    pub fn area(&self) -> f64 {
        match self {
            &Figure::Rect(ref r) => r.area(),
            &Figure::Sq(ref s) => s.area(),
        }
    }
}

// cargo run --bin common_enums
fn main() {
    let rect1 = Figure::Rect(Rectangle::new(3., 5.).unwrap());
    let rect2 = Figure::Rect(Rectangle::new(4., 6.).unwrap());

    let sq1 = Figure::Sq(Square::new(8.).unwrap());
    let sq2 = Figure::Sq(Square::new(4.).unwrap());

    let figures = [&rect1, &rect2, &sq1, &sq2];

    for f in figures.iter() {
        println!("Площадь равна {}", f.area());
    }
}
