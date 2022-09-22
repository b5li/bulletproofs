#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "docs", feature(external_doc))]
#![cfg_attr(feature = "docs", deny(missing_docs))]
#![cfg_attr(feature = "docs", doc(include = "../README.md"))]
#![cfg_attr(
    feature = "docs",
    doc(html_root_url = "https://docs.rs/bulletproofs/4.0.0")
)]

extern crate alloc;

#[macro_use]
extern crate serde_derive;

mod util;

#[cfg_attr(feature = "docs", doc(include = "../docs/notes-intro.md"))]
mod notes {
    #[cfg_attr(feature = "docs", doc(include = "../docs/notes-ipp.md"))]
    mod inner_product_proof {}
    #[cfg_attr(feature = "docs", doc(include = "../docs/notes-rp.md"))]
    mod range_proof {}
    #[cfg_attr(feature = "docs", doc(include = "../docs/notes-r1cs.md"))]
    mod r1cs_proof {}
}

// mod approx_range_proof;
mod blinded_ipp;
mod errors;
mod generators;
mod inner_product_proof;
mod zk_inner_product_proof;
mod k_hot_proof;
mod linear_proof;
mod range_proof;
mod secagg_proof;
mod transcript;

pub use crate::blinded_ipp::BlindedInnerProductProof;
pub use crate::errors::ProofError;
pub use crate::generators::{BulletproofGens, BulletproofGensShare, PedersenGens};
pub use crate::inner_product_proof::InnerProductProof;
pub use crate::k_hot_proof::KHotProof;
pub use crate::linear_proof::LinearProof;
pub use crate::range_proof::RangeProof;
pub use crate::secagg_proof::SecAggProof;
pub use crate::zk_inner_product_proof::ZKInnerProductProof;

#[cfg_attr(feature = "docs", doc(include = "../docs/aggregation-api.md"))]
pub mod range_proof_mpc {
    pub use crate::errors::MPCError;
    pub use crate::range_proof::dealer;
    pub use crate::range_proof::messages;
    pub use crate::range_proof::party;
}

#[cfg(feature = "yoloproofs")]
#[cfg(feature = "std")]
pub mod r1cs;
