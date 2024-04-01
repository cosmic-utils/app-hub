import {invoke} from "@tauri-apps/api/core";

// Save the settings in the configuration file
async function saveSettings(
    theme: string,
    language: string,
    installPath: string | undefined,
    createDesktopEntry: boolean,
) {
    try {
        await invoke("save_settings", {
            settings: {
                theme,
                language,
                installPath,
                createDesktopEntry
            }
        });
    }
    catch (e) {
        console.error(e);
        throw new Error('Failed to save settings');
    }
}

export {
    saveSettings
};

