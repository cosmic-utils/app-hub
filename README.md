<p align="center">
  <img src="./src-tauri/icons/appHubLogo.png" alt="Logo">
</p>

# 🚀 AppHub

AppHub is a Linux desktop application that simplifies the installation and management of .appImage packages through an intuitive graphical interface. Additionally, it provides the ability to easily uninstall applications installed via AppImage.

## 📸 Screenshots
<p align="center">
  <img src="./screenshots/install.png" alt="Screenshot 1">
</p>

More screenshots available in the [screenshots](screenshots) folder.

## 💻 System Requirements
- **Operating System:** Linux*

Note*: The application is currently tested only on Fedora workstation. 
However, it should work on any Linux distribution that supports the AppImage format.
Distribution-specific packages will be provided in the future.

## 💿 Installation Modes
The project supports three installation modes:
1. **.appImage**: Download the .appImage file from the GitHub repository and execute it directly.
2. **.deb**: Download the .deb package from the GitHub repository and install it using your system's package manager.
3. **.rpm**: Download the .rpm package from the GitHub repository and install it using your system's package manager.

### Installation from Source

#### Prerequisites
- **Node.js**: The project is built using Node.js, so you must have it installed on your system.
- **pnpm**: The project uses pnpm as the package manager.
- **Tauri**: The project uses Tauri as the framework for building the desktop application.

#### Steps

To install the application from source, follow these steps:

```bash
# Clone the repository
git clone https://github.com/francesco-gaglione/AppHub.git

# Change directory to the project folder
cd AppHub

# Install the required dependencies
pnpm install

# Build the project
pnpm tauri build

# Install the application
sudo dkpg -i ./target/release/bundle/apphub_<version>_amd64.deb
```
Note: Replace `<version>` with the version number of the application.

## 🌟 Key Features
- Simplified installation of .appImage packages.
- Management of applications installed via AppImage.
- Intuitive graphical interface for easy navigation.

## 📖 Usage
Currently, the usage of the application is not documented. However, detailed guides will be provided once the project is more stable and structured.

## 📝 License
The project is currently released under the MIT license. Please refer to the [LICENSE](LICENSE) file for more information.

Note: The license may change in the future due to the use of third-party libraries and frameworks.

## 🤝 Contributing
AppHub is a project developed in spare time, and contributions are welcome! However, there are currently no specific contribution guidelines. They will be defined once the project is more stable and structured.

## ⚠️ Project Status
The project is currently in its initial development phase. There are still many bugs to be fixed and many features to be implemented. Please be patient as we work to improve and refine the application.

## ⚠️ Experimental
The project is in experimental mode and is not yet ready for production use. 
We use Tauri 2.0, which is still in beta.
Please use it at your own risk.

## 📌 Additional Notes
Please note that this project is developed in spare time and is not a full-time job. Contributions are appreciated and help grow the project.

## 🚀 Roadmap

Roadmap for the project is not fully defined yet. However, the following features are planned for future releases:

- [ ] Add support for other Linux distributions.
- [ ] Add installation support for other application bundle formats in addition to AppImage (e.g., Flatpak, Snap, .deb, .rpm).
- [ ] Add support for other desktop environments.
- [x] Automaticaly extract AppImage icon

## 💖 Support

If you like the project and want to support it, you can do so by donating:

- Bitcoin: `bc1qkxt5nvyanh59pa4lg3xpq5znmwhxsnld6jtkje`
- Polkadot: `15e64wTXGgZH1vBrnAXgz7qTiSJLPA5j9APL4nfDFMbQ6inE`
- Cosmos: `cosmos1asqs7p07yas0523k7eegyy0vq0zupw0cxfwpn4`
- Solana: `H1qkZg3W92gwQQh3cthAdf3ovMJ7ExnKJ217hZg8MJBQ`
- Polygon: `0x976b2f699E420027a845Ac63B98b3B78d798e78A`

---

For further information, please contact the [maintainer of the project](mailto:francesco.gaglione.p@gmail.com).
