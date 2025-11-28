#!/bin/sh
#
# Demonstrates the difference between RGB and OKLab gradient interpolation
# Creates horizontal strip comparison of red-to-green gradients
#

# Clean up any existing files
rm -f rgb_gradient.png oklab_gradient.png comparison.png rgb_strip.png oklab_strip.png

echo "Creating RGB gradient strip (red to lime)..."
cargo run --release -- -f rgb_gradient.png --min-width 64 add-gradient \
  --start-color "red" --end-color "lime" --steps 64 --colorspace rgb

echo "Creating OKLab gradient strip (red to lime)..."
cargo run --release -- -f oklab_gradient.png --min-width 64 add-gradient \
  --start-color "red" --end-color "lime" --steps 64 --colorspace oklab

echo "Cropping to first row and upscaling to 512x8 strips..."
# Crop to first row (64x1), then upscale to 512x8 (each color becomes 8x8 square)
gm convert rgb_gradient.png -crop 64x1+0+0 +repage -filter point -resize 512x8 rgb_strip.png
gm convert oklab_gradient.png -crop 64x1+0+0 +repage -filter point -resize 512x8 oklab_strip.png

echo "Stacking strips vertically..."
# Stack vertically: RGB on top, OKLab on bottom
gm convert -append rgb_strip.png oklab_strip.png comparison.png

# Cleanup intermediate files
rm rgb_gradient.png oklab_gradient.png rgb_strip.png oklab_strip.png

echo ""
echo "✓ Created comparison.png (512x16)"
echo "  Top:    RGB gradient (muddy middle)"
echo "  Bottom: OKLab gradient (perceptually smooth)"
echo ""
