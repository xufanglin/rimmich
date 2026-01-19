# rImmich

[English](#english) | [中文](#中文)

## English

rImmich is a lightweight Immich desktop upload client built with Rust and Dioxus. It aims to provide a simple and efficient way to upload your photos and videos to your self-hosted Immich server.

### Features

*   **Multi-language Support**: Support English and Chinese interface languages.
*   **Multi-user Support**: Manage multiple Immich users' API Keys and switch upload identities at any time.
*   **Concurrent Upload**: Support custom concurrent upload count to fully utilize network bandwidth.
*   **Intuitive Settings**: Graphical interface for configuring server address and user information.
*   **File Support**: Support common image and video formats (jpg, jpeg, png, heic, webp, mp4, mov).
*   **Configuration Persistence**: Configuration files are automatically saved locally (`~/.immich/config.toml`).

### Tech Stack

*   **Language**: [Rust](https://www.rust-lang.org/)
*   **UI Framework**: [Dioxus](https://dioxuslabs.com/) (Desktop)
*   **HTTP Client**: [Reqwest](https://github.com/seanmonstar/reqwest)
*   **Async Runtime**: [Tokio](https://tokio.rs/)
*   **File Dialog**: [rfd](https://github.com/PolyMeilex/rfd)

### Quick Start

#### Prerequisites

*   [Rust](https://www.rust-lang.org/tools/install) environment installed (Cargo).
*   A running [Immich](https://immich.app/) server.

#### Running Development Version

1.  Clone the repository:
    ```bash
    git clone <repository-url>
    cd rimmich
    ```

2.  Run the program:
    ```bash
    # Recommended: use dx (Dioxus CLI) for hot reload support
    cargo install dioxus-cli
    dx serve --desktop
    
    # Or use cargo directly
    cargo run
    ```

### Build Guide

#### Package macOS App

Use the following command to build macOS app bundle (.app) and disk image (.dmg):

```bash
dx bundle --desktop \
    --package-types "macos" \
    --package-types "dmg"
```

#### Automated Deployment (GitHub Actions)

This project includes a pre-configured GitHub Actions workflow (`.github/workflows/release.yml`). When you push a tag starting with `v` to the repository (e.g., `v0.1.0`), it will automatically trigger the build process and publish the generated `.dmg` installer to the GitHub Releases page.

**Trigger release process:**

```bash
git tag v0.1.0
git push origin v0.1.0
```

### Usage Instructions

1.  **Initial Setup**:
    *   After launching the app, click the **Settings** (⚙️) icon in the top right corner.
    *   Select your preferred **Language** (English/中文) from the dropdown menu.
    *   Enter your Immich server address in **Server URL** (e.g., `http://192.168.1.100:2283`) and save.
    *   In **User Management**, enter the username and corresponding API Key (generated in Immich web interface), then click add.
    *   You can set a **Default User** to be automatically selected when opening the app next time.

2.  **Upload Files**:
    *   Return to the main page and select the upload account from the dropdown.
    *   Click **Select Files** to choose the photos or videos you want to upload.
    *   Click **Start Upload**.
    *   The status bar at the bottom will show the current upload progress and results.

### FAQ

#### macOS Shows App is Damaged or Cannot be Opened

macOS automatically marks files downloaded from browsers with the `com.apple.quarantine` flag, which may prevent the app from running normally or show it as damaged.

**Solution:**

Open Terminal and execute the following command to clear the flag:

```bash
# If the app is in the Applications folder
sudo xattr -r -d com.apple.quarantine /Applications/Rimmich.app

# Or specify the specific path where the app is located
sudo xattr -r -d com.apple.quarantine /path/to/Rimmich.app
```

### Configuration File

Configuration file is located in the user's home directory:
*   **macOS / Linux**: `~/.immich/config.toml`

### License

[MIT License](LICENSE)

---

## 中文

rImmich 是一个基于 Rust 和 Dioxus 开发的轻量级 Immich 桌面上传客户端。它旨在提供一个简单、高效的方式将您的照片和视频上传到自托管的 Immich 服务器。

### 功能特性

*   **多语言支持**: 支持中文和英文界面语言。
*   **多用户支持**: 可以管理多个 Immich 用户的 API Key，并随时切换上传身份。
*   **并发上传**: 支持自定义并发上传数量，充分利用网络带宽。
*   **直观的设置**: 图形化界面配置服务器地址和用户信息。
*   **文件支持**: 支持常见的图片和视频格式 (jpg, jpeg, png, heic, webp, mp4, mov)。
*   **配置持久化**: 配置文件自动保存在本地 (`~/.immich/config.toml`)。

### 技术栈

*   **语言**: [Rust](https://www.rust-lang.org/)
*   **UI 框架**: [Dioxus](https://dioxuslabs.com/) (Desktop)
*   **HTTP 客户端**: [Reqwest](https://github.com/seanmonstar/reqwest)
*   **异步运行**: [Tokio](https://tokio.rs/)
*   **文件对话框**: [rfd](https://github.com/PolyMeilex/rfd)

### 快速开始

#### 前置要求

*   已安装 [Rust](https://www.rust-lang.org/tools/install) 环境 (Cargo)。
*   一个运行中的 [Immich](https://immich.app/) 服务器。

#### 运行开发版本

1.  克隆仓库：
    ```bash
    git clone <repository-url>
    cd rimmich
    ```

2.  运行程序：
    ```bash
    # 推荐使用 dx (Dioxus CLI) 运行，支持热重载
    cargo install dioxus-cli
    dx serve --desktop
    
    # 或者直接使用 cargo
    cargo run
    ```

### 构建指南

#### 打包 macOS 应用

使用以下命令构建 macOS 应用包 (.app) 和磁盘镜像 (.dmg):

```bash
dx bundle --desktop \
    --package-types "macos" \
    --package-types "dmg"
```

#### 自动化部署 (GitHub Actions)

本项目包含一个预配置的 GitHub Actions workflow (`.github/workflows/release.yml`)。当您向仓库推送以 `v` 开头的标签时（例如 `v0.1.0`），它会自动触发构建流程，并将生成的 `.dmg` 安装包发布到 GitHub Releases 页面。

**触发发布流程：**

```bash
git tag v0.1.0
git push origin v0.1.0
```

### 使用说明

1.  **初次设置**:
    *   启动应用后，点击右上角的 **设置** (⚙️) 图标。
    *   在下拉菜单中选择您偏好的 **语言** (English/中文)。
    *   在 **服务器 URL** 中输入您的 Immich 服务器地址 (例如 `http://192.168.1.100:2283`) 并保存。
    *   在 **用户管理** 中，输入用户名和对应的 API Key (在 Immich web 端生成的 API Key)，点击添加。
    *   您可以设置一个 **默认用户**，下次打开应用时自动选中。

2.  **上传文件**:
    *   回到主页，在下拉框中选择要使用的上传账号。
    *   点击 **选择文件**，选择您想要上传的照片或视频。
    *   点击 **开始上传**。
    *   底部的状态栏会显示当前的上传进度和结果。

### 常见问题

#### macOS 提示应用已损坏或无法打开

macOS 会自动给从浏览器下载的文件打上 `com.apple.quarantine` (隔离) 标记，这可能导致应用无法正常运行或提示已损坏。

**解决方法：**

打开终端 (Terminal)，执行以下命令清除该标记：

```bash
# 如果应用位于应用程序文件夹
sudo xattr -r -d com.apple.quarantine /Applications/Rimmich.app

# 或者指定应用所在的具体路径
sudo xattr -r -d com.apple.quarantine /path/to/Rimmich.app
```

### 配置文件

配置文件位于用户主目录下：
*   **macOS / Linux**: `~/.immich/config.toml`

### 许可证

[MIT License](LICENSE)
