@tool
class_name ThrowPreview
extends TileNode

func flicker() -> void:
	$AnimationPlayer.play("flicker")
