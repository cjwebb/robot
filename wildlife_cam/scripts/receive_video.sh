#!/bin/bash

# For some reason, I can't get this to work with UDP
# TCP works fine though, with only a minimal delay
nc -l 5001 | mplayer -fps 30 -demuxer h264es -
