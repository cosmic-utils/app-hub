import {invoke} from "@tauri-apps/api/core";

async function pickDirectory(): Promise<string> {
    try {
        return await invoke<string>('pick_dir_command', {});
    }
    catch (e) {
        console.error(e);
        throw new Error('Failed to pick directory');
    }
}

// Pick an image from the file system using a dialog
async function pickAppImage(): Promise<string> {
    try {
        return await invoke<string>('pick_app_image_command', {});
    }
    catch (e) {
        console.error(e);
        throw new Error('Failed to pick app image');
    }
}

export {
    pickDirectory,
    pickAppImage,
};
