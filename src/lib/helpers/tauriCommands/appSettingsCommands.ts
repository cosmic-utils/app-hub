import {invoke} from "@tauri-apps/api/core";
import type {AppSettings} from "$lib/models/Settings";

// Save the settings in the configuration file
async function saveSettings(
    theme: string,
    language: string,
    installPath: string | undefined,
) {
    try {
        await invoke("save_settings_command", {
            settings: {
                theme,
                language,
                installPath,
            }
        });
    }
    catch (e) {
        console.error(e);
        throw new Error('Failed to save settings');
    }
}

async function getSettings() {
    try {
        const settings: AppSettings = await invoke<AppSettings>("read_settings_command");
        if (settings){
            return settings;
        }
        else {
            throw new Error('Failed to read settings');
        }
    } catch (error) {
        console.error('Error reading settings', error);
        throw error;
    }
}

export {
    saveSettings,
    getSettings
};

