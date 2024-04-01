import {invoke} from "@tauri-apps/api/core";
import type {AppList} from "$lib/models/Applist";

// Install an app image in the target directory
async function installAppImage(
    appPath: string,
    iconPath: string,
    appName: string,
    appType: string,
    terminal: boolean,
    appDescription: string,
    noSandBox: boolean,
): Promise<string> {
    try {
        return await invoke<string>('install_app', {
            requestInstallation: {
                filePath: appPath,
                iconPath: iconPath,
                appName: appName,
                appType: appType,
                terminal: terminal,
                appDescription: appDescription,
                noSandbox: noSandBox
            }
        })
    }
    catch (e) {
        console.error(e);
        throw new Error('Failed to install app image');
    }
}

// Get the list of installed apps
async function installedAppsList(): Promise<AppList> {
    try {
        return await invoke<AppList>("read_app_list");
    }
    catch (e) {
        console.error(e);
        throw new Error('Failed to read app list');
    }
}

export {
    installAppImage,
    installedAppsList
}