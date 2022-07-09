use phf::phf_map;
use std::cmp::Ordering;

static FLAGS: phf::Map<&'static str, (f64, f64)> = phf_map! {
    "f t 0"    => (  0.0f64, -39.0f64),
    "f t r 10" => ( 10.0f64, -39.0f64),
    "f t r 20" => ( 20.0f64, -39.0f64),
    "f t r 30" => ( 30.0f64, -39.0f64),
    "f t r 40" => ( 40.0f64, -39.0f64),
    "f t r 50" => ( 50.0f64, -39.0f64),
    "f r t 30" => ( 57.5f64, -30.0f64),
    "f r t 20" => ( 57.5f64, -20.0f64),
    "f r t 10" => ( 57.5f64, -10.0f64),
    "f r 0"    => ( 57.5f64,   0.0f64),
    "f r b 10" => ( 57.5f64,  10.0f64),
    "f r b 20" => ( 57.5f64,  20.0f64),
    "f r b 30" => ( 57.5f64,  30.0f64),
    "f b r 50" => ( 50.0f64,  39.0f64),
    "f b r 40" => ( 40.0f64,  39.0f64),
    "f b r 30" => ( 30.0f64,  39.0f64),
    "f b r 20" => ( 20.0f64,  39.0f64),
    "f b r 10" => ( 10.0f64,  39.0f64),
    "f b 0"    => (  0.0f64,  39.0f64),
    "f b l 10" => (-10.0f64,  39.0f64),
    "f b l 20" => (-20.0f64,  39.0f64),
    "f b l 30" => (-30.0f64,  39.0f64),
    "f b l 40" => (-40.0f64,  39.0f64),
    "f b l 50" => (-50.0f64,  39.0f64),
    "f l b 30" => (-57.5f64,  30.0f64),
    "f l b 20" => (-57.5f64,  20.0f64),
    "f l b 10" => (-57.5f64,  10.0f64),
    "f l 0"    => (-57.5f64,   0.0f64),
    "f l t 10" => (-57.5f64, -10.0f64),
    "f l t 20" => (-57.5f64, -20.0f64),
    "f l t 30" => (-57.5f64, -30.0f64),
    "f t l 50" => (-50.0f64, -39.0f64),
    "f t l 40" => (-40.0f64, -39.0f64),
    "f t l 30" => (-30.0f64, -39.0f64),
    "f t l 20" => (-20.0f64, -39.0f64),
    "f t l 10" => (-10.0f64, -39.0f64),
    "f c"      => (  0.0f64,   0.0f64),
	"f c t"    => (  0.0f64, -34.0f64),
	"f r t"    => ( 52.5f64, -34.0f64),
	"f r b"    => ( 52.5f64,  34.0f64),
	"f c b"    => (  0.0f64,  34.0f64),
	"f l b"    => (-52.5f64,  34.0f64),
	"f l t"    => (-52.5f64, -34.0f64),
	"g l"      => (-52.5f64,   0.0f64),
	"f g l t"  => (-52.5f64,  -7.0f64),
	"f p l t"  => (-36.0f64, -20.0f64),
	"f p l c"  => (-36.0f64,   0.0f64),
	"f p l b"  => (-36.0f64,  20.0f64),
	"f g l b"  => (-52.5f64,   7.0f64),
	"g r"      => ( 52.5f64,   0.0f64),
	"f g r t"  => ( 52.5f64,  -7.0f64),
	"f p r t"  => ( 36.0f64, -20.0f64),
	"f p r c"  => ( 36.0f64,   0.0f64),
	"f p r b"  => ( 36.0f64,  20.0f64),
	"f g r b"  => ( 52.5f64,   7.0f64)
};

pub struct Flag {
    pub x: f64,
    pub y: f64,
    pub distance: f64,
    pub direction: i64
}

#[derive(Debug)]
pub struct BallRaw {
    pub distance: f64,
    pub direction: i64
}

#[derive(Eq)]
pub struct PlayerRaw {
    pub distance: i64,
    pub direction: i64
}

