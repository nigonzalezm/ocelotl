extern crate sexp;

pub struct PlayerType {
    pub player_speed_max: f64,
    pub dash_power_rate: f64,
    pub inertia_moment: f64,
    pub kick_power_rate: f64,
    pub kickable_margin: f64
}

impl PlayerType {
    pub fn build(string: String) -> PlayerType {
        let mut player_speed_max = 1.05;
        let mut dash_power_rate = 0.006;
        let mut inertia_moment = 5.0;
        let mut kick_power_rate = 0.027;
        let mut kickable_margin = 0.7;
        let tree = sexp::parse(&string).unwrap();
        if let sexp::Sexp::List(elements) = tree {
            for element in &elements[1..] {
                if let sexp::Sexp::List(entry) = element {
                    if let sexp::Sexp::Atom(sexp::Atom::S(ref key)) = entry[0] {
                        if key == "player_speed_max" {
                            if let sexp::Sexp::Atom(sexp::Atom::F(value)) = entry[1] {
                                player_speed_max = value;
                            }
                            continue;
                        }
                        if key == "dash_power_rate" {
                            if let sexp::Sexp::Atom(sexp::Atom::F(value)) = entry[1] {
                                dash_power_rate = value;
                            }
                            continue;
                        }
                        if key == "inertia_moment" {
                            if let sexp::Sexp::Atom(sexp::Atom::F(value)) = entry[1] {
                                inertia_moment = value;
                            }
                            continue;
                        }
                        if key == "kick_power_rate" {
                            if let sexp::Sexp::Atom(sexp::Atom::F(value)) = entry[1] {
                                kick_power_rate = value;
                            }
                            continue;
                        }
                        if key == "kickable_margin" {
                            if let sexp::Sexp::Atom(sexp::Atom::F(value)) = entry[1] {
                                kickable_margin = value;
                            }
                            continue;
                        }
                    }
                }
            }
        }
        PlayerType { 
            player_speed_max,
            dash_power_rate,
            inertia_moment,
            kick_power_rate,
            kickable_margin
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_player_type_build() {
        let message = "(player_type (id 1)(player_speed_max 1.05)(stamina_inc_max 49.1614)(player_decay 0.375487)(inertia_moment 4.38719)(dash_power_rate 0.00530644)(player_size 0.3)(kickable_margin 0.702801)(kick_rand 0.102801)(extra_stamina 79.2363)(effort_max 0.883055)(effort_min 0.483055)(kick_power_rate 0.027)(foul_detect_probability 0.5)(catchable_area_l_stretch 1.06956))";
        let player_type = PlayerType::build(message.to_string());
        assert_eq!(player_type.player_speed_max, 1.05);
        assert_eq!(player_type.dash_power_rate, 0.00530644);
        assert_eq!(player_type.inertia_moment, 4.38719);
        assert_eq!(player_type.kick_power_rate, 0.027);
        assert_eq!(player_type.kickable_margin, 0.702801);
    }

}