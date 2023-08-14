use indoc::*;
use std::fs;
use svg::{
    node::element::path::{Command, Data, Position},
    parser::Event,
};

struct Shape {
    name: String,
    surface: Vec<[f32; 2]>,
}

fn read_shapes() -> Vec<Shape> {
    let mut shapes = Vec::new();

    let path = "graphics/drawing.svg";
    let mut content = String::new();
    for event in svg::open(path, &mut content).unwrap() {
        match event {
            Event::Tag("path", _, attributes) => {
                let (Some(id), Some(path)) = (attributes.get("id"), attributes.get("d")) else {
                    continue;
                };

                const PREFIX: &'static str = "export_shape_";

                if !id.starts_with(PREFIX) {
                    continue;
                }

                let name = id[PREFIX.len()..].to_uppercase();
                let surface = path_to_points(path);

                shapes.push(Shape { name, surface });
            }
            _ => {}
        }
    }

    shapes
}

fn path_to_points(path: &str) -> Vec<[f32; 2]> {
    let data = Data::parse(path).unwrap();
    let mut points = Vec::with_capacity(data.len());

    for command in data.iter() {
        match &command {
            &Command::Move(pos, args) => {
                for pts in args.chunks_exact(2) {
                    points.push(abs_point(&points, pos, pts[0], pts[1]));
                }
            }
            &Command::Line(pos, args) => {
                for pts in args.chunks_exact(2) {
                    points.push(abs_point(&points, pos, pts[0], pts[1]));
                }
            }
            &Command::HorizontalLine(pos, args) => {
                for pt in args.into_iter() {
                    points.push([
                        abs_point(&points, pos, *pt, 0.)[0],
                        points.last().unwrap()[1],
                    ]);
                }
            }
            &Command::VerticalLine(pos, args) => {
                for pt in args.into_iter() {
                    points.push([
                        points.last().unwrap()[0],
                        abs_point(&points, pos, 0., *pt)[1],
                    ]);
                }
            }
            _ => {}
        }
    }

    points
}

fn abs_point(vec: &Vec<[f32; 2]>, pos: &Position, x: f32, y: f32) -> [f32; 2] {
    match pos {
        Position::Absolute => [x, y],
        Position::Relative => {
            if let Some(last) = vec.last() {
                [last[0] + x, last[1] + y]
            } else {
                [x, y]
            }
        }
    }
}

fn normalize_shapes(shapes: &mut Vec<Shape>) {
    for shape in shapes {
        let mut min = shape.surface[0];
        let mut max = shape.surface[0];

        for point in &shape.surface {
            min = [min[0].min(point[0]), min[1].min(point[1])];
            max = [max[0].max(point[0]), max[1].max(point[1])];
        }

        let scale = 1. / (max[0] - min[0]);
        let center_x = (max[0] + min[0]) / 2.;
        let center_y = (max[1] + min[1]) / 2.;

        for point in &mut shape.surface {
            point[0] -= center_x;
            point[1] -= center_y;

            point[0] *= scale;
            point[1] *= -scale;
        }
    }
}

fn generate_code(shapes: &Vec<Shape>) {
    let mut code = formatdoc! {"
        // THIS CODE IS GENERATED BY THE BUILD SCRIPT.
        // ANY CHANGE WILL BE OVERWRITTEN.

        use crate::math::*;
    "};

    for shape in shapes {
        let mut points = String::new();

        for point in &shape.surface {
            points.push_str(&format!(
                "    Vec2 {{ x: {}, y: {} }},\n",
                point[0], point[1]
            ));
        }

        code.push_str(&formatdoc! {"
            
            pub const {}: [Vec2; {}] = [
            {}];
        ", shape.name, shape.surface.len(), points});
    }

    fs::write("src/shapes.rs", code).unwrap();
}

pub fn main() {
    let mut shapes = read_shapes();
    normalize_shapes(&mut shapes);
    generate_code(&shapes);

    println!("cargo:rerun-if-changed=drawings");
}
