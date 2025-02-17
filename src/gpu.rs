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
}

/*PPU MODES
MODE 2: OAM scan. Search for objects that overlap this line. 80 dots. VRAM accesible
MODE 3: VRAM scan. Sends pixels to the LCD. 172-289 dots. VRAM and OAM are inaccessible
MODE 0: HBlank. Waits until de end of the scanline. 204 dots.

MODE 1: VBlank. Waits until the next frame. 4560 dots. VRAM and OAM are accessible
*/

const DOT: u16 = 4; // 4 dots per M-cycle
