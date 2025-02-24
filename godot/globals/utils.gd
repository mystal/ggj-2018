extends Node

func dir_to_vec(dir: Enums.Direction) -> Vector2i:
	match dir:
		Enums.Direction.NORTH:
			return Vector2i.UP
		Enums.Direction.SOUTH:
			return Vector2i.DOWN
		Enums.Direction.EAST:
			return Vector2i.RIGHT
		Enums.Direction.WEST:
			return Vector2i.LEFT
	return Vector2i.ZERO
