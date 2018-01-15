use bear_lib_terminal::geometry::Point;

pub fn supercover_line(p0: Point, p1: Point) -> Vec<Point> {
    let dx = p1.x - p0.x;
    let dy = p1.y - p0.x;
    let nx = dx.abs() as f32;
    let ny = dy.abs() as f32;
    let sign_x =
        if dx > 0 {
            1
        } else {
            -1
        };
    let sign_y =
        if dy > 0 {
            1
        } else {
            -1
        };

    let mut p = Point::new(p0.x, p0.y);
    let mut points: Vec<Point> = vec![Point::new(p.x, p.y)];
    let mut ix = 0.0;
    let mut iy = 0.0;
    while ix < nx || iy < ny {
        if (0.5 + ix) / nx == (0.5+iy) /ny {
            //next step is diagonal
            p.x += sign_x;
            p.y += sign_y;
            ix += 1.0;
            iy += 1.0;
        } else if (0.5+ix) / nx < (0.5+iy) / ny {
            //next step is horizontal
            p.x += sign_x;
            ix += 1.0;
        } else {
            //next step is vertical
            p.y += sign_y;
            iy += 1.0;
        }
        points.push(Point::new(p.x, p.y));
    }
    points
}
