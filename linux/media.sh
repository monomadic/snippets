# youtube-dl

youtube-dl --format "best[ext=mp4][height<=720]" 8C9umQ74JSc

# download specific audio + video (video goes first)
youtube-dl -f 136+140 8C9umQ74JSc

# note that 136 is usually h264 (otherwise could be hevc)
