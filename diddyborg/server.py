from flask import Flask, Response, render_template
import subprocess

app = Flask(__name__)

@app.route('/')
def index():
    return render_template('remotecontrol.html')

# ffmpeg used inside of docker container
#   -c:v h264 -i - says that we expect h264 from stdin, denoted with '-i -'.
#   -movflags option is specified on the output so that mp4 can be streamed. it fails to encode otherwise.
#   -f mp4 - specifies that we write an mp4 to stdout.
cmd = 'docker run -i --rm jrottenberg/ffmpeg:3.4-alpine -c:v h264 -i - -movflags frag_keyframe+empty_moov -f mp4 -'
p = subprocess.Popen('cat video.h264 | ' + cmd, shell=True, stdin=subprocess.PIPE, stdout=subprocess.PIPE)

@app.route('/video.mp4')
def video_stream():
    def generate():
        for data in iter(p.stdout.readline, b''):
            yield data
    return Response(generate(), mimetype='video/mp4')

if __name__ == '__main__':
    app.run()

