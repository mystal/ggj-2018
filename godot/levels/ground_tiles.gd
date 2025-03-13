class_name GroundTiles
extends TileMapLayer

@onready var astar_grid: AStarGrid2D

func _ready() -> void:
	astar_grid = AStarGrid2D.new()
	astar_grid.region = get_used_rect()
	astar_grid.diagonal_mode = AStarGrid2D.DIAGONAL_MODE_NEVER
	astar_grid.update()

	# Set the solid points based on empty tilemap cells.
	var start := astar_grid.region.position
	var size := astar_grid.region.size
	for y in size.y:
		for x in size.x:
			var tile := Vector2i(start.x + x, start.y + y)
			if get_cell_source_id(tile) < 0:
				astar_grid.set_point_solid(tile)

func get_tile_path(from: Vector2i, to: Vector2i) -> Array[Vector2i]:
	return astar_grid.get_id_path(from, to)
