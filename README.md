# D7018E: SPecial Studies &emdash; lpc1347 hardware access and drivers
A support crate for the LPC1347 breakout board.

Most tasks, both simple and complex, rely on common hardware to be available to the programmer. For new users to be able to use said hardware, drivers and hardware-support need to be available from the get-go, which is currently not the case for rust on the LPC1347. The idea is to provide a (perhaps multiple) crate(s) that provide support for these things in rust, partially by using the specifications provided by the manufacturer and libraries written in C. 

## Grade 3
Provide GPIO-, PWM and RTFM support for the LPC1347.

## Grade 4
Design a framework for developing further drivers in rust.

## Grade 5
Implement the framework and a driver for ADC, USB or I2C using said framework.
