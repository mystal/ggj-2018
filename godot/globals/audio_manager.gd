extends AudioStreamPlayer

const BGM: AudioStream = preload("res://shared/background_music_loop.ogg")

func play_bgm() -> void:
	if stream == BGM:
		return

	stream = BGM
	volume_db = -10.0
	play()
