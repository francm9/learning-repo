use crate::spi::spi::SPI;
use arduino_hal::{hal::port::{PB0, PB1}, port::{mode::Output, Pin}};
use arduino_hal::delay_ms;

pub struct ST7735S {
    spi: SPI,
    dc: Pin<Output, PB1>,
    rst: Pin<Output, PB0>,
}

// Comandos del ST7735
pub const SWRESET: u8 = 0x01;    // Software reset
pub const SLPOUT: u8 = 0x11;     // Sleep out
pub const NORON: u8 = 0x13;      // Normal display mode on
pub const INVOFF: u8 = 0x20;     // Display inversion off
pub const DISPON: u8 = 0x29;     // Display on
pub const CASET: u8 = 0x2A;      // Column address set
pub const RASET: u8 = 0x2B;      // Row address set
pub const RAMWR: u8 = 0x2C;      // Memory write
pub const MADCTL: u8 = 0x36;     // Memory data access control
pub const COLMOD: u8 = 0x3A;     // Interface pixel format

// Comandos adicionales para la inicialización
pub const FRMCTR1: u8 = 0xB1;    // Frame rate control (In normal mode/ Full colors)
pub const FRMCTR2: u8 = 0xB2;    // Frame rate control (In idle mode/ 8-colors)
pub const FRMCTR3: u8 = 0xB3;    // Frame rate control (In partial mode/ full colors)
pub const INVCTR: u8 = 0xB4;     // Display inversion control
pub const PWCTR1: u8 = 0xC0;     // Power control 1
pub const PWCTR2: u8 = 0xC1;     // Power control 2
pub const PWCTR3: u8 = 0xC2;     // Power control 3
pub const PWCTR4: u8 = 0xC3;     // Power control 4
pub const PWCTR5: u8 = 0xC4;     // Power control 5
pub const VMCTR1: u8 = 0xC5;     // VCOM control 1
pub const GMCTRP1: u8 = 0xE0;    // Gamma correction (positive polarity)
pub const GMCTRN1: u8 = 0xE1;    // Gamma correction (negative polarity)

const WIDTH: u16 = 128;
const HEIGHT: u16 = 160;

