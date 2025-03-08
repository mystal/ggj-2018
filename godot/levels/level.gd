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

	for node in get_tree().get_nodes_in_group("player"):
		var player := node as Player
		if player:
			player.moved.connect(_player_moved)

func _unhandled_key_input(event: InputEvent) -> void:
	if _game_state == GameState.Playing:
		return

	if event.is_action_pressed("ui_accept"):
		if _game_state == GameState.Won:
			get_tree().change_scene_to_packed(NEXT_LEVEL)
		else:
			get_tree().reload_current_scene()

func _player_moved(_old_pos: Vector2i, _new_pos: Vector2i) -> void:
	var all_pugs := get_tree().get_nodes_in_group("pugs")
	for node in all_pugs:
		var pug := node as Pug
		if not pug or pug.is_dead:
			continue
		pug.step()

func player_won() -> void:
	_game_state = GameState.Won
	$HUD.show_won()

func player_lost() -> void:
	$HUD.show_lost()
	_game_state = GameState.Lost
