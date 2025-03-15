<h1 align=center>Hearthstone Reconnecter</h1>
<p align=center>⌨️炉石传说一键拔线⛓️‍💥</p>
<br/>

本工具通过使游戏断线重连，以跳过漫长的动画时间。

### 原理

使用 Windows 防火墙短暂限制炉石传说出口流量，以营造掉线效果。

### 使用方法

0. 抉择：
   - 克隆本仓库并使用`cargo build --release`手动编译。
   - 从 Release 下载编译好的可执行程序。
1. 运行程序
2. 使用快捷键 ALT+A 即可在任何时候拔线三秒。

### 提示

- 本工具会创建一条名为 HearthstoneReconnecter 的防火墙 outbound 规则。你可以出于强迫症在停止使用本工具后手动删除它，但保留也没有任何影响。
- 如果在拔线期间该工具被关闭，会导致一直处于拔线状态（也就是炉石传说连不上网）。重新拔一次线即可恢复。

### 致谢

本工具的灵感来源于 https://github.com/Curtion/HearthStone-AutoReConn

（但不知为何我没法正常使用上述工具，故干脆重写一份）

相较于上述工具，本工具具有以下特点：

- 简明依赖：本工具仅依赖 windows Crate，通过直接调用 Windows api 高效完成需求。
- 已测试在 Windows 11 上可用。（但未测试是否在 Windows 10 上可用（逃
- 编译出的可执行程序小一点(358KB)。
