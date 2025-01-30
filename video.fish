
#!/usr/bin/env fish

# Check for correct arguments
if test (count $argv) -ne 3
    echo "Usage: "(status filename)" input_video input_image output_video"
    exit 1
end

set video $argv[1]
set image $argv[2]
set output $argv[3]

# Get video metadata
set vwidth (ffprobe -v error -select_streams v:0 -show_entries stream=width -of csv=p=0 "$video")
set vheight (ffprobe -v error -select_streams v:0 -show_entries stream=height -of csv=p=0 "$video")
set duration (ffprobe -v error -show_entries format=duration -of default=noprint_wrappers=1:nokey=1 "$video")
set framerate (ffprobe -v error -select_streams v:0 -show_entries stream=r_frame_rate -of csv=p=0 "$video" | cut -d '/' -f1)

# Get image dimensions
set iwidth (identify -format "%w" "$image")
set iheight (identify -format "%h" "$image")

# Prepare background image (ensure exact size)
convert "$image" \
    -resize "$iwidth"x"$iheight"^ \
    -gravity center \
    -extent "$iwidth"x"$iheight" \
    "temp_bg.jpg"

# Determine scaling needs
if test $vwidth -le $iwidth -a $vheight -le $iheight
    set vfilter "[1:v] null [vid]"
else
    set vfilter "[1:v] scale=w=$iwidth:h=$iheight:force_original_aspect_ratio=decrease [vid]"
end

# Create final video
ffmpeg -y \
    -loop 1 \
    -framerate $framerate \
    -t $duration \
    -i "temp_bg.jpg" \
    -i "$video" \
    -filter_complex "$vfilter; [0:v][vid] overlay=(main_w-overlay_w)/2:(main_h-overlay_h)/2:shortest=1" \
    -c:v libx264 \
    -pix_fmt yuv420p \
    "$output"

# Cleanup
rm -f temp_bg.jpg

echo "Video created: $output"
