
# JunoCam Processing Pipeline
Dedicated calibration and processing tools for JunoCam.

## Contributing
Feedback, issues, and contributions are always welcomed. Should enough interest arise in contributing development efforts, I will write up a contribution guide. 

## Citing Mars Raw Utils
Citing this software is not required, but if it has significantly contributed to your research or if you'd like to acknowledge the project in your works, I would be grateful if you did so.  

## Building from source
A working Rust (https://www.rust-lang.org/) installation is required for building.

### Build
`cargo build`

### Install
`cargo install --path .`

### Dependencies
This software requires a working installation of the CSPICE toolkit, which is linked to via [cspice-sys](https://github.com/jacob-pro/cspice-rs/tree/master/cspice-sys). CSPICE requires the `CSPICE_DIR` environment variable to point to the extracted `cspice` directory. **NOTE:**  On Unix like systems you will likely need to rename `lib/cspice.a` to `lib/libcspice.a` so that it can be successfully linked. See the cspice-sys readme for more information on SPICE. 

A working installation of `clang` is also required.

## Runtime Environment
The software runs via `junocam` within the command shell. Various subcommands provide the available tools for processing JunoCam imagery, though all necessary are combined in the `process` subcommand. The environment variables `CSPICE_DIR` and `JUNOBASE` need to be defined as pointing to the `cspice` directory and Juno spice kernels, respectively. 

### SPICE files
Spice files need to be downloaded and pointed to by the `JUNOBASE` environment variable. The script [scripts/update_spice_data.sh](https://github.com/kmgill/junocam_processing/blob/master/scripts/update_spice_data.sh) demonstrates how to set up the spice folders.  

### Calibration and Configuration Files
Calibration files (flats, darks, etc) and the configuration file `config.toml` need to be copied into `~/.junodata` or can be pointed to via an optional `JUNO_DATA` environment variable.

## Docker
The dockerfile demonstrates a method for building and installing the software.

```
docker build -t juno_proc .
docker run --name juno_proc -dit juno_proc
docker exec -it juno_proc bash
```

## Base Usage
```
USAGE:
    junocam [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -v, --verbose    Verbose output

SUBCOMMANDS:
    calibrate        Calibration (dark, flat)
    decompand        Decompand raw image
    help             Print this message or the help of the given subcommand(s)
    hpc              Hot Pixel Correction
    infill           Infill Correction
    process          Process RGB JunoCam image
    triplet-count    Triplet Count
    weights          Infill Correction
```

## Processing
The `process` subcommand provides an end-to-end calibration, blemish repair, & hot pixel correction pipeline.

```
USAGE:
    junocam process [OPTIONS] --input <INPUT> --metadata <METADATA> --output <OUTPUT>

OPTIONS:
    -B, --blue-weight <BLUE_WEIGHT>      Blue weight
    -f, --fov <FOV>                      Fisheye camera field of view, in degrees
    -G, --green-weight <GREEN_WEIGHT>    Green weight
    -h, --help                           Print help information
    -H, --height <HEIGHT>                Output height
    -i, --input <INPUT>                  Input image
    -l, --lens <LENS>                    Camera lens (cylindrical, fisheye)
    -m, --metadata <METADATA>            Input metadata json
    -o, --output <OUTPUT>                Output image
    -p, --predicted                      Use predicted kernels
    -P, --pitch <PITCH>                  Camera pitch, in degrees
    -R, --red-weight <RED_WEIGHT>        Red weight
    -V, --version                        Print version information
    -w, --width <WIDTH>                  Output width
    -y, --yaw <YAW>                      Camera yaw, in degrees
```

### Example
Running the tool to calibrate a Perijove 7 image (https://www.missionjuno.swri.edu/junocam/processing?id=1583), centering on the Great Red Spot, fisheye field of view of 80° and image dimensions of 2048x2048 pixels.

```
junocam -v process -i JNCE_2017192_07C00060_V01-raw.png -m 1583-Metadata.json -o JNCE_2017192_07C00060_V01_processed_1.png -f 80 -P 10 -w 2048 -H 2048
```

## References

Hansen, C. J., et al. "Junocam: Juno’s outreach camera." Space Science Reviews 213.1 (2017): 475-506.
https://doi.org/10.1007/s11214-014-0079-x

Eichstädt, Gerald. "Juno Cam Image Processing" Observatoire de la Côte d'Azur, Nice 2016-05-13 http://www.ajax.ehu.es/Juno_amateur_workshop/talks/06_03_Junocam_processing_Eichstadt.pdf

Caplinger, M., et al. "Juno Software Interface Specification" Malin Space Science Systems, Inc. 2016-08-31 https://pds-imaging.jpl.nasa.gov/data/juno/JNOJNC_0001/DOCUMENT/JUNO_JNC_EDR_RDR_DPSIS.PDF

Semenov, Boris. "Juno JUNOCAM Instrument Kernel." Version 0.3, 2019-08-07 http://naif.jpl.nasa.gov/pub/naif/JUNO/kernels/ik/juno_junocam_v03.ti
