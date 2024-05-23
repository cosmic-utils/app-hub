export class AppDatabase {
    appList: RemoteAppInfo[];

    constructor() {
        this.appList = [];
    }
}

export class RemoteAppInfo {
    name: string;
    path: string;
    sha: string;
    size: number;
    url: string;
    htmlUrl: string;
    gitUrl: string;
    downloadUrl: string;
    typeField: string;
    links: Links;

    constructor() {
        this.name = '';
        this.path = '';
        this.sha = '';
        this.size = 0;
        this.url = '';
        this.htmlUrl = '';
        this.gitUrl = '';
        this.downloadUrl = '';
        this.typeField = '';
        this.links = new Links();
    }
}

export class Links {
    selfField: string;
    git: string;
    html: string;

    constructor() {
        this.selfField = '';
        this.git = '';
        this.html = '';
    }
}