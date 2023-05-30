static mut VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
static mut VGA_BUFFER_OFFSET: isize = 0;

#[derive(Clone)]
pub enum Colors {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Purple = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xa,
    LightCyan = 0xb,
    LightRed = 0xc,
    Pink = 0xd,
    Yellow = 0xe,
    White = 0xf,
}

pub fn print(string: &[u8], color: Colors) -> Result<(), ()> {
    for &byte in string.iter() {
        unsafe {
            *VGA_BUFFER.offset(VGA_BUFFER_OFFSET * 2) = byte;
            *VGA_BUFFER.offset(VGA_BUFFER_OFFSET * 2 + 1) = color.clone() as u8;
        
            VGA_BUFFER_OFFSET += 1;
        }
    }

    return Ok(());
}

pub fn println(string: &[u8], color: Colors) -> Result<(), ()> {
    print(string, color)?;

    unsafe {
        VGA_BUFFER_OFFSET += 80 - (VGA_BUFFER_OFFSET % 80);
    }

    return Ok(());
}
