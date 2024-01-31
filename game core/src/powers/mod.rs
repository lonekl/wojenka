/// Powers are countries, it's institutions, factions and corporations.
pub struct Power {

    name: String,

}

impl Power {

    pub fn new(name: String) -> Self {

        Self {
            name,
        }
    }



    pub fn icon(&self) -> (Vec<u8>, usize, usize) {

        match self.name.as_str() {
            "Poland" => {
                let mut image = vec![];

                for _ in 0..5 * 16 * 3 {
                    image.push(255);
                }
                for _ in 0..5 * 16 {
                    image.push(255);
                    image.push(0);
                    image.push(0);
                }

                (image, 16, 10)
            },
            "Ukraine" => {
                let mut image = vec![];

                for _ in 0..5 * 16 {
                    image.push(64);
                    image.push(64);
                    image.push(255);
                }
                for _ in 0..5 * 16 {
                    image.push(255);
                    image.push(255);
                    image.push(64);
                }
                (image, 16, 10)
            },
            "Hungary" => {
                let image = vec![
                    223,  16,  16, 223,  16,  16, 233,  16,  16, 223,  16,  16, 223,  16,  16, 233,  16,  16, 223,  16,  16, 223,  16,  16, 233,  16,  16,
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                    16,  223,  16,  16, 223,  16,  16, 223,  16,  16, 223,  16,  16, 223,  16,  16, 223,  16,  16, 223,  16,  16, 223,  16,  16, 223,  16,
                ];

                (image, 6, 3)
            },
            _ => (vec![0], 1, 1),
        }
    }

}