const FONT: [[u8; 5]; 95] = [
    [0x00, 0x00, 0x00, 0x00, 0x00], // (espacio)
    [0x00, 0x00, 0x5F, 0x00, 0x00], // !
    [0x00, 0x07, 0x00, 0x07, 0x00], // "
    [0x14, 0x7F, 0x14, 0x7F, 0x14], // #
    [0x24, 0x2A, 0x7F, 0x2A, 0x12], // $
    [0x23, 0x13, 0x08, 0x64, 0x62], // %
    [0x36, 0x49, 0x55, 0x22, 0x50], // &
    [0x00, 0x05, 0x03, 0x00, 0x00], // '
    [0x00, 0x1C, 0x22, 0x41, 0x00], // (
    [0x00, 0x41, 0x22, 0x1C, 0x00], // )
    [0x14, 0x08, 0x3E, 0x08, 0x14], // *
    [0x08, 0x08, 0x3E, 0x08, 0x08], // +
    [0x00, 0x50, 0x30, 0x00, 0x00], // ,
    [0x08, 0x08, 0x08, 0x08, 0x08], // -
    [0x00, 0x60, 0x60, 0x00, 0x00], // .
    [0x20, 0x10, 0x08, 0x04, 0x02], // /
    [0x3E, 0x51, 0x49, 0x45, 0x3E], // 0
    [0x00, 0x42, 0x7F, 0x40, 0x00], // 1
    [0x42, 0x61, 0x51, 0x49, 0x46], // 2
    [0x21, 0x41, 0x45, 0x4B, 0x31], // 3
    [0x18, 0x14, 0x12, 0x7F, 0x10], // 4
    [0x27, 0x45, 0x45, 0x45, 0x39], // 5
    [0x3C, 0x4A, 0x49, 0x49, 0x30], // 6
    [0x01, 0x71, 0x09, 0x05, 0x03], // 7
    [0x36, 0x49, 0x49, 0x49, 0x36], // 8
    [0x06, 0x49, 0x49, 0x29, 0x1E], // 9
    [0x00, 0x36, 0x36, 0x00, 0x00], // :
    [0x00, 0x56, 0x36, 0x00, 0x00], // ;
    [0x08, 0x14, 0x22, 0x41, 0x00], // <
    [0x14, 0x14, 0x14, 0x14, 0x14], // =
    [0x00, 0x41, 0x22, 0x14, 0x08], // >
    [0x02, 0x01, 0x51, 0x09, 0x06], // ?
    [0x32, 0x49, 0x79, 0x41, 0x3E], // @
    [0x7E, 0x11, 0x11, 0x11, 0x7E], // A
    [0x7F, 0x49, 0x49, 0x49, 0x36], // B
    [0x3E, 0x41, 0x41, 0x41, 0x22], // C
    [0x7F, 0x41, 0x41, 0x22, 0x1C], // D
    [0x7F, 0x49, 0x49, 0x49, 0x41], // E
    [0x7F, 0x09, 0x09, 0x09, 0x01], // F
    [0x3E, 0x41, 0x49, 0x49, 0x7A], // G
    [0x7F, 0x08, 0x08, 0x08, 0x7F], // H
    [0x00, 0x41, 0x7F, 0x41, 0x00], // I
    [0x20, 0x40, 0x41, 0x3F, 0x01], // J
    [0x7F, 0x08, 0x14, 0x22, 0x41], // K
    [0x7F, 0x40, 0x40, 0x40, 0x40], // L
    [0x7F, 0x02, 0x0C, 0x02, 0x7F], // M
    [0x7F, 0x04, 0x08, 0x10, 0x7F], // N
    [0x3E, 0x41, 0x41, 0x41, 0x3E], // O
    [0x7F, 0x09, 0x09, 0x09, 0x06], // P
    [0x3E, 0x41, 0x51, 0x21, 0x5E], // Q
    [0x7F, 0x09, 0x19, 0x29, 0x46], // R
    [0x46, 0x49, 0x49, 0x49, 0x31], // S
    [0x01, 0x01, 0x7F, 0x01, 0x01], // T
    [0x3F, 0x40, 0x40, 0x40, 0x3F], // U
    [0x1F, 0x20, 0x40, 0x20, 0x1F], // V
    [0x3F, 0x40, 0x38, 0x40, 0x3F], // W
    [0x63, 0x14, 0x08, 0x14, 0x63], // X
    [0x07, 0x08, 0x70, 0x08, 0x07], // Y
    [0x61, 0x51, 0x49, 0x45, 0x43], // Z
    [0x00, 0x7F, 0x41, 0x41, 0x00], // [
    [0x02, 0x04, 0x08, 0x10, 0x20], // '\'
    [0x00, 0x41, 0x41, 0x7F, 0x00], // ]
    [0x04, 0x02, 0x01, 0x02, 0x04], // ^
    [0x40, 0x40, 0x40, 0x40, 0x40], // _
    [0x00, 0x03, 0x07, 0x00, 0x00], // `
    [0x20, 0x54, 0x54, 0x54, 0x78], // a
    [0x7F, 0x48, 0x44, 0x44, 0x38], // b
    [0x38, 0x44, 0x44, 0x44, 0x20], // c
    [0x38, 0x44, 0x44, 0x48, 0x7F], // d
    [0x38, 0x54, 0x54, 0x54, 0x18], // e
    [0x08, 0x7E, 0x09, 0x01, 0x02], // f
    [0x08, 0x14, 0x54, 0x54, 0x3C], // g
    [0x7F, 0x08, 0x04, 0x04, 0x78], // h
    [0x00, 0x44, 0x7D, 0x40, 0x00], // i
    [0x20, 0x40, 0x44, 0x3D, 0x00], // j
    [0x7F, 0x10, 0x28, 0x44, 0x00], // k
    [0x00, 0x41, 0x7F, 0x40, 0x00], // l
    [0x7C, 0x04, 0x18, 0x04, 0x78], // m
    [0x7C, 0x08, 0x04, 0x04, 0x78], // n
    [0x38, 0x44, 0x44, 0x44, 0x38], // o
    [0x7C, 0x14, 0x14, 0x14, 0x08], // p
    [0x08, 0x14, 0x14, 0x18, 0x7C], // q
    [0x7C, 0x08, 0x04, 0x04, 0x08], // r
    [0x48, 0x54, 0x54, 0x54, 0x20], // s
    [0x04, 0x3F, 0x44, 0x40, 0x20], // t
    [0x3C, 0x40, 0x40, 0x20, 0x7C], // u
    [0x1C, 0x20, 0x40, 0x20, 0x1C], // v
    [0x3C, 0x40, 0x30, 0x40, 0x3C], // w
    [0x44, 0x28, 0x10, 0x28, 0x44], // x
    [0x0C, 0x50, 0x50, 0x50, 0x3C], // y
    [0x44, 0x64, 0x54, 0x4C, 0x44], // z
    [0x00, 0x08, 0x36, 0x41, 0x00], // {
    [0x00, 0x00, 0x7F, 0x00, 0x00], // |
    [0x00, 0x41, 0x36, 0x08, 0x00], // }
    [0x02, 0x01, 0x02, 0x04, 0x02], // ~
];


impl ST7735S {
    pub fn new(spi: SPI, dc: Pin<Output, PB1>, rst: Pin<Output, PB0>) -> Self 
    {
        ST7735S { spi, dc, rst }
    }

