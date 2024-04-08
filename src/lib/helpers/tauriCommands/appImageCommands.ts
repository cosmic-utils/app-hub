import {invoke} from "@tauri-apps/api/core";
import type {App, AppList} from "$lib/models/Applist";

// Install an app image in the target directory
async function installAppImage(
    appPath: string,
    noSandBox: boolean,
): Promise<string> {
    try {
        return await invoke<string>('install_app', {
            requestInstallation: {
                filePath: appPath,
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

// Uninstall an app
async function uninstallApp(app: App): Promise<boolean> {
    try {
        return await invoke<boolean>("uninstall_app", {app: app});
    }
    catch (e) {
        console.error(e);
        throw new Error('Failed to read app list');
    }
}

export {
    installAppImage,
    installedAppsList,
    uninstallApp
}