export interface App {
    name: string;
    iconBase64: string;
    appPath: string;
}

export interface AppList {
    apps: App[];
}