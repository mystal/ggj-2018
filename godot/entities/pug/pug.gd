@tool
class_name Pug
extends TileNode

enum State {
	Guarding,
	Inspecting,
	Dead,
}

@export var facing := Enums.Direction.EAST:
	set(value):
		facing = value
		if is_node_ready():
			_update_sprite()

@export var BARK_SFX: AudioStream

@export_group("Death")
@export var DEAD_FALL_SPEED: float = 400.0
@export var FALL_SFX: AudioStream

var state := State.Guarding
var is_guarding: bool:
	get():
		return state == State.Guarding
var is_inspecting: bool:
	get():
		return state == State.Inspecting
var is_dead: bool:
	get():
		return state == State.Dead
var inspect_pos: Vector2i

func _ready() -> void:
	_update_sprite()

func _process(delta: float) -> void:
	if is_dead:
		position.y += DEAD_FALL_SPEED * delta

func step() -> void:
	if not is_inspecting:
		return

	# TODO: Take a step towards inspection position.
	# TODO: Once reached, go back to Guarding.

func died() -> void:
	if is_dead:
		return

	# Play sound, flip vertically, start falling.
	AudioManager.play_sfx(FALL_SFX)
	$ShadowSprite.visible = false
	$PugSprite.flip_v = true
	z_index = 15
	state = State.Dead

	# Despawn after a timeout
	await get_tree().create_timer(2.0).timeout
	queue_free()

func attack(new_tile_pos: Vector2i) -> void:
	if is_dead:
		return

	# Play sound
	tile_pos = new_tile_pos
	AudioManager.play_sfx(BARK_SFX)

func inspect(new_inspect_pos: Vector2i) -> void:
	if is_dead:
		return

	$QuestionMarkSprite.visible = true
	state = State.Inspecting
	inspect_pos = new_inspect_pos

func _update_sprite() -> void:
	match facing:
		Enums.Direction.NORTH, Enums.Direction.WEST:
			$PugSprite.play("back")
			if Engine.is_editor_hint():
				$PugSprite.pause()
			# TODO: Play from current anim time? Use `set_frame_and_progress`
		Enums.Direction.SOUTH, Enums.Direction.EAST:
			$PugSprite.play("front")
			if Engine.is_editor_hint():
				$PugSprite.pause()
			# TODO: Play from current anim time?
	if facing in [Enums.Direction.NORTH, Enums.Direction.EAST]:
		$PugSprite.flip_h = true
	else:
		$PugSprite.flip_h = false
