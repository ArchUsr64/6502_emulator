loop:
	nop
	jmp loop
	.org $f000:
	.byte $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff
	.byte $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff
	.byte $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff
	.byte $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff
	.byte $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff
	.byte $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff
	.byte $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff
	.byte $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff
	.byte $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $f6, $f6, $fa, $f1, $ff, $ed, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff
	.byte $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $f2, $f1, $ed, $e8, $e8, $e8, $e8, $e8, $f2, $e8, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff
	.byte $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $f2, $f1, $ec, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $f1, $e8, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff
	.byte $ff, $ff, $fb, $ff, $ff, $ff, $ff, $fb, $f6, $f1, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $fa, $fa, $ff, $ff, $ff, $f2, $fb, $ff, $fb
	.byte $ff, $f6, $fb, $e8, $ff, $ff, $ff, $fb, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $f6, $ff, $ff, $f6, $e8, $fb, $fb, $f6
	.byte $f6, $ed, $fb, $e8, $f2, $ff, $f2, $ed, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $ed, $f1, $ff, $ed, $e8, $f6, $ec, $fb
	.byte $fb, $e8, $ed, $e8, $f2, $ff, $f6, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $ed, $ff, $f6, $e8, $e8, $ed, $ff
	.byte $ff, $fa, $e8, $e8, $fb, $f2, $ed, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $c8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $f1, $fb, $e8, $f1, $ff, $ff
	.byte $ff, $ff, $ff, $e8, $fb, $f6, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $fb, $b6, $a4, $e8, $e8, $ff, $6d, $e8, $e8, $e8, $e8, $e8, $e8, $ed, $f6, $f2, $ff, $ff, $ff
	.byte $ff, $ff, $ff, $ff, $ed, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $91, $6d, $60, $e8, $c8, $92, $00, $c4, $e8, $e8, $e8, $e8, $e8, $e8, $ed, $ff, $ff, $ff, $ff
	.byte $ff, $ff, $ff, $ff, $f2, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $c4, $40, $a4, $e8, $e8, $84, $84, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $f6, $ff, $ff, $ff
	.byte $ff, $ff, $ff, $ff, $e8, $f2, $b2, $d1, $c8, $e8, $e8, $e8, $e8, $e8, $e8, $e8, $a4, $c4, $e8, $e8, $e8, $e8, $e8, $c8, $cd, $d6, $d2, $e8, $f2, $ff, $ff, $ff
	.byte $ff, $ff, $ff, $ff, $fb, $e8, $ff, $d6, $ff, $fb, $d6, $d1, $c4, $c8, $e8, $e8, $c4, $c4, $e8, $c8, $c8, $d1, $d6, $fb, $ff, $d6, $fb, $ed, $ff, $ff, $ff, $ff
	.byte $ff, $ff, $ff, $ff, $ff, $fb, $f2, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $fb, $f6, $fb, $ff, $ff, $ff, $ff
	.byte $ff, $ff, $ff, $ff, $ff, $ff, $fb, $fb, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $f6, $ff, $ff, $ff, $ff, $ff
	.byte $ff, $ff, $ff, $ff, $ff, $ff, $ff, $fb, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $fb, $ff, $ff, $ff, $ff, $ff, $ff
	.byte $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff
	.byte $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff
	.byte $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff
	.byte $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff
	.byte $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff
	.byte $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff
	.byte $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff
	.byte $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff, $ff
