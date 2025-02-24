@tool
class_name Player
extends TileNode

@export var DEAD_FALL_SPEED: float = 400.0

@export_category("Sprites")
@export var FRONT_SPRITE: Texture2D
@export var BACK_SPRITE: Texture2D
@export var MAIL_FRONT_SPRITE: Texture2D
@export var MAIL_BACK_SPRITE: Texture2D

@export_category("Movement")
@export var facing := Enums.Direction.EAST:
	set(value):
		facing = value
		if is_node_ready():
			_update_sprite()

var is_dead := false
var has_mail := false

func _ready() -> void:
	_update_sprite()

func _process(delta: float) -> void:
	if is_dead:
		position.y += DEAD_FALL_SPEED * delta

func _unhandled_key_input(event: InputEvent) -> void:
	var dir := Vector2i.ZERO
	if event.is_action_pressed("ui_left"):
		dir = Vector2i.LEFT
		facing = Enums.Direction.WEST
	elif event.is_action_pressed("ui_right"):
		dir = Vector2i.RIGHT
		facing = Enums.Direction.EAST
	elif event.is_action_pressed("ui_up"):
		dir = Vector2i.UP
		facing = Enums.Direction.NORTH
	elif event.is_action_pressed("ui_down"):
		dir = Vector2i.DOWN
		facing = Enums.Direction.SOUTH

	if dir != Vector2i.ZERO:
		# TODO: Check if there is a cell at tile_pos + dir
		var new_pos := tile_pos + dir
		var grid: TileMapLayer = %GroundTiles
		if grid.get_cell_source_id(new_pos) >= 0:
			tile_pos = new_pos
			_check_overlaps()

func _check_overlaps() -> void:
	var all_pugs := get_tree().get_nodes_in_group("pugs")
	for node in all_pugs:
		var pug := node as Pug
		if not pug:
			continue
		if pug.tile_pos + Utils.dir_to_vec(pug.facing) == tile_pos:
			died()
			pug.attack(tile_pos)
			return
		elif pug.tile_pos == tile_pos:
			pug.died()

	var all_mail := get_tree().get_nodes_in_group("mail")
	for node in all_mail:
		var mail := node as Mail
		if mail and mail.tile_pos == tile_pos:
			# Pick up mail piece!
			has_mail = true
			_update_sprite()
			mail.pick_up()

	if has_mail:
		var all_mailboxes := get_tree().get_nodes_in_group("mailbox")
		for node in all_mailboxes:
			var mailbox := node as Mailbox
			if mailbox and mailbox.tile_pos == tile_pos:
				var level := get_tree().current_scene as Level
				if level:
					# TODO: Play win anim and SFX!
					level._start_next_level()


func _update_sprite() -> void:
	match facing:
		Enums.Direction.NORTH, Enums.Direction.WEST:
			if has_mail:
				$FoxSprite.texture = MAIL_BACK_SPRITE
			else:
				$FoxSprite.texture = BACK_SPRITE
		Enums.Direction.SOUTH, Enums.Direction.EAST:
			if has_mail:
				$FoxSprite.texture = MAIL_FRONT_SPRITE
			else:
				$FoxSprite.texture = FRONT_SPRITE
	var x_offset = absf($FoxSprite.position.x)
	if facing in [Enums.Direction.NORTH, Enums.Direction.EAST]:
		$FoxSprite.flip_h = true
		$FoxSprite.position.x = x_offset
	else:
		$FoxSprite.flip_h = false
		$FoxSprite.position.x = -x_offset

func died() -> void:
	# TODO: Play sound, flip vertically, start falling.
	$ShadowSprite.visible = false
	$FoxSprite.flip_v = true
	z_index = 20
	set_process_unhandled_input(false)
	is_dead = true

	await get_tree().create_timer(2.0).timeout
	queue_free()
