pub mod com {
    pub mod lssoft {
        // ** 加粗
        // *** 斜体加粗
        // * 斜体
        // ～～
        // N个# 加空格表示 N级标题
        /// -公共mod commmon
        pub mod common {
            use std::fmt::{Debug};

            /// -结构体 People
            ///   - 属性 name,类型 &mut String
            #[derive(Debug)]
            pub struct People<'a> {
                pub name: &'a mut String,
            }

            impl <'a> People<'a>{
               ///  -modify_name函数
               ///   - 功能介绍
               ///     - 根据参数 attr:str 改变传入的stuct的name属性
               ///   - 参数
               ///     - attr: &str
                pub fn modify_name(&mut self, attr: &'a mut String) {
                    self.name = attr;
                }
            }

        }
    }
}