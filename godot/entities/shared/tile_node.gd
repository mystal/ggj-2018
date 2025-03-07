@tool
class_name TileNode
extends Node2D

@export var tile_pos := Vector2i.ZERO:
	set(value):
		tile_pos = value
		if is_node_ready():
			var grid: TileMapLayer = %GroundTiles
			if grid:
				global_position = grid.to_global(grid.map_to_local(tile_pos))
