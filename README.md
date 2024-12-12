- Group members: Yotam Dubiner (dubiner2), Brian Wang (brianyw2)
- Project introduction: Our project is a Canny Edge Detection algorithm implementation in Rust. Our goal is to take an image as an input and then create a list of edges, then display the edges in a black and white image.
- Technical overview: Convert the image to grayscale.
Use the Sobel operator to calculate gradients.
Compute gradient magnitude and direction.
Thin the edges using non-maximum suppression.
Use hysteresis thresholding to finalize the edges."