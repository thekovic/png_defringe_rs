# png_defringe_rs

png_defringe_rs is a port of Immorpher's PNG Defringe program written in Rust to achieve easier installation and faster performance. Using PNG Defringe can remove colored fringes from resampled and filtered graphics which have transparency, caused by the filtering picking color information from pixels that are meant to be fully transparent.

## Installation

You don't need to do anything to install png_defringe_rs, all you have to do is grab a release from the Releases page and unzip the archive.

## Usage

    Usage: png_defringe.exe <action> <input_file> <output_file>
    ---------------------------------------
    List of actions:
        black - transparent pixels go towards black
        avg - transparent pixels go towards the average of all opaque pixels
        match - transparent pixels are interpolated to match their nearest neighbours

## List of improvements
- [ ] Convert entire folders at once
- [ ] GUI

## Credits
Original software by Immorpher: https://github.com/Immorpher/PNG-Defringe
