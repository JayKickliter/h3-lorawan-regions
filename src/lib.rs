pub mod compact{
    include!(concat!(env!("OUT_DIR"), "/compact.rs"));
}

pub mod nocompact {
    include!(concat!(env!("OUT_DIR"), "/nocompact.rs"));
}
