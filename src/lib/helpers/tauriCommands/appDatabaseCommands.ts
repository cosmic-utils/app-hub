import {invoke} from "@tauri-apps/api/core";
import type {AppDatabase} from "$lib/models/AppDatabase";

// update local apps database
async function updateAppsDatabase(): Promise<string> {
    try {
        return await invoke<string>('update_database_command', {})
    }
    catch (e) {
        console.error(e);
        throw new Error('Failed to install app image');
    }
}

async function readAppsDatabase(): Promise<AppDatabase> {
    try {
        return await invoke<AppDatabase>('get_app_list_command', {})
    }
    catch (e) {
        console.error(e);
        throw new Error('Failed to install app image');
    }
}

async function installAppFromRemote(download_url: string, appName: string): Promise<string> {
    try {
        return await invoke<string>('install_app_from_remote_command', {downloadUrl: download_url, appName: appName});
    }
    catch (e) {
        console.error(e);
        throw new Error('Failed to install app image');
    }
}

export {
    updateAppsDatabase,
    readAppsDatabase,
    installAppFromRemote
}