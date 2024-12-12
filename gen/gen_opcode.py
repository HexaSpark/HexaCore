# Opcode rules:
# If opcode name has more than one addressing mode, add the first letter of the addressing mode to the opcode name.
# Else, use the full opcode.
# If a result is already taken or zero, add a space character at the end.
#
# Example:
# LDR - More than one. (LDRI, LDRA, LDRR)
# CLC - Only one. (CLC)
# DAC & ADC - (Same, add a space at end: 'ADC ')

while True:
    result = 0
    
    for char in input("Enter the opcode: "):
        result += ord(char)
        
    if result == 0: continue
        
    print(f"Result: {hex(result & 0xFF).capitalize().replace('X', 'x')}")