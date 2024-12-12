use image::{open, Rgba, RgbaImage};

// https://en.wikipedia.org/wiki/Canny_edge_detector


fn rgb_to_grayscale(image: &RgbaImage) -> Vec<Vec<u8>> {
    let (width, height) = image.dimensions();
    let mut gray_image = vec![vec![0; width as usize]; height as usize];
    
    for i in 0..height {
        for j in 0..width {
            let pixel = image.get_pixel(j, i).0;
            let r = pixel[0] as u16;
            let g = pixel[1] as u16;
            let b = pixel[2] as u16;
            gray_image[i as usize][j as usize] = (r + g + b) as u8 / 3;
        }
    }

    gray_image
}

fn sobel_operator(image: &Vec<Vec<u8>>) -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
    let sobel_x: [[i8; 3]; 3] = [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]];
    let sobel_y: [[i8; 3]; 3] = [[-1, -2, -1], [0, 0, 0], [1, 2, 1]];

    let height = image.len();
    let width = image[0].len();
    
    let mut grad_x = vec![vec![0.0; width]; height];
    let mut grad_y = vec![vec![0.0; width]; height];
    
    for i in 1..height-1 {
        for j in 1..width-1 {
            let mut gx = 0;
            let mut gy = 0;
            
            for dx in 0..3 {
                for dy in 0..3 {
                    let pixel_value = image[i + dx - 1][j + dy - 1] as i16;
                    gx += pixel_value * sobel_x[dx][dy] as i16;
                    gy += pixel_value * sobel_y[dx][dy] as i16;
                }
            }
            
            grad_x[i][j] = gx as f32;
            grad_y[i][j] = gy as f32;
        }
    }

    (grad_x, grad_y)
}

fn gradient_magnitude_direction(grad_x: &Vec<Vec<f32>>, grad_y: &Vec<Vec<f32>>) -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
    let height = grad_x.len();
    let width = grad_x[0].len();
    
    let mut magnitude = vec![vec![0.0; width]; height];
    let mut direction = vec![vec![0.0; width]; height];
    
    for i in 0..height {
        for j in 0..width {
            magnitude[i][j] = (grad_x[i][j].powi(2) + grad_y[i][j].powi(2)).sqrt();
            direction[i][j] = grad_y[i][j].atan2(grad_x[i][j]);
        }
    }

    (magnitude, direction)
}

fn non_maximum_suppression(magnitude: &Vec<Vec<f32>>, direction: &Vec<Vec<f32>>) -> Vec<Vec<u8>> {
    let height = magnitude.len();
    let width = magnitude[0].len();
    
    let mut suppressed = vec![vec![0; width]; height];
    
    for i in 1..height-1 {
        for j in 1..width-1 {
            let angle = direction[i][j].to_degrees().abs() % 180.0;

            let (p1, p2) = match angle {
                0.0..=22.5 | 157.5..=180.0 => (
                    (i, j + 1),
                    (i, j - 1)
                ),
                22.5..=67.5 => (
                    (i + 1, j + 1),
                    (i - 1, j - 1)
                ),
                67.5..=112.5 => (
                    (i + 1, j),
                    (i - 1, j)
                ),
                _ => (
                    (i + 1, j - 1),
                    (i - 1, j + 1)
                ),
            };

            let p1_value = magnitude[p1.0][p1.1];
            let p2_value = magnitude[p2.0][p2.1];
            
            if magnitude[i][j] >= p1_value && magnitude[i][j] >= p2_value {
                suppressed[i][j] = magnitude[i][j] as u8;
            } else {
                suppressed[i][j] = 0;
            }
        }
    }

    suppressed
}

fn hysteresis_thresholding(image: &Vec<Vec<u8>>, low_threshold: u8, high_threshold: u8) -> Vec<Vec<u8>> {
    let height = image.len();
    let width = image[0].len();
    
    let mut edges = vec![vec![0; width]; height];
    
    for i in 1..height-1 {
        for j in 1..width-1 {
            if image[i][j] >= high_threshold {
                edges[i][j] = 255; 
            } else if image[i][j] >= low_threshold {
                if (i > 0 && j > 0 && image[i - 1][j - 1] == 255) || 
                   (i > 0 && image[i - 1][j] == 255) ||
                   (i > 0 && j < width - 1 && image[i - 1][j + 1] == 255) ||
                   (j > 0 && image[i][j - 1] == 255) ||
                   (j < width - 1 && image[i][j + 1] == 255) ||
                   (i < height - 1 && j > 0 && image[i + 1][j - 1] == 255) ||
                   (i < height - 1 && image[i + 1][j] == 255) ||
                   (i < height - 1 && j < width - 1 && image[i + 1][j + 1] == 255) {
                    edges[i][j] = 255;
                }
            }
        }
    }
    
    edges
}

fn main() {
    let img_path = "media/shrek.jpg";
    let img = open(img_path).expect("bruh");
    
    let rgba_image = img.to_rgba8();

    let gray_image = rgb_to_grayscale(&rgba_image);
    let (grad_x, grad_y) = sobel_operator(&gray_image);
    let (magnitude, direction) = gradient_magnitude_direction(&grad_x, &grad_y);
    let suppressed = non_maximum_suppression(&magnitude, &direction);
    let edges = hysteresis_thresholding(&suppressed, 50, 150);

    let mut output_image = RgbaImage::new(rgba_image.width(), rgba_image.height());
    for i in 0..rgba_image.height() {
        for j in 0..rgba_image.width() {
            let pixel_value = if edges[i as usize][j as usize] == 255 {
                255
            } else {
                0
            };
            output_image.put_pixel(j, i, Rgba([pixel_value, pixel_value, pixel_value, 255]));
        }
    }

    output_image.save("out.png").expect(":sob:");

}
