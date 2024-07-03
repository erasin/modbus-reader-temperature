extends UserManagerView

@onready var user_list: ItemList = %UserList

func _on_update_users() -> void:
	# print("us", self.list)
	user_list.clear()
	for i in self.list:
		user_list.add_item(i)
	pass
