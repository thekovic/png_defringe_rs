/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::env;

use image::{ImageBuffer, Rgba};

fn is_pixel_transparent(pixel: &Rgba<u8>) -> bool
{
    if pixel[3] < 255
    {
        return true;
    }
    else
    {
        return false;
    }
}

fn defringe_to_black(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>)
{
    for pixel in img.pixels_mut()
    {
        if is_pixel_transparent(pixel)
        {
            *pixel = Rgba([0, 0, 0, 0]);
        }
    }
}

fn calculate_pixel_color_average(img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Rgba<u8>
{
    let mut r: u32 = 0;
    let mut g: u32 = 0;
    let mut b: u32 = 0;
    let mut pixel_count: u32 = 0;

    for pixel in img.pixels()
    {
        if !is_pixel_transparent(pixel)
        {
            r += pixel[0] as u32;
            g += pixel[1] as u32;
            b += pixel[2] as u32;
            pixel_count += 1;
        }
    }

    r /= pixel_count;
    g /= pixel_count;
    b /= pixel_count;

    return Rgba([
        r as u8,
        g as u8,
        b as u8,
        0,
    ]);
}

fn defringe_to_average(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>)
{
    let pixel_color_average = calculate_pixel_color_average(img);

    for pixel in img.pixels_mut()
    {
        if is_pixel_transparent(pixel)
        {
            *pixel = pixel_color_average;
        }
    }
}

fn get_neighbouring_pixels(img: &ImageBuffer<Rgba<u8>, Vec<u8>>, x: u32, y: u32) -> Vec<&Rgba<u8>>
{
    let (width, height) = img.dimensions();

    let mut neighbouring_pixels: Vec<&Rgba<u8>> = Vec::with_capacity(8);

    if x != 0
    {
        if y != 0
        {
            neighbouring_pixels.push(img.get_pixel(x - 1, y - 1));
        }

        neighbouring_pixels.push(img.get_pixel(x - 1, y));

        if y != (height - 1)
        {
            neighbouring_pixels.push(img.get_pixel(x - 1, y + 1));
        }
    }
    if x != (width - 1)
    {
        if y != 0
        {
            neighbouring_pixels.push(img.get_pixel(x + 1, y - 1));
        }

        neighbouring_pixels.push(img.get_pixel(x + 1, y));

        if y != (height - 1)
        {
            neighbouring_pixels.push(img.get_pixel(x + 1, y + 1));
        }
    }
    if y != 0
    {
        neighbouring_pixels.push(img.get_pixel(x, y - 1));
    }
    if y != (height - 1)
    {
        neighbouring_pixels.push(img.get_pixel(x, y + 1))
    }

    return neighbouring_pixels;
}

fn defringe_to_interpolation(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>)
{
    let mut original_transparencies = vec![vec![0u8; img.height() as usize]; img.width() as usize];
    for (x, y, pixel) in img.enumerate_pixels()
    {
        original_transparencies[x as usize][y as usize] = pixel[3];
    }

    let mut has_transparent_pixel: bool;

    loop
    {
        has_transparent_pixel = false;
        let previous_img_state = img.clone();
        for (x, y, pixel) in img.enumerate_pixels_mut()
        {
            if is_pixel_transparent(pixel)
            {
                has_transparent_pixel = true;
                let neighbouring_pixels = get_neighbouring_pixels(&previous_img_state, x, y);

                let mut r: u32 = 0;
                let mut g: u32 = 0;
                let mut b: u32 = 0;
                let mut opaque_pixel_count: u32 = 0;
                for neighbouring_pixel in neighbouring_pixels
                {
                    if !is_pixel_transparent(neighbouring_pixel)
                    {
                        r += neighbouring_pixel[0] as u32;
                        g += neighbouring_pixel[1] as u32;
                        b += neighbouring_pixel[2] as u32;
                        opaque_pixel_count += 1;
                    }
                }
                if opaque_pixel_count != 0
                {
                    r /= opaque_pixel_count;
                    g /= opaque_pixel_count;
                    b /= opaque_pixel_count;
                }

                *pixel = Rgba([
                    r as u8,
                    g as u8,
                    b as u8,
                    255, // we are averaging opaque pixels only, so alpha is always 255
                ]);
            }
        }
        if has_transparent_pixel == false
        {
            break;
        }
    }

    for (x, y, pixel) in img.enumerate_pixels_mut()
    {
        pixel[3] = original_transparencies[x as usize][y as usize];
    }
}

fn print_usage()
{
    println!("Usage: png_defringe.exe <action> <input_file> <output_file>");
    println!("---------------------------------------");
    println!("List of actions:");
    println!("\t black - transparent pixels go towards black");
    println!("\t avg - transparent pixels go towards the average of all opaque pixels");
    println!("\t match - transparent pixels are interpolated to match their nearest neighbours");
    panic!("Program failed.");
}

fn main()
{
    let args: Vec<String> = env::args().collect();

    if args.len() != 4
    {
        println!("Wrong number of arguments!");
        print_usage();
    }

    let action = &args[1];
    let input_file = &args[2];
    let output_file = &args[3];

    let img_result = image::open(input_file);
    let img = match img_result
    {
        Ok(file) => file.to_rgba8(),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut out = img.clone();

    match action.as_str()
    {
        "black" => defringe_to_black(&mut out),
        "avg" => defringe_to_average(&mut out),
        "match" => defringe_to_interpolation(&mut out),
        _ => print_usage(),
    }

    out.save(output_file).unwrap();

    println!("Program finished successfully.");
}
