use raster::{Color, Image};
use rand::Rng;

// ---------------- TRAITS ----------------

pub trait Drawable {
    fn draw(&self, img: &mut Image);
    fn color(&self) -> Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}




// ---------------- POINT ----------------

pub struct Point {
        x : i32 ,
        y : i32
}

impl Point {
    pub fn new(a:i32 , b : i32) -> Self {
        Point{
            x: a ,
            y : b
        }
    }
    pub fn random(w : i32 , h : i32) -> Self{
        let mut myrange  =  rand::thread_rng();
        let x =  myrange.gen_range(0..w);
        let y =  myrange.gen_range(0..h);
        Point {
            x ,
            y
        }
    }
}

impl Drawable for Point {
    fn draw(&self, img: &mut Image) {
        
    }

    fn color(&self) -> Color {
        
    }
}





// ---------------- LINE ----------------

pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
       pub fn new(s :  Point ,  e : Point) -> Self {
        Line {
            start :  s , 
            end : e
        }
    } 

    pub fn random(w: i32, h: i32) -> Self {
       
    }
}

impl Drawable for Line {
    fn draw(&self, img: &mut Image) {
        // implementation here
    }

    fn color(&self) -> Color {
        // implementation here
    }
}





// ---------------- RECTANGLE ----------------

pub struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    pub fn new(top_left: &Point, bottom_right: &Point) -> Self {
      
    }
}

impl Drawable for Rectangle {
    fn draw(&self, img: &mut Image) {
        
    }

    fn color(&self) -> Color {
        
    }
}







// ---------------- TRIANGLE ----------------

pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
}

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        
    }
}

impl Drawable for Triangle {
    fn draw(&self, img: &mut Image) {
        
    }

    fn color(&self) -> Color {
        
    }
}








// ---------------- CIRCLE ----------------

pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Self {
        
    }

    pub fn random(w: i32, h: i32) -> Self {
        
    }
}

impl Drawable for Circle {
    fn draw(&self, img: &mut Image) {
        
    }

    fn color(&self) -> Color {
        
    }
}
