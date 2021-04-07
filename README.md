# sam4_xplained

Embedded Rust support crate for several Microchip/Atmel XPlained development boards.

Currently the following boards are supported:
* SAM4E_XPlained_Pro
* SAM4S_XPlained_Pro

## Running the examples using OpenOCD
1) Ensure your development board is connect.
2) Change to the directory that corresponds to your development board.
3) Start OpenOCD in a separate terminal window - This should show a valid connection to the board.   This will be the terminal
   where the semihosting output is displayed.
4) In the original terminal window, execute the following to flash and load an example.
```shell
$ # The following command will run the 'blinky' example.
$ cargo re blinky
```

### Helpful links:
https://dev.to/rubberduck/debugging-rust-arm-cortexm-programs-with-visual-studio-code-336h#setting-up-visual-studio-code

