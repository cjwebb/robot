# Robots

## Running the code
Currently, just a Python environment with OpenCV is required:

    conda create -n robot opencv

## Plans (and general thinking)
Recognise different objects, as my robot moves around the house.
Also, need to make it navigate, and build a map of its environment. This could potentially be done via reinforcement learning.

 - Maps would require LIDAR, or at least ultrasound detectors to map out distance travelled, and obstacles.
 - Reinforcement learning to just make it follow you around.

Also need to finish writing the wildlife camera.
How do I get a test harness for this setup, so that I can develop without the camera?

   https://picamera.readthedocs.io/en/release-1.13/recipes1.html#recording-to-a-circular-stream
