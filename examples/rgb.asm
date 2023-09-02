start:
	jmp main
	.org $0200
main:
	ldx #0
loop:
	inx
	txa
	tay
	sta $fb00,y
	sta $fc00,y
	sta $fd00,y
	sta $fe00,y
	jmp loop
