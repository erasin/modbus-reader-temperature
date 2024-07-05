extends ProgramsView

@onready var items_list: ItemList = %TaskItems
@onready var task_list: ItemList = %TaskList

func _on_update_task_item_list() -> void:
	var sle = -1
	var sls = items_list.get_selected_items()
	if len(sls) > 0:
		sle = sls[0]
	
	items_list.clear()
	for index in self.task_items_str.size():
		var item = self.task_items_str[index]
		items_list.add_item(item)
		# 设置表头背景颜色
		if index == 0:
			items_list.set_item_custom_bg_color(0, Color(0.7, 0.7, 0.7))

	if sle > 0:
		items_list.select(sle)
		
	pass

func _on_update_task_list() -> void:
	var sle = -1;
	var sle_txt = ""
	
	var sls = task_list.get_selected_items()
	if len(sls) > 0:
		sle = sls[0]
		sle_txt = task_list.get_item_text(sle)
	
	task_list.clear()
	for index in self.task_list_str.size():
		var item = self.task_list_str[index]
		task_list.add_item(item)
		if item == sle_txt:
			sle = index

	if sle >= 0 :
		task_list.select(sle)

	pass
