#[derive(Clone)]
pub struct Population {

    pub amount: u32,

}

impl Population {

    pub fn new(amount: u32) -> Self {

        Self {
            amount,
        }
    }

}
