use crate::{
    cpu::Registers,
    mmu::{self, MMU},
};

enum GPUControlRegisters {
    LCDC = 0xFF40,
    /*
    bit 7: LCD Display Enable (0=Off, 1=On) 1 to 0 only during VBlank
    bit 6: Window Tile Map area (0=9800-9BFF, 1=9C00-9FFF)
    bit 5: Window Enable (0=Off, 1=On)
    bit 4: BG & Window Tile Data area (0=8800-97FF, 1=8000-8FFF)
    bit 3: BG Tile Map area (0=9800-9BFF, 1=9C00-9FFF)
    bit 2: OBJ (Sprite) Size (0=8x8, 1=8x16)
    bit 1: OBJ (Sprite) Display Enable (0=Off, 1=On)
    bit 0: BG enable / priority (0=Off, 1=On)
    */
    LY = 0xFF44,
    /*
    Read only
    Indicates the current horizontal line of the draw process
    Can hold any value from 0 to 153, with values from 144 to 153 indicating the VBlank period.
     */
    LYC = 0xFF45,
    /*
    The Game Boy constantly compares the value of the LYC and LY registers.
    When both values are identical, the “LYC=LY” flag in the STAT register is set, and
    (if enabled) a STAT interrupt is requested.
    */
    STAT = 0xFF41,
    /*
    bit 7: not used
    bit 6: If set, selects the LYC == LY condition for the STAT interrupt
    bit 5: If set, enables the Mode 2 STAT interrupt
    bit 4: If set, enables the Mode 1 STAT interrupt
    bit 3: If set, enables the Mode 0 STAT interrupt
    bit 2: Set when LY contains the same value as LYC. READ-ONLY
    bit 1-0: Indicates PPU current status. READ-ONLY
     */
    SCY = 0xFF42,
    SCX = 0xFF43,
    /*
    Background viewport Y position, X position

    Specyfies the top-left coordinates of the visible 160×144 pixel area within the
    256×256 pixels BG map. Values in the range 0–255

    The PPU calculates the bottom-right coordinates of the viewport with those formulas:
    bottom := (SCY + 143) % 256
    right := (SCX + 159) % 256
     */
    WY = 0xFF4A,
    WX = 0xFF4B,
    /*
    Window Y position, X position
    specifies the on-screen coordinates of the Window’s top-left pixel.
    The Window is visible (if enabled) when both coordinates are in the ranges
    WX=0..166, WY=0..143
    Values WX=7, WY=0 place the Window at the top left of the screen, completely covering the background.
    */
    BGP = 0xFF47,
    /*
    Background Palette Data
    Specifies the colors for the background. Each 2 bits of the byte represent a color.
    Bit 7-6 - ID3
    Bit 5-4 - ID2
    Bit 3-2 - ID1
    Bit 1-0 - ID0
    */
    OBP0 = 0xFF48,
    OBP1 = 0xFF49,
    /*
    Object Palette Data 0, 1
    Assigns gray shades to the color indexes of the OBJs that use the corresponding palette.
    Works like BGP but the lower two bits are ignored because color 0 is always transparent.
    */
    DMA = 0xFF46,
    /*
    DMA Transfer and Start Address
    Writing to this register launches a DMA transfer from ROM or RAM to OAM
    The transfer takes 160 M-Cycles, 640 dots
    The source address is specified by the written value * 100h.
    */
}

const SCANLINES_PER_FRAME: u16 = 154;
const DOTS_PER_SCANLINE: u16 = 456;
const DOT: u16 = 4; // 4 dots per M-cycle

enum TileMap {
    TILE_MAP_1 = 0x9800, // Tile Map 1 (32x32 tiles) 0x9800-0x9BFF
    TILE_MAP_2 = 0x9C00, // Tile Map 2 (32x32 tiles) 0x9C00-0x9FFF
}
const TILE_MAP_LENTH: u16 = 1024; // 1024 bytes

const OAM: usize = 0xFE00; // Object (Sprites) 0xFE00-0xFE9F
const OAM_END: usize = 0xFE9F; // 4 x 40 bytes

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

pub struct OamObject {
    y: u8, // byte 0: Y position + 16
    x: u8, // byte 1: X position + 8
    tile_index: u8,
    /*
       byte 2: Tile index
       In 8×8 mode specifies the object’s only tile index ($00-$FF). (Es el ordinal de la tile)
       This unsigned value selects a tile from the memory area at $8000-$8FFF

       In 8×16 mode the memory area at $8000-$8FFF is still interpreted as a series of 8×8 tiles,
       where every 2 tiles form an object.
       In this mode, this byte specifies the index of the first (top) tile of the object.
       the least significant bit of the tile index is ignored
    */
    flags: u8,
    /*
    byte 3: Flags
        bit 7: Priority
            0: OBJ above BG
            1: OBJ behind BG
        bit 6: Y flip
        bit 5: X flip
        bit 4: Palette number
            0: OBJ Palette 0
            1: OBJ Palette 1
        bit 3-0: not used
    */
}

