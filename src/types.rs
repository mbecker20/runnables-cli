use std::{path::PathBuf};

use derive_variants::EnumVariants;

#[derive(Clone, Debug)]
pub struct Runnable {
    pub name: String,
    pub path: PathBuf,
    pub rtype: RunnableParamsVariant,
}

#[derive(Debug, Clone, EnumVariants)]
#[variant_derive(Debug, Clone, Copy)]
pub enum RunnableParams {
    Rust { release: bool, args: Option<String> },
    Javascript {},
}
