data = [
    0x00, 0xE0,        # CLS (Clear the screen)

    0x60, 0x05,        # LD V0, 5   (Set X coord)
    0x61, 0x05,        # LD V1, 5   (Set Y coord)
    0xA2, 0x10,        # LD I, 0x210 (Sprite address)
    0xD0, 0x11,        # DRW V0, V1, 1  (Draw sprite at (5,5))

    0xF2, 0x0A,        # LD V2, K (Wait for keypress)
    
    0x72, 0x01,        # ADD V2, 1 (V2 += 1)
    0x82, 0x31,        # OR V2, V3
    0x82, 0x32,        # AND V2, V3
    0x82, 0x33,        # XOR V2, V3

    0x82, 0x35,        # SUB V2, V3
    0x82, 0x37,        # SUBN V2, V3

    0x82, 0x36,        # SHR V2
    0x82, 0x3E,        # SHL V2

    0xA2, 0x20,        # LD I, 0x220
    0xF3, 0x55,        # Store V0-V3 in memory (at I)
    0xF3, 0x65,        # Load V0-V3 from memory

    0x22, 0x30,        # CALL subroutine at 0x230
    0x12, 0x40,        # JUMP to 0x240

    # Subroutine at 0x230 (Returns immediately)
    0x00, 0xEE,        # RET (Return from subroutine)

    # Address 0x240 (End)
    0x12, 0x40,        # JUMP to 0x240 (Loop infinitely)
]

if __name__ == "__main__":
    with open("bin/output.bin", "wb") as f:
        f.write(bytes(data))
