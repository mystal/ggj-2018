class_name MenuScreen
extends Control

@export var NEXT_LEVEL: PackedScene

func _ready() -> void:
	AudioManager.play_bgm()

func _unhandled_key_input(event: InputEvent) -> void:
	if event.is_action_pressed("ui_accept"):
		get_tree().change_scene_to_packed(NEXT_LEVEL)
