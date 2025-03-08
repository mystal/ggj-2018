@tool
class_name Bone
extends TileNode

const THROW_PREVIEW: PackedScene = preload("res://entities/bone/throw_preview.tscn")

var _throw_previews: Array[ThrowPreview]

func pick_up() -> void:
	var grid: TileMapLayer = %GroundTiles
	for dir: Vector2i in [Vector2i.UP, Vector2i.DOWN, Vector2i.LEFT, Vector2i.RIGHT]:
		var preview_pos := tile_pos + dir
		if grid.get_cell_source_id(preview_pos) >= 0:
			var new_preview = THROW_PREVIEW.instantiate()
			add_child(new_preview)
			new_preview.tile_pos = preview_pos
			new_preview.global_position = grid.to_global(grid.map_to_local(preview_pos))
			_throw_previews.append(new_preview)
	$BoneSprite.visible = false
	$ShadowSprite.visible = false

func throw(throw_tile_pos: Vector2i) -> void:
	# Only destroy the ones not at throw_tile_pos.
	for preview in _throw_previews:
		if preview.tile_pos == throw_tile_pos:
			preview.flicker()
		else:
			preview.queue_free()

	# Tell nearby pugs to inspect this spot.
	var all_pugs := get_tree().get_nodes_in_group("pugs")
	for node in all_pugs:
		var pug := node as Pug
		if not pug or pug.is_dead:
			continue
		if pug.tile_pos.distance_squared_to(throw_tile_pos) <= 2:
			pug.inspect(throw_tile_pos)

	await get_tree().create_timer(0.5).timeout

	# Destroy self and last preview bone.
	queue_free()
