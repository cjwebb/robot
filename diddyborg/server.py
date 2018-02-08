from flask import Flask, Response, render_template
import subprocess

app = Flask(__name__)

@app.route('/')
def index():
    return render_template('remotecontrol.html')

cmd = 'docker run -i --rm jrottenberg/ffmpeg:3.4-alpine' \
        ' -c:v h264' \ # input is h264
        ' -i -' \ # input from stdin
        ' -movflags frag_keyframe+empty_moov' \ # allows mp4 to be streamable
        ' -vf scale=640:-1' \ # scales output to 640px in height, and a width to keep the current ratio.
        ' -f mp4 -' # output in mp4 to stdout
p = subprocess.Popen('cat video.h264 | ' + cmd, shell=True, stdin=subprocess.PIPE, stdout=subprocess.PIPE)

@app.route('/video.mp4')
def video_stream():
    def generate():
        for data in iter(p.stdout.readline, b''):
            yield data
    return Response(generate(), mimetype='video/mp4')

if __name__ == '__main__':
    app.run()

