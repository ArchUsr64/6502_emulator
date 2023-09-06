start:
	; Set initial position for the pixel
	ldx #$20
	ldy #$f0
	jmp main
	.org $200

; Main app loop
main:

	; Handle inputs
	lda $fb
	cmp #$0
	bne not_left
	dex
not_left:
	lda $fc
	cmp #$0
	bne not_down
	iny
not_down:
	lda $fd
	cmp #$0
	bne not_up
	dey
not_up:
	lda $fe
	cmp #$0
	bne not_right
	inx

not_right:
	; Set random color for the pixel
	lda $ff
	jsr render_pixel
	jmp main

; Renders the pixel to screen
render_pixel:
	; Expected parameters:
	; x-coordinate -> X
	; y-coordinate -> Y
	; Color -> A

	; Store coords for later retrival
	sta $04
	sty $05

; Y %= 32
y_mod_32:
	tya
	and #$1f
	tay

; X %= 32
x_mod_32:
	txa
	and #$1f
	tax

	; X = (Y & 0b111) << 5 + X
	tya
	and #$7
	asl
	asl
	asl
	asl
	asl
	stx $00
	adc $00
	tax

	; Y = (Y >> 3) & 0b11
	tya
	lsr
	lsr
	lsr
	and #$3
	tay
	lda $04
	cpy #1
	bcs ge_to_1
	sta $fb00,x
	ldx $00
	ldy $05
	rts
ge_to_1:
	cpy #2
	bcs ge_to_2
	sta $fc00,x
	ldx $00
	ldy $05
	rts
ge_to_2:
	cpy #3
	bcs ge_to_3
	sta $fd00,x
	ldx $00
	ldy $05
	rts
ge_to_3:
	sta $fe00,x
	ldx $00
	ldy $05
	rts
