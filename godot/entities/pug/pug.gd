@tool
class_name Pug
extends TileNode

@export var facing := Enums.Direction.EAST:
	set(value):
		facing = value
		if is_node_ready():
			_update_sprite()

@export var DEAD_FALL_SPEED: float = 400.0

var is_dead := false

func _ready() -> void:
	_update_sprite()

func _process(delta: float) -> void:
	if is_dead:
		position.y += DEAD_FALL_SPEED * delta

func died() -> void:
	# TODO: Play sound, flip vertically, start falling.
	$ShadowSprite.visible = false
	$PugSprite.flip_v = true
	z_index = 15
	is_dead = true

	# Despawn after a timeout
	await get_tree().create_timer(2.0).timeout
	queue_free()

func attack(new_tile_pos: Vector2i) -> void:
	# TODO: Play sound
	tile_pos = new_tile_pos

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
