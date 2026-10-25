// 无条件使用 windows 子系统：debug 版双击运行时也不能弹出控制台窗口
// （控制台子系统下用户关掉终端标签页会把整个应用一起杀掉）。
// 该属性仅在 Windows 生效，其他平台直接忽略；Android 构建不编译 main.rs。
#![windows_subsystem = "windows"]

fn main() {
    simdays_lib::run()
}
