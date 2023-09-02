start:
	jmp main
	.org $0200
main:
	lda $ff
	sta $fb00,x
	lda $ff
	sta $fc00,x
	lda $ff
	sta $fd00,x
	lda $ff
	sta $fe00,x
	inx
	jmp main
