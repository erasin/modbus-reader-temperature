extends UserManagerView

@onready var user_list: ItemList = %UserList

func _on_update_users() -> void:
	var sle = -1;
	var sle_txt = ""
	
	var sls = user_list.get_selected_items()
	if len(sls) > 0:
		sle = sls[0]
		sle_txt = user_list.get_item_text(sle)
	
	user_list.clear()
	for index in self.list.size():
		var item = self.list[index]
		user_list.add_item(item)
		if item == sle_txt:
			sle = index

	if sle >= 0 :
		user_list.select(sle)
		
	pass
