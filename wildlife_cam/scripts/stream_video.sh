#!/bin/bash

# This should be run on a RaspberryPi
raspivid -vf -ih -fps 30 -t 0 -w 800 -h 800 -o - | nc -v 192.168.1.95 5001
