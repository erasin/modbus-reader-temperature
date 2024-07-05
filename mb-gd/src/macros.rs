// #[macro_use]
// extern crate paste;

// 宏定义
#[macro_export]
macro_rules! define_get_node {
    ($fn_name:ident, $unique_name:expr, $node_type:ty) => {
        fn $fn_name(&mut self) -> Gd<$node_type> {
            self.base().get_node_as::<$node_type>($unique_name.as_ref())
        }
    };
}

#[macro_export]
macro_rules! define_get_nodes {
    [$(($fn_name:ident, $unique_name:expr, $node_type:ty)),* $(,)?] => {
        $(
            fn $fn_name(&mut self) -> Gd<$node_type> {
                self.base().get_node_as::<$node_type>($unique_name.as_ref())
            }
        )*
    };
}
