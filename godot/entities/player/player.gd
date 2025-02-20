@tool
class_name Player
extends Node2D

@export_category("Sprites")
@export var FRONT_SPRITE: Texture2D
@export var BACK_SPRITE: Texture2D
@export var MAIL_FRONT_SPRITE: Texture2D
@export var MAIL_BACK_SPRITE: Texture2D

@export_category("Movement")
@export var facing := Enums.Direction.EAST:
	set(value):
		facing = value
		_update_sprite()
@export var tile_pos := Vector2i.ZERO:
	set(value):
		tile_pos = value
		var grid: TileMapLayer = %GroundTiles
		if grid:
			position = grid.to_global(grid.map_to_local(tile_pos))

var _has_mail := false

func _ready() -> void:
	_update_sprite()

func _unhandled_key_input(event: InputEvent) -> void:
	var dir := Vector2i.ZERO
	if event.is_action_pressed("ui_left"):
		dir = Vector2i.LEFT
		facing = Enums.Direction.WEST
	elif event.is_action_pressed("ui_right"):
		dir = Vector2i.RIGHT
		facing = Enums.Direction.EAST
	elif event.is_action_pressed("ui_up"):
		dir = Vector2i.UP
		facing = Enums.Direction.NORTH
	elif event.is_action_pressed("ui_down"):
		dir = Vector2i.DOWN
		facing = Enums.Direction.SOUTH

	if dir != Vector2i.ZERO:
		# TODO: Check if there is a cell at tile_pos + dir
		var new_pos := tile_pos + dir
		var grid: TileMapLayer = %GroundTiles
		if grid.get_cell_source_id(new_pos) >= 0:
			tile_pos = new_pos
			_check_overlaps()

func _check_overlaps() -> void:
	var all_mail := get_tree().get_nodes_in_group("mail")
	for node in all_mail:
		var mail = node as Mail
		if mail and mail.tile_pos == tile_pos:
			# Pick up mail piece!
			_has_mail = true
			_update_sprite()
			mail.pick_up()

func _update_sprite() -> void:
	match facing:
		Enums.Direction.NORTH, Enums.Direction.WEST:
			if _has_mail:
				$FoxSprite.texture = MAIL_BACK_SPRITE
			else:
				$FoxSprite.texture = BACK_SPRITE
		Enums.Direction.SOUTH, Enums.Direction.EAST:
			if _has_mail:
				$FoxSprite.texture = MAIL_FRONT_SPRITE
			else:
				$FoxSprite.texture = FRONT_SPRITE
	var x_offset = absf($FoxSprite.position.x)
	if facing in [Enums.Direction.NORTH, Enums.Direction.EAST]:
		$FoxSprite.flip_h = true
		$FoxSprite.position.x = x_offset
	else:
		$FoxSprite.flip_h = false
		$FoxSprite.position.x = -x_offset
