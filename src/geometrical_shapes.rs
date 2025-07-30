use raster::{Color, Image};
use rand::Rng;

// ---------------- TRAITS ----------------

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

// ---------------- POINT ----------------

pub struct Point {
    x : i32,
    y : i32
}

impl Point {
    pub fn new(a:i32, b : i32) -> Self {
        Point{
            x: a,
            y: b
        }
    }
    pub fn random(w : i32 , h : i32) -> Self{
        let mut myrange  =  rand::thread_rng();
        let x =  myrange.gen_range(0..w);
        let y =  myrange.gen_range(0..h);
        Self {x, y}
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.x, self.y, self.color());
    }
    fn color(&self) -> Color {
        Color::red()
    }
}


// ---------------- LINE ----------------

pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
       pub fn new(s: Point, e: Point) -> Self {
        Line { 
            start: s,
            end :e
        }
    } 
    pub fn random(w: i32, h: i32) -> Self {
       Self {
            start: Point::random(w, h),
            end: Point::random(w, h)
       }
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let mut x0 = self.start.x;
        let mut y0 = self.start.y;
        let x1 = self.end.x;
        let y1 = self.end.y;

        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 
            1 
        }else{
            -1 
        };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 
            1 
        } else { 
            -1 
        };
        let mut err = dx + dy;
        let color = self.color();

        loop {
            image.display(x0, y0, color.clone());
            if x0 == x1 && y0 == y1 { 
                break; 
            }
            let e2 = 2 * err;
            if e2 >= dy { 
                err += dy;
                x0 += sx; 
            }
            if e2 <= dx { 
                err += dx;
                y0 += sy; 
            }
        }
    }

    fn color(&self) -> Color {
        Color::blue()
    }
}





// ---------------- RECTANGLE ----------------

pub struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self {
            p1: Point::new(p1.x, p1.y),
            p2: Point::new(p2.x, p2.y),
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        // Draw the four lines connecting the corners directly
        Line::new(Point::new(self.p1.x, self.p1.y), Point::new(self.p2.x, self.p1.y)).draw(image);
        Line::new(Point::new(self.p2.x, self.p1.y), Point::new(self.p2.x, self.p2.y)).draw(image);
        Line::new(Point::new(self.p2.x, self.p2.y), Point::new(self.p1.x, self.p2.y)).draw(image);
        Line::new(Point::new(self.p1.x, self.p2.y), Point::new(self.p1.x, self.p1.y)).draw(image);
    }

    fn color(&self) -> Color {
        Color::green()
    }
}



// // ---------------- TRIANGLE ----------------

pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
}

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        Self { 
                p1:Point::new(p1.x, p1.y),
                p2:Point::new(p2.x, p2.y), 
                p3:Point::new(p3.x, p3.y) 
            }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        Line::new(Point::new(self.p1.x, self.p1.y), Point::new(self.p2.x, self.p2.y)).draw(image);
        Line::new(Point::new(self.p2.x, self.p2.y), Point::new(self.p3.x, self.p3.y)).draw(image);
        Line::new(Point::new(self.p3.x, self.p3.y), Point::new(self.p1.x, self.p1.y)).draw(image);
    }

    fn color(&self) -> Color {
        Color::green()
    }
}



// // ---------------- CIRCLE ----------------

// pub struct Circle {
//     center: Point,
//     radius: i32,
// }

// impl Circle {
//     pub fn new(center: &Point, radius: i32) -> Self {
        
//     }

//     pub fn random(w: i32, h: i32) -> Self {
        
//     }
// }

// impl Drawable for Circle {
//    
        
//     }
// }
