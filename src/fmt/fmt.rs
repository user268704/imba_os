use core::arch::asm;
use core::fmt::{self, Write};
use crate::COM1;

pub struct SerialWriter;


impl Write for SerialWriter {

    fn write_str(&mut self, s: &str) -> fmt::Result {

        for byte in s.bytes() {
            serial_write_byte(byte);
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! serial_print {

    ($($arg:tt)*) => {

        {
            use core::fmt::Write;

            let mut writer = $crate::fmt::fmt::SerialWriter;

            writer
                .write_fmt(format_args!($($arg)*))
                .unwrap();
        }

    };
}

#[macro_export]
macro_rules! serial_println {

    () => {
        $crate::serial_print!("\n");
    };


    ($($arg:tt)*) => {

        $crate::serial_print!(
            "{}\n",
            format_args!($($arg)*)
        );

    };
}

pub fn serial_init() {
    unsafe {
        // Запрещаем UART-прерывания.
        outb(COM1 + 1, 0x00);

        // Включаем DLAB, чтобы задать делитель baud rate.
        outb(COM1 + 3, 0x80);

        // Делитель 3: 115200 / 3 = 38400 baud.
        outb(COM1, 0x03);
        outb(COM1 + 1, 0x00);

        // 8 бит, без parity, один stop bit.
        outb(COM1 + 3, 0x03);

        // Включаем FIFO и очищаем очереди.
        outb(COM1 + 2, 0xC7);

        // Включаем DTR, RTS и OUT2.
        outb(COM1 + 4, 0x0B);
    }
}


pub fn serial_write_str(value: &str) {
    for byte in value.bytes() {
        if byte == b'\n' {
            serial_write_byte(b'\r');
        }

        serial_write_byte(byte);
    }
}

/*
 * Отправка одного байта.
 */
fn serial_write_byte(byte: u8) {
    /*
     * Бит 5 регистра Line Status означает, что передающий
     * буфер готов принять новый байт.
     */
    while (unsafe { inb(COM1 + 5) } & 0x20) == 0 {
        core::hint::spin_loop();
    }

    unsafe {
        outb(COM1, byte);
    }
}

/*
 * Запись байта в x86 I/O port.
 */
#[inline]
unsafe fn outb(port: u16, value: u8) {
    unsafe {
        asm!(
        "out dx, al",
        in("dx") port,
        in("al") value,
        options(nomem, nostack, preserves_flags),
        );
    }
}

/*
 * Чтение байта из x86 I/O port.
 */
#[inline]
unsafe fn inb(port: u16) -> u8 {
    let value: u8;

    unsafe {
        asm!(
        "in al, dx",
        in("dx") port,
        lateout("al") value,
        options(nomem, nostack, preserves_flags),
        );
    }

    value
}
