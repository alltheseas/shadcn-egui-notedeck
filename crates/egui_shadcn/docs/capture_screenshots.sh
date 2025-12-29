#!/bin/bash
# Component Screenshot & GIF Capture Script
# Automates capturing screenshots and GIFs from the egui_shadcn showcase

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOCS_DIR="$SCRIPT_DIR"
SCREENSHOTS_DIR="$DOCS_DIR/assets/screenshots"
GIFS_DIR="$DOCS_DIR/assets/gifs"
CRATE_DIR="$(dirname "$DOCS_DIR")"

# Ensure directories exist
mkdir -p "$SCREENSHOTS_DIR/light" "$SCREENSHOTS_DIR/dark" "$GIFS_DIR"

# Components to capture (matching showcase sections)
COMPONENTS=(
    "button"
    "badge"
    "avatar"
    "card"
    "alert"
    "skeleton"
    "kbd"
    "separator"
    "input"
    "textarea"
    "checkbox"
    "switch"
    "slider"
    "progress"
    "radio"
    "select"
    "combobox"
    "toggle"
    "tabs"
    "sidebar"
    "menubar"
    "breadcrumb"
    "pagination"
    "dialog"
    "drawer"
    "popover"
    "tooltip"
    "toast"
    "table"
    "calendar"
    "datepicker"
    "carousel"
    "chart"
    "accordion"
    "spinner"
)

# Animated components that need GIF capture
ANIMATED_COMPONENTS=(
    "spinner"
    "skeleton"
    "progress"
)

echo "=== egui_shadcn Screenshot Capture ==="
echo "Screenshots dir: $SCREENSHOTS_DIR"
echo "GIFs dir: $GIFS_DIR"

# Function to find showcase window
find_showcase_window() {
    xdotool search --name "egui_shadcn showcase" | head -1
}

# Function to capture screenshot of window
capture_screenshot() {
    local window_id=$1
    local output_file=$2

    # Use import (ImageMagick) to capture specific window
    import -window "$window_id" "$output_file"
    echo "Captured: $output_file"
}

# Function to capture GIF of window (3 seconds)
capture_gif() {
    local window_id=$1
    local output_file=$2
    local duration=${3:-3}

    # Get window geometry
    local geometry=$(xwininfo -id "$window_id" | grep -E "geometry" | awk '{print $2}')
    local position=$(xwininfo -id "$window_id" | grep -E "Absolute|Width|Height" | awk '{print $NF}' | tr '\n' ' ')

    # Get window position and size
    local abs_x=$(xwininfo -id "$window_id" | grep "Absolute upper-left X" | awk '{print $NF}')
    local abs_y=$(xwininfo -id "$window_id" | grep "Absolute upper-left Y" | awk '{print $NF}')
    local width=$(xwininfo -id "$window_id" | grep "Width:" | awk '{print $NF}')
    local height=$(xwininfo -id "$window_id" | grep "Height:" | awk '{print $NF}')

    # Capture video with ffmpeg, then convert to GIF
    local temp_video="/tmp/capture_$$.mp4"

    echo "Capturing ${duration}s GIF at ${width}x${height}+${abs_x}+${abs_y}..."

    ffmpeg -y -f x11grab -video_size "${width}x${height}" -framerate 15 \
        -i ":0.0+${abs_x},${abs_y}" -t "$duration" \
        -vf "fps=10,scale=${width}:-1:flags=lanczos" \
        -c:v libx264 -preset ultrafast "$temp_video" 2>/dev/null

    # Convert to GIF with good quality
    ffmpeg -y -i "$temp_video" \
        -vf "fps=10,scale=${width}:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse" \
        "$output_file" 2>/dev/null

    rm -f "$temp_video"
    echo "Captured GIF: $output_file"
}

# Main capture function
capture_all() {
    local mode=$1  # "light" or "dark"

    echo ""
    echo "=== Capturing $mode mode screenshots ==="

    # Find showcase window
    local window_id=$(find_showcase_window)
    if [ -z "$window_id" ]; then
        echo "ERROR: Showcase window not found. Please start the showcase first:"
        echo "  cargo run -p egui_shadcn --example showcase"
        exit 1
    fi

    echo "Found showcase window: $window_id"

    # Focus the window
    xdotool windowactivate "$window_id"
    sleep 0.5

    # Capture full window screenshot
    capture_screenshot "$window_id" "$SCREENSHOTS_DIR/$mode/showcase_full.png"

    # For animated components, capture GIFs
    for component in "${ANIMATED_COMPONENTS[@]}"; do
        if [ "$mode" = "light" ]; then
            capture_gif "$window_id" "$GIFS_DIR/${component}_${mode}.gif" 3
        fi
    done

    echo "=== $mode mode capture complete ==="
}

# Check if showcase is running
check_showcase() {
    if ! find_showcase_window > /dev/null 2>&1; then
        echo "Showcase not running. Starting it..."
        cd "$CRATE_DIR/.."
        cargo run -p egui_shadcn --example showcase &
        sleep 5
    fi
}

# Print usage
usage() {
    echo "Usage: $0 [command]"
    echo ""
    echo "Commands:"
    echo "  light     Capture light mode screenshots"
    echo "  dark      Capture dark mode screenshots"
    echo "  all       Capture both modes"
    echo "  gif       Capture GIFs of animated components"
    echo "  help      Show this help"
    echo ""
    echo "Note: Start the showcase first, then toggle light/dark mode manually"
    echo "before running the corresponding capture command."
}

# Main
case "${1:-all}" in
    light)
        check_showcase
        capture_all "light"
        ;;
    dark)
        check_showcase
        capture_all "dark"
        ;;
    all)
        check_showcase
        echo "Capture light mode first, then toggle to dark mode in the app"
        capture_all "light"
        echo ""
        echo "Now toggle to DARK mode in the showcase app, then press Enter..."
        read -r
        capture_all "dark"
        ;;
    gif)
        check_showcase
        window_id=$(find_showcase_window)
        for component in "${ANIMATED_COMPONENTS[@]}"; do
            capture_gif "$window_id" "$GIFS_DIR/${component}.gif" 3
        done
        ;;
    help|--help|-h)
        usage
        ;;
    *)
        echo "Unknown command: $1"
        usage
        exit 1
        ;;
esac

echo ""
echo "Done! Screenshots saved to: $SCREENSHOTS_DIR"
echo "GIFs saved to: $GIFS_DIR"
