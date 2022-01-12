extern crate sexp;

pub struct PlayerParam {
    pub player_types: i64
}

impl PlayerParam {
    pub fn build(string: String) -> PlayerParam {
        let mut player_types = 0;
        let tree = sexp::parse(&string).unwrap();
        if let sexp::Sexp::List(elements) = tree {
            for element in &elements[1..] {
                if let sexp::Sexp::List(entry) = element {
                    if let sexp::Sexp::Atom(sexp::Atom::S(ref key)) = entry[0] {
                        if key == "player_types" {
                            if let sexp::Sexp::Atom(sexp::Atom::I(value)) = entry[1] {
                                player_types = value;
                            } 
                        }
                    }
                }
            }
        }
        PlayerParam { player_types }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_player_param_build() {
        let message = "(player_param (allow_mult_default_type 0)(catchable_area_l_stretch_max 1.3)(catchable_area_l_stretch_min 1)(dash_power_rate_delta_max 0)(dash_power_rate_delta_min 0)(effort_max_delta_factor -0.004)(effort_min_delta_factor -0.004)(extra_stamina_delta_max 50)(extra_stamina_delta_min 0)(foul_detect_probability_delta_factor 0)(inertia_moment_delta_factor 25)(kick_power_rate_delta_max 0)(kick_power_rate_delta_min 0)(kick_rand_delta_factor 1)(kickable_margin_delta_max 0.1)(kickable_margin_delta_min -0.1)(new_dash_power_rate_delta_max 0.0008)(new_dash_power_rate_delta_min -0.0012)(new_stamina_inc_max_delta_factor -6000)(player_decay_delta_max 0.1)(player_decay_delta_min -0.1)(player_size_delta_factor -100)(player_speed_max_delta_max 0)(player_speed_max_delta_min 0)(player_types 18)(pt_max 1)(random_seed 1578169118)(stamina_inc_max_delta_factor 0)(subs_max 3))";
        let player_param = PlayerParam::build(message.to_string());
        assert_eq!(player_param.player_types, 18);
    }

}