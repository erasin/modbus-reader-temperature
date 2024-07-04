extends ProgramsView

@onready var items_list: ItemList = %TaskItems
@onready var task_list: ItemList = %TaskList

func _on_update_task_item_list() -> void:
	items_list.clear()
	for i in self.task_items_str.size():
		var str = self.task_items_str[i]
		items_list.add_item(str)
		if i == 0:
			items_list.set_item_custom_bg_color(0, Color(0.7, 0.7, 0.7))  # 设置表头背景颜色
			
	pass

func _on_update_task_list() -> void:
	task_list.clear()
	for str in self.task_list_str:
		task_list.add_item(str)
	pass
