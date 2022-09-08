#[cfg(feature = "compact")]
pub mod compact {
    include!(concat!(env!("OUT_DIR"), "/compact.rs"));
}

#[cfg(feature = "nocompact")]
pub mod nocompact {
    include!(concat!(env!("OUT_DIR"), "/nocompact.rs"));
}
