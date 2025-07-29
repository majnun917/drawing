// use rand::Rng;

// mod geometrical_shapes;

// pub struct Point {
//         x : i32 ,
//         y : i32
// }
// impl Point {
//     pub fn new(a:i32 , b i32) -> Self {
//         Point{
//             x: a ,
//             y : b
//         }
//     }
//     pub fn random(w : i32 , h : i32) -> Self{
//         let mut myrange  =  rand::thread_rng();
//         let x =  myrange.gen_range(0..w);
//         let y =  myrange.gen_range(0..h);
//         Point {
//             x ,
//             y
//         }
//     }
// }

// pub struct Line {
//      start :  Point , 
//      end  :  Point
// }
// impl Line {
//     pub fn new(s :  Point ,  e : Point) -> Self {
//         Line {
//             start :  s , 
//             end : e
//         }
//     } 
// }