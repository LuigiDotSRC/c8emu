data = [0x41, 0x42, 0x43, 0x44, 0xA2, 0xF0]

if __name__ == "__main__":
    with open("bin/output.bin", "wb") as f:
        f.write(bytes(data))