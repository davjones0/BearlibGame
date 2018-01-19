use num_traits::{Float, NumCast, NumAssignOps, Signed};
use num_traits::cast::ToPrimitive;

use bear_lib_terminal::geometry::Point;

pub struct Supercover {
    point: Point,
    ix: f32,
    iy: f32,
    sign_x: i32,
    sign_y: i32,
    ny: f32,
    nx: f32
}

impl Supercover {
    pub fn new(start: Point, end: Point) -> Self {
        let (dx, dy) = (end.x - start.x, end.y - start.y);
        println!("-----------------------");
        println!("start Point: {:?}, end Point: {:?}", start, end);
        println!("dx: {}, dy: {}", dx, dy);
        Self {
            point: start,
            ix: 0.0,
            iy: 0.0,
            sign_x: dx.signum(),
            sign_y: dy.signum(),
            nx: dx.abs().to_f32().unwrap(),
            ny: dy.abs().to_f32().unwrap(),
        }

    }
}

impl Iterator for Supercover {
    type Item = Point;

    fn next(&mut self) -> Option<(Point)> {
        println!("ix:{} <= nx:{} && iy: {} <= ny: {}", self.ix, self.nx, self.iy, self.ny);
        if self.ix <= self.nx && self.iy <= self.ny {
            let point = self.point;
            println!("{} and {}", point.x, point.y);
            let comparison = ((0.5 + self.ix) / self.nx) - ((0.5 + self.iy) / self.ny);
            println!("comparison: {}", comparison);
            // If the comparison is equal then jump diagonally
            if comparison == 0.0 {
                self.point.x += self.sign_x;
                self.point.y += self.sign_y;
                self.ix += 1.0;
                self.iy += 1.0;
            } else if comparison < 0.0 {
                self.point.x += self.sign_x;
                self.ix += 1.0;
            } else {
                self.point.y += self.sign_y;
                self.iy += 1.0;
            }

            Some(point)
        } else {
            None
        }
    } 
}

#[test]
fn supercover_tests() {
    let supercover = |a, b| Supercover::new(a, b).collect::<Vec<_>>();

    // supercover should jump diagonally if the difference is equal

    assert_eq!(
        supercover(Point::new(0, 0), Point::new(5, 5)),
        [Point::new(0, 0), Point::new(1, 1), Point::new(2, 2), Point::new(3, 3), Point::new(4, 4), Point::new(5, 5)]
    );

    assert_eq!(
        supercover(Point::new(0, 0), Point::new(3, 1)),
        [Point::new(0, 0), Point::new(1, 0), Point::new(2, 1), Point::new(3, 1)]
    );

    assert_eq!(
        supercover(Point::new(0, 0), Point::new(0, 5)),
        [Point::new(0, 0), Point::new(0, 1), Point::new(0, 2), Point::new(0, 3), Point::new(0, 4), Point::new(0, 5)]
    );

    assert_eq!(
        supercover(Point::new(0, 0), Point::new(5, 0)),
        [Point::new(0, 0), Point::new(1, 0), Point::new(2, 0), Point::new(3, 0), Point::new(4, 0), Point::new(5, 0)]
    );

    assert_eq!(
        supercover(Point::new(12, 10), Point::new(12, 11)),
        [Point::new(12, 10), Point::new(12, 11), Point::new()]
    );
}

// pub fn supercover_line(p0: Point, p1: Point) -> Vec<Point> {
//     println!("super{:?},{:?}", p0, p1);
//     let dx = p1.x - p0.x;
//     let dy = p1.y - p0.x;
//     let nx = dx.abs() as f32;
//     let ny = dy.abs() as f32;
//     let sign_x =
//         if dx > 0 {
//             1
//         } else {
//             -1
//         };
//     let sign_y =
//         if dy > 0 {
//             1
//         } else {
//             -1
//         };

//     let mut p = Point::new(p0.x, p0.y);
//     let mut points: Vec<Point> = vec![Point::new(p.x, p.y)];
//     let mut ix = 0.0;
//     let mut iy = 0.0;
//     while ix < nx || iy < ny {
//         if (0.5 + ix) / nx == (0.5+iy) /ny {
//             //next step is diagonal
//             p.x += sign_x;
//             p.y += sign_y;
//             ix += 1.0;
//             iy += 1.0;
//         } else if (0.5+ix) / nx < (0.5+iy) / ny {
//             //next step is horizontal
//             p.x += sign_x;
//             ix += 1.0;
//         } else {
//             //next step is vertical
//             p.y += sign_y;
//             iy += 1.0;
//         }
//         points.push(Point::new(p.x, p.y));
//     }
//     points
// }