impl Ord for PlayerRaw {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for PlayerRaw {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PlayerRaw {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

pub struct See {
    pub flags: Vec<Flag>,
    pub ball: Option<BallRaw>,
    pub players: Vec<PlayerRaw>
}

fn sexp_as_int(element: &sexp::Sexp) -> i64 {
    match element {
        sexp::Sexp::Atom(sexp::Atom::F(float)) => *float as i64,
        sexp::Sexp::Atom(sexp::Atom::I(int)) => *int,
        _ => 0
    }
}

fn sexp_as_float(element: &sexp::Sexp) -> f64 {
    match element {
        sexp::Sexp::Atom(sexp::Atom::F(float)) => *float,
        sexp::Sexp::Atom(sexp::Atom::I(int)) => *int as f64,
        _ => 0.0
    }
}

impl See {
    pub fn get_flag(flag: String) -> (f64, f64) {
        let (x, y) = FLAGS.get(&flag).unwrap();
        (*x, *y)
    }
    pub fn build(string: String) -> See {
        let mut flags: Vec<Flag> = Vec::new();
        let mut ball: Option<BallRaw> = None;
        let mut players: Vec<PlayerRaw> = Vec::new();
        let tree = sexp::parse(&string).unwrap();
        if let sexp::Sexp::List(elements) = tree {
            for element in &elements[2..] {
                if let sexp::Sexp::List(entry) = element {
                    if let sexp::Sexp::List(object_name) = &entry[0] {
                        if let sexp::Sexp::Atom(sexp::Atom::S(ref object_type)) = object_name[0] {
                            if object_type == "f" && object_name.len() > 1 && entry.len() > 2 {
                                let mut name_vec: Vec<String> = Vec::new();
                                for name_element in object_name {
                                    if let sexp::Sexp::Atom(sexp::Atom::S(ref value)) = name_element {
                                        name_vec.push(value.to_string());
                                    }
                                    if let sexp::Sexp::Atom(sexp::Atom::I(value)) = name_element {
                                        name_vec.push(value.to_string());
                                    }
                                }
                                let distance = sexp_as_float(&entry[1]);
                                let direction = sexp_as_int(&entry[2]);
                                let name = name_vec.join(" ");
                                let (x, y) = FLAGS.get(&name).unwrap();
                                flags.push(Flag { x: *x, y: *y, distance, direction });
                            }
                            if object_type == "b" && entry.len() > 2 {
                                let distance = sexp_as_float(&entry[1]);
                                let direction = sexp_as_int(&entry[2]);
                                ball = Some(BallRaw { distance, direction });
                            }
                            if object_type == "p" && entry.len() > 2 {
                                let distance = sexp_as_int(&entry[1]);
                                let direction = sexp_as_int(&entry[2]);
                                players.push(PlayerRaw { distance, direction });
                            }
                        }
                    }
                }
            }
        }
        See { flags, ball, players }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_flag() {
        let (x, y) = See::get_flag("g r".to_string());
        assert_eq!(52.5, x);
        assert_eq!(0.0, y);
    }

    #[test]
    fn test_see_build() {
        let message = "(see 0 ((f c) 10 0 0 0) ((f r t) 70.8 -29) ((f r b) 70.8 29) ((f g r b) 62.8 6) ((g r) 62.8 0) ((f g r t) 62.8 -6) ((f p r b) 50.4 24) ((f p r c) 46.1 0) ((f p r t) 50.4 -24) ((f r 0) 67.4 0) ((f r t 10) 68 -8) ((f r t 20) 70.1 -17) ((f r t 30) 73.7 -24) ((f r b 10) 68 8) ((f r b 20) 70.1 17) ((f r b 30) 73.7 24) ((b) 10 0 -0 0) ((l r) 62.8 90) ((p \"ocelotl\" 1) 20.1 10 0 0 -155 -155) ((p \"ocelotl\" 2) 24.5 -12 0 -0 149 149))";
        let see = See::build(message.to_string());
        assert_eq!(15, see.flags.len());
        assert_eq!(true, see.ball.is_some());
        assert_eq!(2, see.players.len());
    }

}