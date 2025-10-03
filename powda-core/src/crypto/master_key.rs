use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Zeroize, ZeroizeOnDrop)]
pub(crate) struct MasterKey {
    pub(crate) key: Vec<u8>,
}