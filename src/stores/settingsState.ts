import { writable } from 'svelte/store';
import type {AppSettings} from "$lib/models/Settings";

const settingsInit: AppSettings = {
    theme: '',
    language: '',
    installPath: '',
    createDesktopEntry: false,
};

// Load the initial state from localStorage or use a default value
const initialAppState = JSON.parse(localStorage.getItem('settingsState')) || settingsInit;

// Create a writable store with the initial state
export const settingsState = writable(initialAppState);

// Subscribe to changes in the store and update localStorage
settingsState.subscribe(($appState) => {
    localStorage.setItem('settingsState', JSON.stringify($appState));
});
