export interface App {
    name: string;
    iconBase64: string;
    appPath: string;
    version: string;
    category: string;
}

export interface AppList {
    apps: App[];
}