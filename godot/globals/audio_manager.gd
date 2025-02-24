extends AudioStreamPlayer

const BGM: AudioStream = preload("res://shared/background_music_loop.ogg")

var num_sfx_players := 8
var _available: Array[AudioStreamPlayer] = []

func _ready() -> void:
	bus = "Music"

	for i in num_sfx_players:
		_create_player()

func play_bgm() -> void:
	if stream == BGM:
		return

	stream = BGM
	volume_db = -10.0
	play()

func play_sfx(sfx_stream: AudioStream) -> void:
	# TODO: Try to get an available player, if none available make a new one!
	if _available.is_empty():
		_create_player()

	var player := _available.pop_back() as AudioStreamPlayer
	player.stream = sfx_stream
	player.play()

func _create_player() -> void:
	var player = AudioStreamPlayer.new()
	add_child(player)
	_available.append(player)
	player.finished.connect(_on_stream_finished.bind(player))
	player.bus = "SFX"

func _on_stream_finished(player: AudioStreamPlayer) -> void:
	_available.append(player)
