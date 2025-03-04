@tool
class_name Bone
extends TileNode

const THROW_PREVIEW: PackedScene = preload("res://entities/bone/throw_preview.tscn")

var _throw_previews: Array[Node2D]

func pick_up() -> void:
	var grid: TileMapLayer = %GroundTiles
	for dir: Vector2i in [Vector2i.UP, Vector2i.DOWN, Vector2i.LEFT, Vector2i.RIGHT]:
		var preview_pos := tile_pos + dir
		if grid.get_cell_source_id(preview_pos) >= 0:
			var new_preview = THROW_PREVIEW.instantiate()
			add_child(new_preview)
			new_preview.global_position = grid.to_global(grid.map_to_local(preview_pos))
			_throw_previews.append(new_preview)
	visible = false

func throw(throw_tile_pos: Vector2i) -> void:
	# TODO: Only destroy the ones not at throw_tile_pos.
	# TODO: And destroy the last one after it blinks for a set time.
	for preview in _throw_previews:
		preview.queue_free()
	queue_free()
