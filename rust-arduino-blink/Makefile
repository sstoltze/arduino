setup:
	sudo apt install gcc-avr arduino
flash:
	sudo avrdude -c arduino -v -p atmega328p -P /dev/ttyACM1 -D -U flash:w:target/avr-atmega328p/debug/rust-arduino-blink.elf:e
find-tty:
	dmesg | grep tty
