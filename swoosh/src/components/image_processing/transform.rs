// src/transform.rs
use image::{
    imageops::{self, FilterType}, GenericImageView,  ImageError, ImageResult,
};
use std::path::Path;
use tracing::{instrument, info, error};

#[instrument(level = "info", skip_all, fields(input_path = %input_path.display(), output_path = %output_path.display(), width = width, height = height, preserve_aspect_ratio = preserve_aspect_ratio))]
pub fn resize_image(
    input_path: &Path,
    output_path: &Path,
    width: u32,
    height: u32,
    preserve_aspect_ratio: bool,
) -> ImageResult<()> {
    let img = image::open(input_path)?;
    let resized_img = if preserve_aspect_ratio {
        let (w, h) = img.dimensions();
        let ratio = f64::from(w) / f64::from(h);
        let new_width = if width > 0 { width } else { (f64::from(height) * ratio) as u32 };
        let new_height = if height > 0 { height } else { (f64::from(width) / ratio) as u32 };

        img.resize(new_width, new_height, FilterType::Lanczos3)
    } else {
        img.resize_exact(width, height, FilterType::Lanczos3)
    };

    resized_img.save(output_path)?;
    info!(message = "Image resized");
    Ok(())
}


#[instrument(level = "info", skip_all, fields(input_path = %input_path.display(), output_path = %output_path.display()))]
pub fn rotate_image(input_path: &Path, output_path: &Path, degrees: i32) -> ImageResult<()> {
    let img = image::open(input_path)?;

    let rotated_image = match degrees {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => {
            error!(message = "Invalid rotation angle", angle = degrees);
            return Err(ImageError::Parameter(image::error::ParameterError::from_kind(image::error::ParameterErrorKind::Generic(format!("Invalid rotation angle: {}", degrees)))));
        }
    };
    rotated_image.save(output_path)?;
    info!(message = "Image rotated");
    Ok(())
}



#[instrument(level = "info", skip_all, fields(input_path = %input_path.display(), output_path = %output_path.display()))]
pub fn flip_image(input_path: &Path, output_path: &Path, horizontal: bool, vertical: bool) -> ImageResult<()> {
    let img = image::open(input_path)?;

    let flipped_img = if horizontal && vertical {
        img.flipv().fliph() // Chain flips for both directions
    } else if horizontal {
        img.fliph()
    } else if vertical {
        img.flipv()
    } else {
        img
    };

    flipped_img.save(output_path)?;
    info!(message = "Image flipped");
    Ok(())
}


#[instrument(level = "info", skip_all, fields(input_path = %input_path.display(), output_path = %output_path.display(), sigma = sigma))]
pub fn blur_image(input_path: &Path, output_path: &Path, sigma: f32) -> ImageResult<()> {
    let img = image::open(input_path)?;
    let blurred_img = img.blur(sigma);
    blurred_img.save(output_path)?;
    info!(message = "Image blurred");

    Ok(())
}

#[instrument(level = "info", skip_all, fields(input_path = %input_path.display(), output_path = %output_path.display(), x = x, y = y, width = width, height = height))]
pub fn crop_image(
    input_path: &Path,
    output_path: &Path,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> ImageResult<()> {
    let mut img = image::open(input_path)?;
    let cropped_img = imageops::crop(&mut img, x, y, width, height).to_image(); // Extract the cropped image
    cropped_img.save(output_path)?;
    info!(message = "Image cropped");

    Ok(())
}

#[instrument(level = "info", skip_all, fields(input_path = %input_path.display(), output_path = %output_path.display(), sigma = sigma, threshold = threshold))]
pub fn unsharpen_image(
    input_path: &Path,
    output_path: &Path,
    sigma: f32,
    threshold: i32,
) -> ImageResult<()> {
    let img = image::open(input_path)?;
    let unsharpened_img = img.unsharpen(sigma, threshold);
    unsharpened_img.save(output_path)?;
    info!(message = "Image unsharpened");
    Ok(())
}

#[instrument(level = "info", skip_all, fields(input_path = %input_path.display(), output_path = %output_path.display(), value = value))]
pub fn brighten_image(input_path: &Path, output_path: &Path, value: i32) -> ImageResult<()> {
    let img = image::open(input_path)?;
    let brightened_img = img.brighten(value);
    brightened_img.save(output_path)?;
    info!(message = "Image brightened");

    Ok(())
}