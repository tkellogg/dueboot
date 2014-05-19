#![feature(asm)]

#![allow(ctypes)]
#![no_std]

#![crate_id = "blinky#0.1"]

use arduino::{init, delay, pinMode, digitalWrite, analogRead, DigitalValue, Low, High, OUTPUT};

mod arduino;

struct Pins {
	index: u32,
	count: u32,
	base: u32
}

impl Pins {

	fn each(&self, body: |u32|) {
		let mut i = 0;
		while i < self.count {
			body(i);
			i += 1;
		}
	}

	pub fn init_pins(&self) {
		self.each(|i| {
			let pin = self.base + i;
			pinMode(pin, OUTPUT);
		});
	}

	pub fn increment_and_draw(&mut self) {
		self.index = (self.index + 1) % self.count;
		self.each(|i| {
			let pin = self.base + i;
			let setting = if i == self.index { Low } else { High };
			digitalWrite(pin, setting);
		});
	}
}

fn read_potentiometer() -> u32 {
	analogRead(0) as u32
}

#[no_mangle]
pub fn main() {
	init();

	let mut pins = Pins { index: 0, count: 4, base: 10 };
	pins.init_pins();

	loop {
		pins.increment_and_draw();
		let delay_time = read_potentiometer();
		delay(delay_time);
	}
}
