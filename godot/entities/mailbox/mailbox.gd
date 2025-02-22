@tool
class_name Mailbox
extends Node2D

@export var tile_pos := Vector2i.ZERO:
	set(value):
		tile_pos = value
		var grid: TileMapLayer = %GroundTiles
		if grid:
			position = grid.to_global(grid.map_to_local(tile_pos))
