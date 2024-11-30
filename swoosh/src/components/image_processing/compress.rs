// src/compress.rs
use image::{
    codecs::{
        avif::AvifEncoder,
        gif::GifEncoder,
        jpeg::JpegEncoder,
        openexr::OpenExrEncoder,
        png::PngEncoder,
        webp::WebPEncoder,
    },
    ImageEncoder,
    ImageError,
    ImageFormat,
    ImageResult,
};
use std::path::Path;
use image::error;
use tracing:: info; // For logging

pub fn compress_image(
    input_path: &Path,
    output_path: &Path,
    format: ImageFormat,
    quality: u8
) -> ImageResult<()> {
    let raw = image::open(input_path)?; // Chain into_rgba8()
    let img = raw.clone().into_rgba8();
    let mut output = std::fs::File::create(output_path)?; // Create output file only once

    match format {
        ImageFormat::Jpeg => {
            JpegEncoder::new_with_quality(&mut output, quality).write_image(
                raw.into_rgb8().as_raw(),
                img.width(),
                img.height(),
                image::ExtendedColorType::Rgb8
            )?;
        }

        ImageFormat::Avif => {
            AvifEncoder::new(&mut output).write_image(
                img.as_raw(),
                img.width(),
                img.height(),
                image::ExtendedColorType::Rgba8
            )?;
        }

        ImageFormat::Png => {
            PngEncoder::new_with_quality(
                &mut output,
                image::codecs::png::CompressionType::Best,
                image::codecs::png::FilterType::Adaptive
            ).write_image(
                img.as_raw(),
                img.width(),
                img.height(),
                image::ExtendedColorType::Rgba8
            )?;
        }
        ImageFormat::WebP => {
            WebPEncoder::new_lossless(&mut output).encode(
                img.as_raw(),
                img.width(),
                img.height(),
                image::ExtendedColorType::Rgba8
            )?;
        }
        ImageFormat::Gif => {
            GifEncoder::new_with_speed(&mut output, (quality/10*3).into()).encode(
                img.as_raw(),
                img.width(),
                img.height(),
                image::ExtendedColorType::Rgba8
            )?;
        }
        ImageFormat::OpenExr => {
            OpenExrEncoder::new(&mut output).write_image(
                img.as_raw(),
                img.width(),
                img.height(),
                image::ExtendedColorType::Rgba32F
            )?;
        }
        _ => {
            return Err(ImageError::Unsupported(error::ImageFormatHint::Exact(format).into()));
        } // Simplified unsupported format handling
    }

    info!("Compressed {:?} to {:?}: {:?}", input_path, format, output_path);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_compress_jpeg() {
        let temp_dir = tempdir().unwrap();
        let input_path = PathBuf::from("tests/images/test.png"); // Replace with a real test image
        let output_path = temp_dir.path().join("test_image.jpeg");
        let result = compress_image(&input_path, &output_path, ImageFormat::Jpeg, 90);
        assert!(result.is_ok());
        assert!(output_path.exists());


        // Clean up
        temp_dir.close().unwrap();
    }

    #[traced_test]
    #[test]
    fn test_compress_png() {
        // Similar structure as the JPEG test
        let temp_dir = tempdir().unwrap();
        let input_path = PathBuf::from("tests/images/test.jpg"); // Use a different test image
        let output_path = temp_dir.path().join("test_image.png");
        let result = compress_image(&input_path, &output_path, ImageFormat::Png, 90);

        assert!(result.is_ok());
        assert!(output_path.exists());
        temp_dir.close().unwrap();
    }

    #[traced_test]
    #[test]
    fn test_compress_webp() {
        let temp_dir = tempdir().unwrap();
        let input_path = PathBuf::from("tests/images/test.png"); // Use a different test image
        let output_path = temp_dir.path().join("test_image.webp");
        let result = compress_image(&input_path, &output_path, ImageFormat::WebP, 90);
        assert!(result.is_ok());
        assert!(output_path.exists());

        temp_dir.close().unwrap();
    }

    #[traced_test]
    #[test]
    fn test_compress_gif() {
        let temp_dir = tempdir().unwrap();
        let input_path = PathBuf::from("tests/images/test.png"); // Use a different test image
        let output_path = temp_dir.path().join("test_image.gif");
        let result = compress_image(&input_path, &output_path, ImageFormat::Gif, 90);
        assert!(result.is_ok());
        assert!(output_path.exists());
        temp_dir.close().unwrap();
    }

    // ... Add similar tests for other formats (GIF, OpenEXR, etc.)

    #[traced_test]
    #[test]
    fn test_compress_unsupported_format() {
        let temp_dir = tempdir().unwrap();
        let input_path = PathBuf::from("tests/images/test.xrp");
        let output_path = temp_dir.path().join("test_image.invalid");
        let result = compress_image(&input_path, &output_path, ImageFormat::Ico, 90); // Use an unsupported format
        assert!(result.is_err());
        assert!(!output_path.exists()); // The output file should not be created

        temp_dir.close().unwrap();
    }

    #[traced_test]
    #[test]
    fn test_compress_invalid_input_path() {
        let temp_dir = tempdir().unwrap();
        let input_path = PathBuf::from("nonexistent_image.png"); // Non-existent file
        let output_path = temp_dir.path().join("test_image.jpg");
        let result = compress_image(&input_path, &output_path, ImageFormat::Jpeg, 90);
        assert!(result.is_err());
        assert!(!output_path.exists());

        temp_dir.close().unwrap();
    }
}

