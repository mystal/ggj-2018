@tool
class_name Mail
extends Node2D

const PICK_UP_SFX: AudioStream = preload("res://entities/mail/got_mail.wav")

@export var tile_pos := Vector2i.ZERO:
	set(value):
		tile_pos = value
		var grid: TileMapLayer = %GroundTiles
		if grid:
			position = grid.to_global(grid.map_to_local(tile_pos))

func pick_up() -> void:
	AudioManager.play_sfx(PICK_UP_SFX)
	queue_free()
