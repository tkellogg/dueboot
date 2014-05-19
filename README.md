dueboot
=======

Infrastructure for programming an [Arduino Due][1] in the [Rust programming language][2]. This includes a sample sketch that can be 
modified to do most any other sketch.

Based on https://github.com/neykov/armboot

Compiling
---------

Install the prerequisites:
* Download [an Arduino Due installation][1]
* `brew install rust`
* `brew install llvm --all-targets`

Then modify `Rakefile`:
1. `ARDUINO` points to the directory in the Arduino Due installation that has tools to flash the device. Should contain `arm-none-eabi-ar` and `arm-none-eabi-objcopy` among other executables.
2. `RUSTC` points to the `rustc` executable (the Rust compiler).
3. `LLC` points to the LLVM compiler executable called `llc`.
4. `PORT` is the name of the USB connection. Do an `ls /dev/tty*` to get a list of options. Most likely something like `tty.usbmodem1415. RUST_SRC is the name of the *.rs file that contains the `main` function for your sketch.

Then run "rake burn" to upload to the Arduino.

Here is a circuit diagram of the Arduino sketch:

![Circuit diagram](https://raw.githubusercontent.com/tkellogg/dueboot/master/rust-arduino-circuit.png)


Structure
---------

    core.rs - sample program (blinks 4 LEDs in series like Christmas lights with a potentiometer to control the speed).
              Feel free to change the program & circuit as you wish.
    arduino.rs - extern stubs for the core Arduino libraries
    hardware/ - from a random Arduino IDE for OS X


Credits
-------

  - armboot: https://github.com/neykov/armboot
  - zero.rs: https://github.com/pcwalton/zero.rs

 [1]: http://arduino.cc/en/Guide/ArduinoDue
 [2]: http://www.rust-lang.org/

