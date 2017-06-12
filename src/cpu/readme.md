Just gonnda dump some shit here for the time being

/// Memory adressing modes
imm = #$00
zp = $00
zpx = $00,X
zpy = $00,Y
izx = ($00,X)
izy = ($00),Y
abs = $0000
abx = $0000,X
aby = $0000,Y
ind = ($0000)

/// Not so obvious "features"
ZP indexing will not leave ZP when crossing the page boundary:
LDX #$01
LDA $0xFF,x // this will NOT load $0x0100, rather it will load $0x0000
LDA $0x00FF,x // this _WILL_ load $0x0100, since it is in abs adressing mode

Indrect addressing will not fetch adresses that cross page boundaries (even in abs!):
LDX #$00
LDA ($FF),y // this will fetch the lobyte from $00FF and the hibyte from $0000 
JMP ($06FF) // this will fetch the lobyte from $06FF and the hibyte from $0600