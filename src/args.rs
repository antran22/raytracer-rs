use clap::{Parser, ValueEnum};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[arg(short, long, value_enum, default_value = "fast")]
    pub mode: Mode,

    /// Number of times to greet
    #[arg(short, long, value_enum, default_value = "complex")]
    pub scene: Scene,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Mode {
    Fast,
    Slow,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Scene {
    Complex,
    CheckeredSphere,
    Earth,
    Perlin,
    Quads,
    SimpleLight,
}
