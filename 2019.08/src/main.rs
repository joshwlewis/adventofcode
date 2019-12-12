use std::io::{self, Read};
use std::ops::RangeInclusive;

fn main() {
    let mut imgstr = String::new();
    io::stdin().read_to_string(&mut imgstr).unwrap();

    let image = Image::parse(imgstr, 25, 6);

    let few0prod12 = findleast0sum12(&image);
    println!("{}", few0prod12);

    image.print();
}

fn findleast0sum12(image: &Image) -> usize {
    let mut layers = image.layers();
    layers.sort_by(|la, lb| {
        let a0s = la.iter().filter(|p| p.c == 0).count();
        let b0s = lb.iter().filter(|p| p.c == 0).count();
        a0s.cmp(&b0s)
    });
    let layer = layers.first().unwrap();

    let layer1s = layer.iter().filter(|p| p.c == 1).count();
    let layer2s = layer.iter().filter(|p| p.c == 2).count();

    layer1s * layer2s
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Pixel {
    x: usize,
    y: usize,
    z: usize,
    c: usize,
}

impl Pixel {
    fn get(&self, axis: char) -> usize {
        match axis {
            'x' => self.x,
            'y' => self.y,
            'z' => self.z,
            _ => panic!("only 3 dimensions!"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Image(Vec<Pixel>);

impl Image {
    pub fn parse(s: String, xmax: usize, ymax: usize) -> Self {
        Self(
            s.trim()
                .chars()
                .enumerate()
                .map(|(i, c)| Pixel {
                    x: i % xmax,
                    y: (i / xmax) % ymax,
                    z: (i / (xmax * ymax)) % (xmax * ymax),
                    c: c.to_digit(10).unwrap() as usize,
                })
                .collect(),
        )
    }

    fn range(&self, axis: char) -> RangeInclusive<usize> {
        let ps = self.0.iter().map(|p| p.get(axis));
        let min = ps.clone().min().unwrap();
        let max = ps.max().unwrap();
        min..=max
    }

    fn color(&self, x: usize, y: usize) -> usize {
        let mut pixels: Vec<&Pixel> = self.0.iter().filter(|p| x == p.x && y == p.y).collect();
        pixels.sort_by(|pa, pb| pa.z.cmp(&pb.z));

        let mut color = 2;
        for p in pixels {
            if color == 2 && p.c != 2 {
                color = p.c
            }
        }
        color
    }

    fn layers(&self) -> Vec<Vec<&Pixel>> {
        self.range('z')
            .map(|z| self.0.iter().filter(|p| p.z == z).collect())
            .collect()
    }

    fn print(&self) {
        for y in self.range('y') {
            for x in self.range('x') {
                match self.color(x, y) {
                    0 => print!("⬛️"),
                    1 => print!("⬜️"),
                    _ => print!("🔵"),
                }
            }
            print!("\n");
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_image_parse() {
        let imagestr = "012345678901234567890".to_string();
        let pixels = Image::parse(imagestr, 2, 3).0;
        assert_eq!(
            pixels[0],
            Pixel {
                x: 0,
                y: 0,
                z: 0,
                c: 0
            }
        );
        assert_eq!(
            pixels[1],
            Pixel {
                x: 1,
                y: 0,
                z: 0,
                c: 1
            }
        );
        assert_eq!(
            pixels[3],
            Pixel {
                x: 1,
                y: 1,
                z: 0,
                c: 3
            }
        );
        assert_eq!(
            pixels[10],
            Pixel {
                x: 0,
                y: 2,
                z: 1,
                c: 0
            }
        );
        assert_eq!(
            pixels[19],
            Pixel {
                x: 1,
                y: 0,
                z: 3,
                c: 9
            }
        );
    }
}
