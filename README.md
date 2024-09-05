# vox_to_gmod

a program to convert [MagicaVoxel](https://ephtracy.github.io/) files to the custom format used by the [common_computer](https://www.gmodstore.com/market/view/common-computer-the-best-office-system) Garry's Mod addon

## Building

`cargo build` will build the exe to ./target/debug/vox_to_gmod.exe

## Usage
```
Usage: vox_to_gmod.exe [OPTIONS] <FILE>

Arguments:
  <FILE>  magicavoxel file to convert

Options:
  -o, --output <OUTPUT>  output path for converted file [default: output.dat]
  -l, --layer <LAYER>    layer id for model/animation [default: 0]
  -a, --animation        treats the .vox file like an animation
  -h, --help             Print help
```
