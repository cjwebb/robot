# mainly taken from
# http://docs.opencv.org/master/d4/d73/tutorial_py_contours_begin.html#gsc.tab=0

import numpy as np
import cv2
import sys

if len(sys.argv) < 2:
    sys.exit("image file name needed as first argument")

im = cv2.imread(sys.argv[1], 0) # the zero loads in grayscale

# find contours
ret, thresh = cv2.threshold(im,127,255,0)
contours, hierarchy = cv2.findContours(thresh,cv2.RETR_TREE,cv2.CHAIN_APPROX_SIMPLE)

shape = im.shape
im = np.zeros(shape, np.uint8)  # create a black base 'image'
cv2.drawContours(im, contours, -1, (128,255,0), 3)

# resize to fit screen
ratio = 1000.0 / im.shape[1]
dimensions = (1000, int(im.shape[0] * ratio))
resized_image = cv2.resize(im, dimensions, interpolation = cv2.INTER_AREA)

# show image
cv2.imshow('image',resized_image)
cv2.waitKey(0)
cv2.destroyAllWindows()
