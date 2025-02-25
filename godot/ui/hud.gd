class_name Hud
extends CanvasLayer

func show_won() -> void:
	pass

func show_lost() -> void:
	$LoseText.visible = true
	$RetryText.visible = true
	$RetryText/TextAnimationPlayer.play("play_text_blink")
