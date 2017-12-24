# Wildlife Camera

Written in Rust, cross-compiled from MacOS to a RaspberryPi Zero W.
Uses an docker image with an ARM toolchain installed properly in order to cross-compile. This was easier than installing all dependencies aand work-arounds on MacOS.

Once `docker-build-arm.sh` is run, the RaspberryPi build will be in:

    target/arm-unknown-linux-gnueabihf/release

This can be moved to the Raspberry Pi using SCP (for example):

    scp wildlife_cam pi@192.168.1.100:/home/pi


## Running

This project is compiled as a binary, and uses a default configuration file to specify config. The defaults can be overridden via command line arguments. For example:

    APP_COMMAND_PROGRAM=raspivid APP_COMMAND_ARGUMENTS="-t 0 -o -" ./wildlife_cam
