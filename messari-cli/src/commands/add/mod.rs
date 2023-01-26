mod abi;
mod spkg;

use abi::Abi;
use spkg::Spkg;

#[derive(clap::Subcommand)]
pub(crate) enum Add {
    Abi(Abi),
    Spkg(Spkg)
}

impl Add {
    pub(crate) fn execute(&mut self) {
        match self {
            Add::Abi(abi) => abi.execute(),
            Add::Spkg(spkg) => spkg.execute(),
        }
    }
}
