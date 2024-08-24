use dot_vox::{self, Color};
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    /// vox file to convert
    file: String,
}

fn main() {
    let args = Args::parse();

    if let Ok(vox_file) = dot_vox::load(args.file.as_str()) {
        if vox_file.models.len() == 0 {
            panic!("need at least 1 model");
        }

        // magicavoxel palette
        let palette = vox_file.palette;

        // gmod color lookup table
        let mut colors: Vec<Color> = Vec::new();

        let model = &vox_file.models[0];
        let model_size = model.size;

        let voxel_data = &model.voxels;
        let mut voxels: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; model_size.z as usize]; model_size.y as usize]; model_size.x as usize];

        for voxel in voxel_data {
            // add any new colors to the lookup table
            if !colors.contains(&palette[voxel.i as usize]) {
                colors.push(palette[voxel.i as usize]);
            }
            // store the lookup table position for each voxel, offset by 1 for lua
            voxels[voxel.x as usize][voxel.y as usize][voxel.z as usize] = get_color_index(&colors, &palette, voxel.i as usize) + 1;
        }

        // TODO: transplant ExportData3 function from lua file "cl_ccvox.lua" in the libs folder
    }
}

/// returns the lookup table index for a given palette index
fn get_color_index(colors: &Vec<Color>, palette: &Vec<Color>, index: usize) -> usize {
    for i in 0..colors.len() {
        if colors[i] == palette[index] {
            return i;
        }
    }

    // default to first color in table
    return 0;
}
