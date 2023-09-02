start:
	jmp main
	.org $0200
main:
	lda $fe
	sta $f000,x
	lda $fe
	sta $f100,x
	lda $fe
	sta $f200,x
	lda $fe
	sta $f300,x
	inx
	jmp main
