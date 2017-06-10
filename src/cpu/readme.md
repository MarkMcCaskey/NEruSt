Just gonnda dump some shit here for the time being

// Memory addressing modes
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

// How "izx" works
ixy isn't used nearly as often as "izy." In fact, it is not used once anywhere in the
Super Mario Bros rom. It works like zpx, but dereferences the result.

Lets say your RAM looks like this:
$2C: 0x20
$2D: 0x22
$2E: 0x10
$2F: 0x11

and "x" contained 2
``LDA ($2C,x)`` would become ``LDA $1011``

// How "izy" works
It is far more common than izx. It works like izx, except it dereferences first, then indexes.

Imagine this: Lets say your RAM looks like this
$13: 0x25
$14: 0x45

and "y" contained 5
``LDA ($13),y`` would become ``LDA $2545``