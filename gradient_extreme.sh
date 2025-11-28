#!/bin/sh
#
# Demonstrates OKLab's advantages in extreme/challenging gradient cases:
# - High-saturation colors on opposite sides of hue wheel
# - Very light and very dark vivid tones (where sRGB gamut narrows)
# Creates horizontal strips for compact comparison
#

echo "Testing extreme gradient cases..."
echo ""

# Clean up
rm -f extreme_*.png extreme_all_comparison.png

# Case 1: Red to Blue (opposite hues, high saturation)
echo "1. Red → Blue (opposite hues)..."
cargo run --release -- -f extreme_rgb_red_blue.png --min-width 64 add-gradient \
  --start-color "red" --end-color "blue" --steps 64 --colorspace rgb

cargo run --release -- -f extreme_oklab_red_blue.png --min-width 64 add-gradient \
  --start-color "red" --end-color "blue" --steps 64 --colorspace oklab

# Case 2: Magenta to Green (complementary colors)
echo "2. Magenta → Lime (complementary)..."
cargo run --release -- -f extreme_rgb_mag_green.png --min-width 64 add-gradient \
  --start-color "magenta" --end-color "lime" --steps 64 --colorspace rgb

cargo run --release -- -f extreme_oklab_mag_green.png --min-width 64 add-gradient \
  --start-color "magenta" --end-color "lime" --steps 64 --colorspace oklab

# Case 3: Yellow to Cyan (complementary)
echo "3. Yellow → Cyan (complementary)..."
cargo run --release -- -f extreme_rgb_yel_cyan.png --min-width 64 add-gradient \
  --start-color "yellow" --end-color "cyan" --steps 64 --colorspace rgb

cargo run --release -- -f extreme_oklab_yel_cyan.png --min-width 64 add-gradient \
  --start-color "yellow" --end-color "cyan" --steps 64 --colorspace oklab

# Case 4: Very dark to very light (narrow gamut at extremes)
echo "4. DarkRed → Pink (dark to light vivid)..."
cargo run --release -- -f extreme_rgb_dark_light.png --min-width 64 add-gradient \
  --start-color "darkred" --end-color "pink" --steps 64 --colorspace rgb

cargo run --release -- -f extreme_oklab_dark_light.png --min-width 64 add-gradient \
  --start-color "darkred" --end-color "pink" --steps 64 --colorspace oklab

echo ""
echo "Cropping to first row and upscaling to 512x8 strips..."

# Crop to first row (64x1), then upscale to 512x8 horizontal strips (each color becomes 8x8 square)
for file in extreme_*.png; do
    gm convert "$file" -crop 64x1+0+0 +repage -filter point -resize 512x8 "strip_$file"
done

echo "Stacking all strips vertically (RGB/OKLab pairs)..."

# Stack all 8 strips vertically in pairs
gm convert \
  strip_extreme_rgb_red_blue.png \
  strip_extreme_oklab_red_blue.png \
  strip_extreme_rgb_mag_green.png \
  strip_extreme_oklab_mag_green.png \
  strip_extreme_rgb_yel_cyan.png \
  strip_extreme_oklab_yel_cyan.png \
  strip_extreme_rgb_dark_light.png \
  strip_extreme_oklab_dark_light.png \
  -append extreme_all_comparison.png

# Cleanup temp files
rm -f extreme_rgb_*.png extreme_oklab_*.png strip_extreme_*.png

echo ""
echo "✓ Created extreme_all_comparison.png (512x64)"
echo "  8 horizontal strips stacked vertically:"
echo "  - Red→Blue (RGB)"
echo "  - Red→Blue (OKLab)"
echo "  - Magenta→Lime (RGB)"
echo "  - Magenta→Lime (OKLab)"
echo "  - Yellow→Cyan (RGB)"
echo "  - Yellow→Cyan (OKLab)"
echo "  - DarkRed→Pink (RGB)"
echo "  - DarkRed→Pink (OKLab)"
echo ""
