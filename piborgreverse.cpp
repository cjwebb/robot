#include <iostream>
#include <stdio.h>
#include <linux/i2c-dev.h>
#include <sys/ioctl.h>
#include <fcntl.h> // for O_RDWR

int main() {
  const char * devName = "/dev/i2c-1";
  char buf[4];

  int file = open(devName, O_RDWR);
  if (file == -1) {
    perror(devName);
    return 1;
  }

  if (ioctl(file, I2C_SLAVE, 0x44) < 0) {
    perror("Failed to acquire bus access and/or talk to slave");
    return 1;
  }

  // motor1 is the right side (0 to 255 power values)
  // motor2 is the left side (0 to 255 power values)

  buf[0] = 6;
  buf[1] = 0;
  int res = write(file, buf, 2);

  std::cout << res << '\n';
}
