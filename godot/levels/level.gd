class_name Level
extends Node2D

@export var NEXT_LEVEL: PackedScene

enum GameState {
	Playing,
	Won,
	Lost,
}

var _game_state := GameState.Playing

func _ready() -> void:
	AudioManager.play_bgm()

func _unhandled_key_input(event: InputEvent) -> void:
	if _game_state == GameState.Playing:
		return

	if event.is_action_pressed("ui_accept"):
		if _game_state == GameState.Won:
			get_tree().change_scene_to_packed(NEXT_LEVEL)
		else:
			get_tree().reload_current_scene()

func player_won() -> void:
	_game_state = GameState.Won
	$HUD.show_won()

func player_lost() -> void:
	$HUD.show_lost()
	_game_state = GameState.Lost
