use std::fs::read_to_string;

use nalgebra::{Vector3, Point2, Vector2};
use z3::{Solver, SatResult, Config, Context, ast::{self, Ast}, Optimize};


fn parse_line(line: &str) -> (Vector3<f64>, Vector3<f64>) {
    let (pos, vel) = line.split_once(" @ ").unwrap();
    let mut pos = pos.split(", ");
    let mut vel = vel.split(", ");
    let px = pos.next().unwrap().trim().parse::<i64>().unwrap();
    let py = pos.next().unwrap().trim().parse::<i64>().unwrap();
    let pz = pos.next().unwrap().trim().parse::<i64>().unwrap();
    let vx = vel.next().unwrap().trim().parse::<i64>().unwrap();
    let vy = vel.next().unwrap().trim().parse::<i64>().unwrap();
    let vz = vel.next().unwrap().trim().parse::<i64>().unwrap();
    (Vector3::new(px as f64, py as f64, pz as f64), Vector3::new(vx as f64, vy as f64, vz as f64))
}

const EPS1: f64 = 1e-10;

const EPS2: f64 = 10.0;

fn line_line_intersect(
    p1: &Vector3<f64>,
    p2: &Vector3<f64>,
    p3: &Vector3<f64>,
    p4: &Vector3<f64>,
) -> Option<(Vector3<f64>, Vector3<f64>, f64, f64)> {
    let p13 = p1 - p3;
    let p43 = p4 - p3;

    if p43.x.abs() < EPS1 && p43.y.abs() < EPS1 && p43.z.abs() < EPS1 {
        return None;
    }

    let p21 = p2 - p1;

    if p21.x.abs() < EPS1 && p21.y.abs() < EPS1 && p21.z.abs() < EPS1 {
        return None;
    }

    let d1343 = p13.dot(&p43);
    let d4321 = p43.dot(&p21);
    let d1321 = p13.dot(&p21);
    let d4343 = p43.norm_squared();
    let d2121 = p21.norm_squared();

    let denom = d2121 * d4343 - d4321 * d4321;

    if denom.abs() < EPS1 {
        return None;
    }

    let numer = d1343 * d4321 - d1321 * d4343;

    let mua = numer / denom;
    let mub = (d1343 + d4321 * mua) / d4343;

    let pa = p1 + mua * p21;
    let pb = p3 + mub * p43;

    Some((pa, pb, mua, mub))
}

fn check_intersection(pos1: &Vector3<f64>, vel1: &Vector3<f64>, pos2: &Vector3<f64>, vel2: &Vector3<f64>) -> Option<Vector3<f64>> {
    let p1 = Vector3::new(pos1.x as f64, pos1.y as f64, pos1.z as f64);
    let p2: nalgebra::Matrix<f64, nalgebra::Const<3>, nalgebra::Const<1>, nalgebra::ArrayStorage<f64, 3, 1>> = Vector3::new(p1.x + vel1.x as f64, p1.y + vel1.y as f64, p1.z + vel1.z as f64);
    let p3 = Vector3::new(pos2.x as f64, pos2.y as f64, pos2.z as f64);
    let p4 = Vector3::new(p3.x + vel2.x as f64, p3.y + vel2.y as f64, p3.z + vel2.z as f64);

    if let Some((pa, pb, mua, mub)) = line_line_intersect(&p1, &p2, &p3, &p4) {
        // Check whether the points have a distance of almost 0
        let dist = pa.metric_distance(&pb);
        if dist < EPS2 && mua > 0.0 && mub > 0.0 {
            // Check whether the points are within the range of the line segments
            return Some(Vector3::new(pa.x, pa.y, pa.z));
        }
    }
    None
}


// check intersection plus integer time
fn check_intersection2(pos1: &Vector3<f64>, vel1: &Vector3<f64>, pos2: &Vector3<f64>, vel2: &Vector3<f64>) -> bool {
    let p1 = Vector3::new(pos1.x as f64, pos1.y as f64, pos1.z as f64);
    let p2 = Vector3::new(p1.x + vel1.x as f64, p1.y + vel1.y as f64, p1.z + vel1.z as f64);
    let p3 = Vector3::new(pos2.x as f64, pos2.y as f64, pos2.z as f64);
    let p4 = Vector3::new(p3.x + vel2.x as f64, p3.y + vel2.y as f64, p3.z + vel2.z as f64);

    if let Some((pa, pb, mua, mub)) = line_line_intersect(&p1, &p2, &p3, &p4) {
        // Check whether the points have a distance of almost 0
        let dist = pa.metric_distance(&pb);
        if dist < EPS2 && (mua - mub).abs() < 1e-5 && (mua - mua.round()).abs() < 1e-2 && (mub - mub.round()).abs() < 1e-2 {
            // Check whether the points are within the range of the line segments
            return true;
        }
    }
    false
}

