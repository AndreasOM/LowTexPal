#!/bin/sh
#
# Demonstrates OKLab's advantages in extreme/challenging gradient cases:
# - High-saturation colors on opposite sides of hue wheel
# - Very light and very dark vivid tones (where sRGB gamut narrows)
#

echo "Testing extreme gradient cases..."
echo ""

# Clean up
rm -f extreme_*.png comparison_extreme.png

# Case 1: Red to Blue (opposite hues, high saturation)
echo "1. Red → Blue (opposite hues)..."
cargo run --release -- -f extreme_rgb_red_blue.png add-gradient \
  --start-color "red" --end-color "blue" --steps 64 --colorspace rgb

cargo run --release -- -f extreme_oklab_red_blue.png add-gradient \
  --start-color "red" --end-color "blue" --steps 64 --colorspace oklab

# Case 2: Magenta to Green (complementary colors)
echo "2. Magenta → Lime (complementary)..."
cargo run --release -- -f extreme_rgb_mag_green.png add-gradient \
  --start-color "magenta" --end-color "lime" --steps 64 --colorspace rgb

cargo run --release -- -f extreme_oklab_mag_green.png add-gradient \
  --start-color "magenta" --end-color "lime" --steps 64 --colorspace oklab

# Case 3: Yellow to Cyan (complementary)
echo "3. Yellow → Cyan (complementary)..."
cargo run --release -- -f extreme_rgb_yel_cyan.png add-gradient \
  --start-color "yellow" --end-color "cyan" --steps 64 --colorspace rgb

cargo run --release -- -f extreme_oklab_yel_cyan.png add-gradient \
  --start-color "yellow" --end-color "cyan" --steps 64 --colorspace oklab

# Case 4: Very dark to very light (narrow gamut at extremes)
echo "4. DarkRed → Pink (dark to light vivid)..."
cargo run --release -- -f extreme_rgb_dark_light.png add-gradient \
  --start-color "darkred" --end-color "pink" --steps 64 --colorspace rgb

cargo run --release -- -f extreme_oklab_dark_light.png add-gradient \
  --start-color "darkred" --end-color "pink" --steps 64 --colorspace oklab

echo ""
echo "Upscaling all gradients to 256x256..."

# Upscale 8x8 palettes to 256x256 (32x scale, maintaining square aspect ratio)
for file in extreme_*.png; do
    gm convert "$file" -filter point -resize 256x256 "big_$file"
done

echo "Creating comparison image..."

# Stack vertically: RGB on left, OKLab on right for each case
# Format: [Label] RGB | OKLab
gm convert \
  big_extreme_rgb_red_blue.png big_extreme_oklab_red_blue.png +append \
  big_extreme_rgb_mag_green.png big_extreme_oklab_mag_green.png +append \
  big_extreme_rgb_yel_cyan.png big_extreme_oklab_yel_cyan.png +append \
  big_extreme_rgb_dark_light.png big_extreme_oklab_dark_light.png +append \
  -append comparison_extreme.png

echo "Creating side-by-side comparison for all cases..."

# Create horizontal stacked versions of each colorspace
gm convert \
  big_extreme_rgb_red_blue.png \
  big_extreme_rgb_mag_green.png \
  big_extreme_rgb_yel_cyan.png \
  big_extreme_rgb_dark_light.png \
  +append stacked_rgb.png

gm convert \
  big_extreme_oklab_red_blue.png \
  big_extreme_oklab_mag_green.png \
  big_extreme_oklab_yel_cyan.png \
  big_extreme_oklab_dark_light.png \
  +append stacked_oklab.png

# Create side-by-side comparison (RGB on top, OKLab on bottom)
gm convert stacked_rgb.png stacked_oklab.png -append extreme_all_comparison.png

# Cleanup temp files
rm -f stacked_rgb.png stacked_oklab.png
rm -f extreme_rgb_*.png extreme_oklab_*.png big_extreme_*.png

echo ""
echo "✓ Created comparison_extreme.png (512x1024)"
echo "  Each row: RGB (left) | OKLab (right)"
echo ""
echo "✓ Created extreme_all_comparison.png (1024x512)"
echo "  Top row: RGB (all 4 cases side-by-side)"
echo "  Bottom row: OKLab (all 4 cases side-by-side)"
echo ""
