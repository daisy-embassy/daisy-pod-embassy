#![no_std]

// use same configuration concept as https://github.com/zlosynth/daisy
#[cfg(all(
    feature = "seed_1_2",
    any(feature = "seed_1_1", feature = "seed", feature = "patch_sm")
))]
compile_error!("only a single target board must be selected");

#[cfg(all(
    feature = "seed_1_1",
    any(feature = "seed_1_2", feature = "seed", feature = "patch_sm")
))]
compile_error!("only a single target board must be selected");

#[cfg(all(
    feature = "seed",
    any(feature = "seed_1_2", feature = "seed_1_1", feature = "patch_sm")
))]
compile_error!("only a single target board must be selected");

#[cfg(all(
    feature = "patch_sm",
    any(feature = "seed_1_2", feature = "seed_1_1", feature = "seed")
))]
compile_error!("only a single target board must be selected");

#[cfg(not(any(
    feature = "seed_1_2",
    feature = "seed_1_1",
    feature = "seed",
    feature = "patch_sm"
)))]
compile_error!(
    "target board must be selected using a feature: \"seed_1_2\" | \"seed_1_1\" | \"seed\" | \"patch_sm\""
);

pub use daisy_embassy;

pub mod peri;