impl OamObject {
    fn new(address: u16, mmu: &MMU) -> Self {
        OamObject {
            y: mmu.read_byte(address) - 16,
            x: mmu.read_byte(address + 1) - 8,
            tile_index: mmu.read_byte(address + 2),
            flags: mmu.read_byte(address + 3),
        }
    }
}

struct Tile {
    pixels: [u8; 8 * 8],
}

struct Pixel {
    Color: u8,
    Palette: Option<bool>,
    Bg_priority: bool,
}

pub struct Screen {
    pub pixels: [u8; WIDTH * HEIGHT],
    pub ppu_mode: u8,
    pub obj_list: Vec<OamObject>,
    tile_map: TileMap,
    fifo_obj: Vec<Pixel>,
    fifo_bg: Vec<Pixel>,
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            pixels: [0; 160 * 144],
            ppu_mode: 0,
            obj_list: Vec::new(),
            tile_map: TileMap::TILE_MAP_1,
            fifo_obj: Vec::new(),
            fifo_bg: Vec::new(),
        }
    }

    pub fn step(&mut self, mmu: &mut MMU) -> u16 {
        let ly = mmu.read_byte(GPUControlRegisters::LY as u16);

        match self.ppu_mode {
            2 => {
                //OAM scan. Search for objects that overlap this line. 80 dots. VRAM accesible
                mmu.oam_enable = false;
                mmu.vram_enable = true;

                self.obj_list.clear();
                for i in (OAM as u16..=OAM_END as u16).step_by(DOT as usize) {
                    if self.obj_list.len() == 10 {
                        break;
                    }
                    if mmu.read_byte(i) - 16 == ly {
                        self.obj_list.push(OamObject::new(i, &mmu));
                    }
                }

                self.ppu_mode = 3;
                80
            }
            3 => {
                //VRAM scan. Sends pixels to the LCD. 172-289 dots. VRAM and OAM are inaccessible
                mmu.oam_enable = false;
                mmu.vram_enable = false;
                let dots = 172;

                for x_coord in 0..WIDTH {
                    let pixel_obj = self.get_pixel_obj(x_coord as u8, ly, mmu);
                }

                dots
            }
            0 => {
                //HBlank. Waits until de end of the scanline. 204 dots.
                mmu.oam_enable = true;
                mmu.vram_enable = true;
                0
            }
            1 => {
                //VBlank. Waits until the next frame. 4560 dots. VRAM and OAM are accessible
                mmu.oam_enable = true;
                mmu.vram_enable = true;
                0
            }
            _ => {
                panic!("Invalid PPU mode")
            }
        }
    }

    fn get_pixel_obj(&mut self, x: u8, y: u8, mmu: &MMU) -> Option<Pixel> {
        let obj_in_range = self
            .obj_list
            .iter()
            .filter(|obj| (obj.x..obj.x + 8).contains(&x))
            .min_by_key(|obj| obj.x);

        match obj_in_range {
            None => None,
            Some(obj) => Some(Pixel {
                Color: self.get_obj_color(obj, x, y, mmu),
                Palette: self.get_obj_palette(obj),
                Bg_priority: self.get_obj_priority(obj),
            }),
        }
    }

    fn get_obj_color(&self, obj: &OamObject, x: u8, y: u8, mmu: &MMU) -> u8 {
        // Posicion x e y dentro de la tile. se tiene en cuenta el flip
        let x_rel = if obj.flags & 0x20 != 0 {
            7 - (x - obj.x)
        } else {
            x - obj.x
        };
        let y_rel = if obj.flags & 0x40 != 0 {
            7 - (y - obj.y)
        } else {
            y - obj.y
        };

        // Comprobar modo 8x8 o 8x16
        let tile_base = if mmu.read_byte(GPUControlRegisters::LCDC as u16) & 0x04 != 0 {
            obj.tile_index & 0xFE // Ignorar el bit menos significativo en modo 8x16
        } else {
            obj.tile_index // Usar el índice tal cual en modo 8x8
        };

        let address = 0x8000 + (tile_base as u16 * 16) + (y_rel as u16 * 2);
        let lower = mmu.read_byte(address);
        let higher: u8 = mmu.read_byte(address + 1);

        let bit = 7 - x_rel;
        ((higher >> bit) & 1) << 1 | ((lower >> bit) & 1)
    }

    fn get_obj_palette(&self, obj: &OamObject) -> Option<bool> {
        if obj.flags & 0x10 != 0 {
            Some(true)
        } else {
            Some(false)
        }
    }

    fn get_obj_priority(&self, obj: &OamObject) -> bool {
        if obj.flags & 0x80 != 0 {
            true
        } else {
            false
        }
    }
}
