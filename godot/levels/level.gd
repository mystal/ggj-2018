class_name Level
extends Node2D

@export var NEXT_LEVEL: PackedScene

func _ready() -> void:
	AudioManager.play_bgm()

func _start_next_level() -> void:
	get_tree().change_scene_to_packed(NEXT_LEVEL)
