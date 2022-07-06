extern crate sexp;

pub struct SenseBody {
    pub game_time: i64,
    pub view_mode: String,
    pub effort: f64,
    pub amount_of_speed: f64,
    pub last_effort: f64,
    pub last_amount_of_speed: f64,
    pub last_dash_power: f64,
    pub last_turn_moment: f64
}

fn sexp_as_float(element: &sexp::Sexp) -> f64 {
    match element {
        sexp::Sexp::Atom(sexp::Atom::F(float)) => *float,
        sexp::Sexp::Atom(sexp::Atom::I(int)) => *int as f64,
        _ => 0.0
    }
}

impl SenseBody {
    pub fn build() -> SenseBody {
        SenseBody {
            game_time: 0,
            view_mode: String::from(""),
            effort: 0.0,
            amount_of_speed: 0.0,
            last_effort: 0.0,
            last_amount_of_speed: 0.0,
            last_dash_power: 0.0,
            last_turn_moment: 0.0
        }
    }
    pub fn update(mut self, string: String) -> Self {
        let tree = sexp::parse(&string).unwrap();
        if let sexp::Sexp::List(elements) = tree {
            if let sexp::Sexp::Atom(sexp::Atom::I(value)) = elements[1] {
                self.game_time = value;
            }
            for element in &elements[2..] {
                if let sexp::Sexp::List(entry) = element {
                    if let sexp::Sexp::Atom(sexp::Atom::S(ref key)) = entry[0] {
                        if key == "view_mode" {
                            if let sexp::Sexp::Atom(sexp::Atom::S(ref value)) = entry[1] {
                                self.view_mode = value.to_string();
                            } 
                        } else if key == "stamina" {
                            self.effort = sexp_as_float(&entry[2]);
                        } else if key == "speed" {
                            self.amount_of_speed = sexp_as_float(&entry[1]);
                        }
                    }
                }
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sense_body_update() {
        let message = String::from("(sense_body 0 (view_mode high normal) (stamina 8000 1 130600) (speed 0 0) (head_angle 0) (kick 0) (dash 0) (turn 0) (say 0) (turn_neck 0) (catch 0) (move 1) (change_view 0) (arm (movable 0) (expires 0) (target 0 0) (count 0)) (focus (target none) (count 0)) (tackle (expires 0) (count 0)) (collision none) (foul  (charged 0) (card none)))");
        let mut sense_body = SenseBody::build();
        sense_body = sense_body.update(message);
        assert_eq!(sense_body.game_time, 0);
        assert_eq!(sense_body.view_mode, "high");
        assert_eq!(sense_body.effort, 1.0);
        assert_eq!(sense_body.amount_of_speed, 0.0);
    }

}