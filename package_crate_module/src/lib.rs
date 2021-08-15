// 默认约定src/lib.rs是library crate的crate root，当存在main.rs时，说明这个package中有一个library crate
// crate名与package名相同

// mod关键字定义mod
mod front_of_house {
    // 默认情况下，函数，方法，struct, enum, 模块，常量都是私有的
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // crate root文件会创建一个隐式的mod,并且名称固定为crate
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // super用来访问父级模块路径中的内容，类似文件系统中的..
        super::serve_order();
    }

    fn cook_order() {}
}
