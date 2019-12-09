use std::io::{self,BufRead};

fn main() {
    let mut uni = Universe(Vec::new());
    let stin = io::stdin();
    let relations = stin.lock().lines().filter_map(|l| l.ok());
    for rel in relations {
        let names = rel.split(')').collect::<Vec<&str>>();
        uni.ensure_exists(names[0]);
        uni.ensure_orbitee(names[1], names[0]);
    }
    println!("Orbit Sum: {}", uni.orbit_sum());
    println!("Distance YOU to SAN: {}", uni.orbital_distance("YOU","SAN").unwrap() - 2);
}

#[derive(Debug,Clone)]
struct Planet {
    name: String,
    orbitee: Option<String>,
}

#[derive(Debug,Clone)]
struct Universe(Vec<Planet>);

impl Universe {
    fn ensure_exists(&mut self, name: &str) -> &mut Self {
        match self.find(name) {
            None => self.0.push(Planet{name: name.to_string(), orbitee: None}),
            _ => (),
        };
        self
    }

    fn ensure_orbitee(&mut self, name: &str, orbitee: &str) -> &mut Self {
        match self.find_index(name) {
            Some(i) => self.0[i] =
                            Planet{
                                name: name.to_string(),
                                orbitee: Some(orbitee.to_string())
                            },
            None => self.0.push(
                        Planet{
                            name: name.to_string(),
                            orbitee: Some(orbitee.to_string())
                        }
                    ),
        };
        self
    }

    fn orbit_sum(&self) -> usize {
        self.0.iter().map(|p| p.orbit_sum(&self)).sum()
    }

    fn find_index(&self, name: &str) -> Option<usize> {
        self.0.iter().position(|p| p.name == *name)
    }

    fn find(&self, name: &str) -> Option<&Planet> {
        self.0.iter().find(|p| p.name == *name)
    }

    fn orbital_distance(&self, from: &str, to: &str) -> Option<usize> {
        match (self.find(from), self.find(to)) {
            (Some(fp), Some(tp)) => {
                let fos = fp.orbitees(self);
                let tos = tp.orbitees(self);
                let common_orbit = fos.iter()
                    .find(|fon| tos.iter().find(|ton| fon == ton).is_some());
                match common_orbit {
                    Some(on) => Some(fp.orbital_distance(on, self).unwrap() + tp.orbital_distance(on, self).unwrap()),
                    None => None,
                }
            },
            (_, _) => None,
        }
    }
}

impl Planet {
    fn orbitee_planet<'a>(&self, uni: &'a Universe) -> Option<&'a Planet> {
        match self.orbitee {
          Some(ref name) => uni.find(name),
          None => None
        }
    }

    fn orbit_sum(&self, uni: &Universe) -> usize {
        match self.orbitee_planet(uni) {
            Some(p) => 1 + p.orbit_sum(uni),
            None => 0
        }
    }


    fn orbitees(&self, uni: &Universe) -> Vec<String> {
        match self.orbitee_planet(uni) {
            Some(p) => {
                let mut orbs = p.orbitees(uni);
                orbs.insert(0, p.name.to_string());
                orbs
            },
            None => Vec::new(),
        }
    }

    fn orbital_distance(&self, name: &str, uni: &Universe) -> Option<usize> {
        if self.name == name {
            Some(0)
        } else {
            match self.orbitee_planet(uni) {
                Some(p) => {
                    match p.orbital_distance(name, uni) {
                        Some(n) => Some(n + 1),
                        None => None,
                    }
                },
                None => None,
            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_orbit_sum() {
        let mut universe = Universe(Vec::new());
        universe.ensure_exists("A")
                .ensure_orbitee("B", "A")
                .ensure_orbitee("C", "B")
                .ensure_orbitee("D", "B");
        let a = universe.find(&"A").unwrap();
        let c = universe.find(&"C").unwrap();
        assert_eq!(a.orbit_sum(&universe), 0);
        assert_eq!(c.orbit_sum(&universe), 2);
        assert_eq!(universe.orbit_sum(), 5);
    }
}
