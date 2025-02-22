@tool
class_name Mail
extends TileNode

const PICK_UP_SFX: AudioStream = preload("res://entities/mail/got_mail.wav")

func pick_up() -> void:
	AudioManager.play_sfx(PICK_UP_SFX)
	queue_free()
