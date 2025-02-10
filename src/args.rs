use clap::{Parser, ValueEnum};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'd', long, default_value = "50")]
    pub depth: u32,

    #[arg(short = 'y', long, default_value = "600")]
    pub image_width: u32,

    #[arg(short = 'x', long, default_value = "400")]
    pub image_height: u32,

    #[arg(short = 's', long, default_value = "50")]
    pub samples_per_pixel: u32,

    #[arg(short = 'c', long, value_enum, default_value = "complex")]
    pub scene: Scene,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Scene {
    Complex,
    CheckeredSphere,
    Earth,
    Perlin,
    Quads,
    SimpleLight,
    CornellBox,
}
