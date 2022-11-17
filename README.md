# png_defringe_rs

Using PNG Defringe can remove colored fringes from resampled and filtered graphics which have transparency, caused by the filtering picking color information from pixels that are meant to be fully transparent.

## Installation

You don't need to do anything to install png_defringe_rs, all you have to do is grab a release from the Releases page and unzip the archive.

## Usage

    Usage: png_defringe.exe <action> <input_file> <output_file>
    ---------------------------------------
    List of actions:
        black - transparent pixels go towards black
        avg - transparent pixels go towards the average of all opaque pixels
        match - transparent pixels are interpolated to match their nearest neighbours
