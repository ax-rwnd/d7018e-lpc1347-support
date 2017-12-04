# Project: lpc1347 hardware access and drivers
A support crate for the LPC1347 breakout board.

Most tasks, both simple and complex, rely on common hardware to be available to the programmer. For new users to be able to use said hardware, drivers and hardware-support need to be available from the get-go, which is currently not the case for rust on the LPC1347. The idea is to provide a (perhaps multiple) crate(s) that provide support for these things in rust, partially by using the specifications provided by the manufacturer and libraries written in C. 

## Grade 3
Provide GPIO-, RTFM and PWM support for the LPC1347.

## Grade 4
Provide USB support?

## Grade 5
