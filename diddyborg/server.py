from flask import Flask, Response, render_template

app = Flask(__name__)

@app.route('/')
def index():
    return render_template('remotecontrol.html')

f = open('video.mp4', 'rb')

@app.route('/video.mp4')
def video_stream():
    def generate():
        for data in iter(f):
            yield data
    return Response(generate(), mimetype='video/mp4')

if __name__ == '__main__':
    app.run()

