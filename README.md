# SIMDays 📅

手机卡重置倒计时小工具。如果你的手机卡每隔 N 天需要手动重置（如 180 天），这个小工具会帮你倒计时，临期时自动弹窗提醒。支持添加任意多张卡，每张卡独立倒计时、独立提醒。

基于 **Tauri 2 + Rust** 构建，前端为纯静态 HTML/CSS/JS，无任何 npm 依赖。

## 使用

直接运行 `SIMDays.exe`（免安装绿色单文件）：

1. **添加卡**：点「＋」→ 填卡名称、图标、上次重置日期（默认今天）和重置周期（默认 180 天）
2. **重置后**：在卡片视图点「✓ 今天已重置」，从当天重新推算下一次重置日期
3. **两种视图**：顶栏「☰」列表视图（按紧急度排序，一眼看清每张卡）/「▢」卡片视图（大字倒计时，左右切换），偏好会被记住
4. **编辑/删除卡**：卡片视图 →「编辑这张卡」；删除需在 3 秒内点两次确认
5. **改设置**：顶栏「⚙」→ 开机自启

## 功能

- 多卡管理：每张卡独立的上次重置日期 / 重置周期 / 提前提醒天数 / emoji 图标
- 大字倒计时 + 三态颜色：正常（绿）/ 临期（黄）/ 已过期（红）
- 置顶弹窗提醒：进入提前提醒期（默认到期前 3 天，可设 0-30）后每天弹一次；多张卡同时临期时一窗列出，可逐卡重置
- 系统托盘常驻：图标显示所有卡中最差的状态，悬停显示最紧急一张的倒计时；左键单击显示/隐藏窗口，右键菜单可打开 / 退出
- 点窗口 × 只是隐藏到托盘，退出请用托盘右键菜单「退出」
- 开机自启：设置里可开关（默认关）
- 单实例：重复启动只唤起已有窗口
- 旧版单卡数据升级后自动迁移为一张卡，不丢数据
- 数据保存在 `%APPDATA%\com.simdays.app\config.json`，删除即恢复初始状态

## 开发

```bash
# 环境：Rust + cargo-tauri（cargo install tauri-cli）+ WebView2
node tools/gen_icons.js          # 生成图标源图（纯 Node，无依赖）
cargo tauri icon tools/app-icon.png
cd src-tauri
cargo build                      # 调试构建
cargo build --release            # 发布构建
# 产物：src-tauri/target/release/simdays.exe
```

## 项目结构

```
src-ui/            前端（index.html 主界面 / remind.html 提醒弹窗，纯静态）
src-tauri/
  src/state.rs     多卡配置读写、状态计算与旧数据迁移（正常/临期/到期/超期）
  src/commands.rs  Tauri 命令（get_cards/add_card/reset_card/save_card/delete_card/自启）
  src/tray.rs      托盘图标、菜单与悬停提示（聚合所有卡的最差状态）
  src/remind.rs    置顶提醒弹窗逻辑（每卡每天最多一次）
  src/lib.rs       应用入口、后台检查线程（每 30 分钟）、关窗隐藏
tools/gen_icons.js 图标生成脚本（SDF 绘制，无第三方依赖）
```
