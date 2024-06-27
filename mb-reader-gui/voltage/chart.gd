extends Control

@onready var chart: Chart = $Chart

var f1: Function

func _ready():
	
	var x: Array = range(0, 60, 1)
	var y: PackedFloat32Array = range(0.0, 240.0, 10.0)
	# var y: Array = ArrayOperations.multiply_int(range(0, 240, 10), 1)
	print(x, y)

	var cp: ChartProperties = ChartProperties.new()
	cp.colors.frame = Color("#161a1d")
	cp.colors.background = Color.TRANSPARENT
	cp.colors.grid = Color("#283442")
	cp.colors.ticks = Color("#283442")
	cp.colors.text = Color.WHITE_SMOKE
	cp.draw_bounding_box = false
	cp.title = "电压"
	cp.x_label = "m"
	cp.y_label = "V"
	cp.x_scale = 10
	cp.y_scale = 10
	cp.interactive = true
	
	f1 = Function.new(
		x, y, "Pressure",
		{ 
			color = Color("#36a2eb"), 		# The color associated to this function
			marker = Function.Marker.CIRCLE, 
			type = Function.Type.LINE,
			interpolation = Function.Interpolation.STAIR
		}
	)

	chart.plot([f1], cp)
	
	set_process(false)


var new_val: float = 4.5

func _process(delta: float):
	# This function updates the values of a function and then updates the plot
	new_val += 5
	
	# we can use the `Function.add_point(x, y)` method to update a function
	f1.add_point(new_val, cos(new_val) * 20)
	f1.remove_point(0)
	chart.queue_redraw() # This will force the Chart to be updated


func _on_CheckButton_pressed():
	set_process(not is_processing())
