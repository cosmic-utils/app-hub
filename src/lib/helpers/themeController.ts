import {themes} from "$lib/themes";

export function set_theme(theme: string) {
    if (themes.includes(theme)) {
        const one_year = 60 * 60 * 24 * 365
        window.localStorage.setItem('theme', theme)
        document.cookie = `theme=${theme}; max-age=${one_year}; path=/;`
        document.documentElement.setAttribute('data-theme', theme)
    }
}