    pub fn init(&mut self) {
        // Reset hardware
        self.reset();
        
        // Software reset
        self.write_command(SWRESET);
        delay_ms(150);

        // Out of sleep mode
        self.write_command(SLPOUT);
        delay_ms(500);

        // Set color mode to 16 bit per pixel
        self.write_command(COLMOD);
        self.write_data(&[0x05]);
        delay_ms(10);

        // Set frame rate
        self.write_command(0xB1);
        self.write_data(&[0x00, 0x06, 0x03]);
        delay_ms(10);

        // Inversion off
        self.write_command(INVOFF);
        delay_ms(10);

        // Memory access control (directions)
        // self.set_rotation(self.rotation);

        // Column address set
        self.write_command(CASET);
        self.write_data(&[0x00, 0x00, 0x00, 0x7F]);

        // Row address set
        self.write_command(RASET);
        self.write_data(&[0x00, 0x00, 0x00, 0x9F]);

        // Normal display on
        self.write_command(NORON);
        delay_ms(10);

        // Display on
        self.write_command(DISPON);
        delay_ms(100);
    }

    pub fn write_data(&mut self, data: &[u8]) 
    {
        self.dc.set_high();
        self.spi.write(data);
    }
    
    pub fn write_command(&mut self, command: u8) 
    {
        self.dc.set_low();
        self.spi.write(&[command]);
    }

    fn reset(&mut self) 
    {
        self.rst.set_low();
        delay_ms(50);
        self.rst.set_high();
        delay_ms(50);
    }

    pub fn draw_pixel(&mut self, x: u16, y: u16, color: u16) 
    {
        if x >= WIDTH || y >= HEIGHT
        {
            return;
        }

        self.set_address_window(x, y, x, y);
        self.write_command(RAMWR);
        self.write_data(&[(color >> 8) as u8, color as u8]);
    }

    fn set_address_window(&mut self, x0: u16, y0: u16, x1: u16, y1: u16) 
    {
        self.write_command(CASET);
        self.write_data(&[(x0 >> 8) as u8, x0 as u8, (x1 >> 8) as u8, x1 as u8]);
        self.write_command(RASET);
        self.write_data(&[(y0 >> 8) as u8, y0 as u8, (y1 >> 8) as u8, y1 as u8]);
    }

    pub fn draw_char(&mut self, x: u16, y: u16, char: char, color: u16) {
        // Verifica que el carácter esté dentro del rango imprimible (ASCII 32-127)
        let index = char as usize;
        if index < 32 || index > 127 {
            return;
        }

        // Obtiene la definición del carácter desde la fuente
        let glyph = FONT[index - 32];

        // Dibuja el carácter en una matriz de 5x7 píxeles
        for (col, column_bits) in glyph.iter().enumerate() {
            for row in 0..7 {
                if (column_bits >> row) & 1 == 1 {
                    self.draw_pixel(x + col as u16, y + row as u16, color);
                }
            }
        }
    }

    pub fn draw_text(&mut self, x: u16, y: u16, text: &str, color: u16) {
        let mut cursor_x = x;

        for char in text.chars() {
            self.draw_char(cursor_x, y, char, color);
            cursor_x += 6; // Espacio entre caracteres (5 píxeles + 1 de espacio)
        }
    }

    pub fn fill_rectangle(&mut self, x: u16, y: u16, w: u16, h: u16, color: u16) {
        // Configura la ventana para el área del rectángulo completo
        self.set_address_window(x, y, x + w - 1, y + h - 1);

        // Envía el comando para iniciar la escritura en RAM
        self.write_command(RAMWR);

        // Escribe el color en todos los píxeles del área del rectángulo en modo continuo
        let pixel_data = [(color >> 8) as u8, color as u8];
        for _ in 0..(w * h) {
            self.write_data(&pixel_data);
        }

        // Restablece la ventana después del dibujo para evitar rastros
        self.reset_address_window();
    }

    fn reset_address_window(&mut self) {
        // Configura la ventana de dirección al área completa de la pantalla
        self.set_address_window(0, 0, WIDTH - 1, HEIGHT - 1);
    }

    pub fn draw_circle(&mut self, x_center: i16, y_center: i16, radius: i16, color: u16) 
    {
        let mut x = radius;
        let mut y = 0;
        let mut err = 0;

        while x >= y {
            self.draw_pixel((x_center + x) as u16, (y_center + y) as u16, color);
            self.draw_pixel((x_center + y) as u16, (y_center + x) as u16, color);
            self.draw_pixel((x_center - y) as u16, (y_center + x) as u16, color);
            self.draw_pixel((x_center - x) as u16, (y_center + y) as u16, color);
            self.draw_pixel((x_center - x) as u16, (y_center - y) as u16, color);
            self.draw_pixel((x_center - y) as u16, (y_center - x) as u16, color);
            self.draw_pixel((x_center + y) as u16, (y_center - x) as u16, color);
            self.draw_pixel((x_center + x) as u16, (y_center - y) as u16, color);

            y += 1;
            err += 1 + 2 * y;
            if 2 * (err - x) + 1 > 0 {
                x -= 1;
                err += 1 - 2 * x;
            }
        }
    }

    pub fn clear(&mut self, color: u16) 
    {
        self.fill_rectangle(0, 0, WIDTH, HEIGHT, color);
    }
}