fn intersection_point(line1_origin: Point2<f64>, line1_direction: Vector2<f64>, line2_origin: Point2<f64>, line2_direction: Vector2<f64>) -> Option<Point2<f64>> {
    let delta = line2_origin - line1_origin;

    let det = line2_direction.x * line1_direction.y - line2_direction.y * line1_direction.x;

    if det.abs() < 1e-10 {
        // Lines are parallel or coincident
        return None;
    }

    let s = (line2_direction.x * delta.y - line2_direction.y * delta.x) / det;
    let t = (line1_direction.x * delta.y - line1_direction.y * delta.x) / det;

    // check whether both are positive

    if s < 0.0 || t < 0.0 {
        return None;
    }

    Some(line1_origin + s * line1_direction)
}

fn remove_z(v: &Vector3<f64>) -> Vector3<f64> {
    Vector3::new(v.x, v.y, 0.0)
}


fn main() {
    let input = read_to_string("input.txt").unwrap().lines().filter(|l| !l.is_empty()).map(parse_line).collect::<Vec<_>>();


    let range = 200000000000000.0..400000000000000.0;

    //let range = 7.0f64..=27.0f64;

    // for each combination of two lines check if they intersect within the range
    let mut result = 0;
    let mut result2 = 0;
    for i in 0..input.len() {
        for j in i+1..input.len() {
            let (pos1, vel1) = input[i];
            let (pos2, vel2) = input[j];

            let mut did1 = false;
            let mut did2 = false;
            if let Some(p) = check_intersection(&remove_z(&pos1), &remove_z(&vel1), &remove_z(&pos2), &remove_z(&vel2)) {
                if range.contains(&p.x) && range.contains(&p.y) {
                    result += 1;
                    did1 = true;
                }
            }

            let p1 = Point2::new(pos1.x, pos1.y);
            let v1 = Vector2::new(vel1.x, vel1.y);
            let p2 = Point2::new(pos2.x, pos2.y);
            let v2 = Vector2::new(vel2.x, vel2.y);
            if let Some (p) = intersection_point(p1, v1, p2, v2) {
                if range.contains(&p.x) && range.contains(&p.y) {
                    result2 += 1;
                    did2 = true;
                }
            }

            if did1 != did2 {
                println!("{} {} {} {} {} {}", pos1, vel1, pos2, vel2, did1, did2);
            }
        }
    }

    println!("Result 1 (using 3D logic): {}", result);
    println!("Result 2 (using 2D logic): {}", result2);

    /*
        Equation system for part 2:
        p + t1 * v = p1 + t1 * v1
        p + t2 * v = p2 + t2 * v2
        p + t3 * v = p3 + t3 * v3
        ...
     */

    // I tried but failed to solve this underdetermined equation system by myself so i gave up and just used z3...

    // Create Z3 context
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    // Variables for the line to be found
    let p0_x = ast::Real::new_const(&ctx, "p0_x");
    let p0_y = ast::Real::new_const(&ctx, "p0_y");
    let p0_z = ast::Real::new_const(&ctx, "p0_z");
    let v_x = ast::Real::new_const(&ctx, "v_x");
    let v_y = ast::Real::new_const(&ctx, "v_y");
    let v_z = ast::Real::new_const(&ctx, "v_z");


    // Formulate equations
    for (i, (p, v)) in input.iter().enumerate() {
        let tn = ast::Real::new_const(&ctx, format!("t{}", i));
        let px = ast::Real::from_real_str(&ctx, &p.x.to_string(), &1.to_string()).unwrap();
        let py = ast::Real::from_real_str(&ctx, &p.y.to_string(), &1.to_string()).unwrap();
        let pz = ast::Real::from_real_str(&ctx, &p.z.to_string(), &1.to_string()).unwrap();
        let vx = ast::Real::from_real_str(&ctx, &v.x.to_string(), &1.to_string()).unwrap();
        let vy = ast::Real::from_real_str(&ctx, &v.y.to_string(), &1.to_string()).unwrap();
        let vz = ast::Real::from_real_str(&ctx, &v.z.to_string(), &1.to_string()).unwrap();
        solver.assert(&(&p0_x + &tn * &v_x)._eq(&(&px + &tn * &vx)));
        solver.assert(&(&p0_y + &tn * &v_y)._eq(&(&py + &tn * &vy)));
        solver.assert(&(&p0_z + &tn * &v_z)._eq(&(&pz + &tn * &vz)));
    }

    // Check satisfiability
    if solver.check() == SatResult::Sat {
        let model = solver.get_model().unwrap();
        // Retrieve the values of the variables for the line to be found
        let p0_val = (
            model.eval(&p0_x, true).unwrap().as_real().unwrap(),
            model.eval(&p0_y, true).unwrap().as_real().unwrap(),
            model.eval(&p0_z, true).unwrap().as_real().unwrap(),
        );
        let _v_val = (
            model.eval(&v_x, true).unwrap().as_real().unwrap(),
            model.eval(&v_y, true).unwrap().as_real().unwrap(),
            model.eval(&v_z, true).unwrap().as_real().unwrap(),
        );

        println!("result 2: {}", p0_val.0.0 + p0_val.1.0 + p0_val.2.0)
    } else {
        println!("No solution found.");
    }
}
