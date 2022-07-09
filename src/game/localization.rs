extern crate nalgebra;

use super::super::server::see::Flag;
use nalgebra::base::{MatrixXx2, MatrixXx1};

#[derive(Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub body: f64
}

fn angle(x0: f64, y0: f64, x1: f64, y1: f64, offset: f64) -> f64 {
    let mut direction = (y1 - y0).atan2(x1 - x0).to_degrees() - offset;
    if direction >= 180.0 {
        direction -= 360.0;
    } else if direction < -180.0 {
        direction += 360.0;
    }
    direction
}

// based on https://www3.nd.edu/~cpoellab/teaching/cse40815/Chapter10.pdf
fn triangulate(flags: &Vec<Flag>) -> Option<Position> {
    let flag0 = flags.get(0).unwrap();
    let mut a_x_1: Vec<f64> = Vec::new();
    let mut a_x_2: Vec<f64> = Vec::new();
    let mut b_elements: Vec<f64> = Vec::new();
    for flag in &flags[1..] {
        a_x_1.push(2.0 * (flag0.x - flag.x));
        a_x_2.push(2.0 * (flag0.y - flag.y));
        b_elements.push(flag.distance.powi(2) - flag0.distance.powi(2) - flag.x.powi(2) - flag.y.powi(2) + flag0.x.powi(2) + flag0.y.powi(2))
    }
    a_x_1.append(&mut a_x_2);
    let a = MatrixXx2::from_iterator(flags.len() - 1, a_x_1);
    let b = MatrixXx1::from_iterator(flags.len() - 1, b_elements);
    if let Some(c) = (&a.transpose() * &a).try_inverse() {
        let position = c * a.transpose() * b;
        let x = *position.get(0).unwrap();
        let y = *position.get(1).unwrap();
        let mut uxs = 0.0;
        let mut uys = 0.0;
        for flag in flags {
            let direction = angle(x, y, flag.x, flag.y, 0.0).to_radians();
            uxs += direction.cos();
            uys += direction.sin();
        }
        let flags_count = flags.len() as f64;
        let mut body = (uys / flags_count).atan2(uxs / flags_count).to_degrees();
        if body >= 180.0 {
            body -= 360.0;
        } else if body < -180.0 {
            body += 360.0;
        }
        Some(Position { x, y, body })
    } else {
        None
    }
}

fn estimate_by_actions(position: &Position, velc: f64, turn: f64) -> Position {
    let x = position.x + velc * position.body.to_radians().cos();
    let y = position.y + velc * position.body.to_radians().sin();
    let mut body = position.body + turn;
    if body > 180.0 {
        body -= 360.0;
    } else if body < -180.0 {
        body += 180.0;
    }
    Position { x, y, body }
}

impl Position {
    pub fn create(x: f64, y: f64, body: f64) -> Position {
        Position { x, y, body }
    }
    pub fn distance_to(&self, x: f64, y: f64) -> f64 {
        ((self.x - x).powi(2) + (self.y - y).powi(2)).sqrt()
    }
    pub fn direction_to(&self, x: f64, y: f64) -> f64 {
        angle(self.x, self.y, x, y, self.body)
    }
    pub fn position_from(&self, distance: f64, direction: i64) -> Position {
        let x = self.x + distance * (direction as f64).to_radians().cos();
        let y = self.y + distance * (direction as f64).to_radians().sin();
        Position { x, y, body: 0.0 }
    }
    pub fn localize(position: &Position, velc: f64, turn: f64, flags: &Vec<Flag>) -> Position {
        if flags.len() > 2 {
            if let Some(position) = triangulate(flags) {
                position
            } else {
                estimate_by_actions(position, velc, turn)
            }
        } else {
            estimate_by_actions(position, velc, turn)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use super::super::super::server::see::See;

    #[test]
    fn test_localize_by_triangulation() {
        let message = "(see 0 ((f c) 10 0 0 0) ((f r t) 70.8 -29) ((f r b) 70.8 29) ((f g r b) 62.8 6) ((g r) 62.8 0) ((f g r t) 62.8 -6) ((f p r b) 50.4 24) ((f p r c) 46.1 0) ((f p r t) 50.4 -24) ((f r 0) 67.4 0) ((f r t 10) 68 -8) ((f r t 20) 70.1 -17) ((f r t 30) 73.7 -24) ((f r b 10) 68 8) ((f r b 20) 70.1 17) ((f r b 30) 73.7 24) ((b) 10 0 -0 0) ((l r) 62.8 90))";
        let see = See::build(message.to_string());
        let previous = Position { x: -1.0, y: -1.0, body: -1.0 };
        let position = Position::localize(&previous, 0.0, 0.0, &see.flags);
        assert!(position.x > -12.0 && position.x < -8.0);
        assert!(position.y > -1.0 && position.y < 1.0);
        assert_eq!(position.body, 0.0);
    }

    fn test_localize_by_actions() {
        let previous = Position { x: -1.0, y: -1.0, body: -1.0 };
        let position = Position::localize(&previous, 1.0, 10.0, &Vec::<Flag>::new());
        assert_eq!(position.x, 0.0);
        assert_eq!(position.y, 0.0);
        assert_eq!(position.body, 10.0);
    }

}