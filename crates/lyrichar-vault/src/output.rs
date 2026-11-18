use crate::nonce::Nonce;

pub type Data = Vec<u8>;

pub struct Output {
    data: Data,
    nonce: Nonce,
}

impl Output {
    pub const fn new(data: Data, nonce: Nonce) -> Self {
        Self { data, nonce }
    }

    pub const fn data(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub const fn nonce(&self) -> &Nonce {
        &self.nonce
    }

    pub fn into_parts(self) -> (Data, Nonce) {
        (self.data, self.nonce)
    }
}
