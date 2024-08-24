use std::fs;

use dot_vox::{self, Color};
use clap::Parser;
use gmod_lzma;

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
        let mut output = Vec::new();
        output.push(0);
        output.push(0);
        output.push(model_size.x as u8 - 1);
        output.push(model_size.y as u8 - 1);
        output.push(model_size.z as u8 - 1);

        output.push(colors.len() as u8);
        for color in colors {
            output.push(color.r);
            output.push(color.g);
            output.push(color.b);
        }

        let mut premulti = Vec::new();
        for x in 0..(model_size.x as usize) {
            for y in 0..(model_size.y as usize) {
                for z in 0..(model_size.z as usize) {
                    let c = voxels[x][y][z];
                    premulti.push(c);
                }
            }
        }

        let mut last_color_num: u8 = 0;
        let mut zeros_in_row: u8 = 0;
        let mut colors_in_row: u8 = 0;
        for v in premulti {
            if v == 0 {
                if colors_in_row > 0 {
                    output.push(colors_in_row + 127);
                    output.push(last_color_num);

                    colors_in_row = 0;
                    last_color_num = 0;
                }

                zeros_in_row += 1;

                if zeros_in_row == 255 {
                    output.push(0);
                    output.push(zeros_in_row);
                    zeros_in_row = 0;
                }
            } else {
                if zeros_in_row > 0 {
                    output.push(0);
                    output.push(zeros_in_row);
                    zeros_in_row = 0;
                }

                if last_color_num == 0 {
                    last_color_num = v as u8;
                    colors_in_row = 1;
                } else if last_color_num == v as u8 {
                    colors_in_row += 1;
                    if colors_in_row == 128 {
                        output.push(colors_in_row + 127);
                        output.push(last_color_num);

                        colors_in_row = 0;
                        last_color_num = 0;
                    }
                } else {
                    output.push(colors_in_row + 127);
                    output.push(last_color_num);

                    colors_in_row = 1;
                    last_color_num = v as u8;
                }
            }
        }

        if colors_in_row > 0 {
            output.push(colors_in_row + 127);
            output.push(last_color_num);
        } else if zeros_in_row > 0 {
            output.push(0);
            output.push(zeros_in_row);
        }

        let mut compressed = gmod_lzma::compress(&output[..], 9).expect("failed to compress");
        let mut compressed_data = vec![0, 254];
        compressed_data.append(&mut compressed);

        if output.len() > compressed_data.len() {
            output = compressed_data;
        }

        println!("{:0X?}", output);
        fs::write("output.dat", output).unwrap();
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
