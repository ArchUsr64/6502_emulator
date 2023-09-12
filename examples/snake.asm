; Global game state:
	; Snake size: 0x10
		; One more than the size to be rendered
		; The tail is used to erase off the remaining elements
	; Facing: 0x11
		; 1 -> Left
		; 2 -> Down
		; 3 -> Up
		; 4 -> Right
	; Body coords:
		; Stored form 0x12 to 0x(2*size + 1) with [x, y] point indexing
	; Apple position stored at x: 0xa0 y: 0xa1

start:
	jmp main
	.org $200

; Initialization stuff
main:
	; Position for the apple
	lda #12
	sta $a0
	lda #12
	sta $a1
	; Snake size
	lda #$4
	sta $10
	; Direction for snake
	lda #$1
	sta $11
	; Snake head coords
	; (10, 10), (10, 11), (11, 11), (12, 11)
	lda #$10
	sta $12
	lda #$10
	sta $13
	lda #$10
	sta $14
	lda #$11
	sta $15
	lda #$11
	sta $16
	lda #$11
	sta $17
	lda #$12
	sta $18
	lda #$11
	sta $19
	jmp loop

; game loop
loop:
	; Handle inputs
	lda $fb
	cmp #$0
	beq not_left
	lda #$1
	jmp store_input
not_left:
	lda $fc
	cmp #$0
	beq not_down
	lda #$2
	jmp store_input
not_down:
	lda $fd
	cmp #$0
	beq not_up
	lda #$3
	jmp store_input
not_up:
	lda $fe
	cmp #$0
	beq store_input
	lda #$4
	jmp store_input

store_input:
	cmp #$0
	beq input_handled 
	sta $11

input_handled:
	jsr update_snake
	jsr render_apple
	jsr render_snake
	jmp loop

; Update snake position
update_snake:

	; Compute new head position based on facing direction
	; NewHead.x -> $e2
	; NewHead.y -> $e3
	lda $12
	sta $e2
	lda $13
	sta $e3
	lda $11
	cmp #$1
	bne snake_not_left
	dec $e2
	jmp direction_done
snake_not_left:
	cmp #$2
	bne snake_not_down
	inc $e3
	jmp direction_done
snake_not_down:
	cmp #$3
	bne snake_not_up
	dec $e3
	jmp direction_done
snake_not_up:
	cmp #$4
	bne direction_done
	inc $e2
direction_done:
	lda $e2
	and #$1f
	sta $e2
	lda $e3
	and #$1f
	sta $e3

	; Move each value two positions down the memory
	; using temporary coordinate at 0xe8, 0xe9
	; length iterator at 0xe0
	lda $10
	sta $e0
iter_snake:
	; A goes from 0 to tail - 1
	lda $10
	sbc $e0
	asl
	tax
	tay
	iny

	lda $12,x
	sta $e4
	lda $12,y
	sta $e5

	; Move the X coordinate
	lda $e2
	sta $12,x
	; Move the Y coordinate
	lda $e3
	sta $12,y

	lda $e4
	sta $e2
	lda $e5
	sta $e3

	dec $e0
	lda $e0
	cmp #$0
	bne iter_snake

	rts

; Render the snake to screen
render_snake:
	; is body boolean at 0xe1
	; Stores color of head initially, once head is rendered changed to body color
	lda #$1e
	sta $e1
	; length iterator at 0xe0
	lda $10
	sta $e0
iter:
	; A goes from 0 to tail - 1
	lda $10
	sbc $e0
	asl
	tax
	tay
	iny
	; Corresponding registers X and Y store the offset of coordinate from 0x12
	lda $12,x
	tax
	lda $12,y
	tay

	; Erase tail
	lda $e0
	cmp #$1
	bne not_tail
	lda #$0
	sta $e1

not_tail:
	lda $e1
	stx $e4
	sty $e5
	jsr render_pixel

	; Store green color once the head has been rendered
	lda #$10
	sta $e1


	dec $e0
	lda $e0
	cmp #$0
	bne iter

	; .byte $97, $23
	rts


; Renders the apple to screen
render_apple:
	ldx $a0
	ldy $a1
	lda #$e0
	jsr render_pixel
	rts

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
