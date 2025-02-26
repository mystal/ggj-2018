class_name Hud
extends CanvasLayer

func show_won() -> void:
	$WinText.visible = true
	$RetryText.visible = true
	$RetryText.text = "Press Space"
	$RetryText/TextAnimationPlayer.play("play_text_blink")

func show_lost() -> void:
	$LoseText.visible = true
	$RetryText.visible = true
	$RetryText/TextAnimationPlayer.play("play_text_blink")
