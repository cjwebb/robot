import subprocess
import sys

ffmpeg_command = 'bash looping.sh'
# https://stackoverflow.com/questions/18421757/live-output-from-subprocess-command
# https://medium.com/coconut-stories/using-ffmpeg-with-docker-94523547f35co
# -movflags
process = subprocess.Popen(ffmpeg_command, shell=True, stdout=subprocess.PIPE)
for data in iter(process.stdout.readline, b''):
    sys.stdout.write(str(data))
    sys.stdout.flush()

