#!/bin/sh
#
# Demonstrates the difference between RGB and OKLab gradient interpolation
# Creates a side-by-side comparison of red-to-green gradients
#

# Clean up any existing files
rm -f rgb_gradient.png oklab_gradient.png comparison.png rgb_big.png oklab_big.png

echo "Creating RGB gradient (red to lime)..."
cargo run --release -- -f rgb_gradient.png add-gradient \
  --start-color "red" --end-color "lime" --steps 64 --colorspace rgb

echo "Creating OKLab gradient (red to lime)..."
cargo run --release -- -f oklab_gradient.png add-gradient \
  --start-color "red" --end-color "lime" --steps 64 --colorspace oklab

echo "Upscaling gradients to 128x128..."
# Upscale both to 128x128 with point filter (crisp pixels)
gm convert rgb_gradient.png -filter point -resize 128x128 rgb_big.png
gm convert oklab_gradient.png -filter point -resize 128x128 oklab_big.png

echo "Creating side-by-side comparison..."
# Combine side-by-side: RGB on left, OKLab on right
gm convert +append rgb_big.png oklab_big.png comparison.png

echo "Creating animated GIF (flipping between RGB and OKLab)..."
# Create animated GIF that flips between the two (500ms per frame)
gm convert -delay 50 rgb_big.png oklab_big.png -loop 0 gradient_animated.gif

# Cleanup intermediate files
rm rgb_gradient.png oklab_gradient.png rgb_big.png oklab_big.png

echo ""
echo "✓ Created comparison.png (256x128)"
echo "  Left:  RGB gradient (muddy middle)"
echo "  Right: OKLab gradient (perceptually smooth)"
echo ""
echo "✓ Created gradient_animated.gif"
echo "  Flips between RGB and OKLab every 500ms"
echo ""
