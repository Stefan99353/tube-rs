import { NewVideoOption } from './video-option';

export class Video {
    public id: string;
    public title: string;
    // tslint:disable-next-line: variable-name
    public added_at: string;
    public downloaded: boolean;
    public path: string;
    public quality: string;
}

export class NewVideo {
    public id: string;
    public title: string;
    // tslint:disable-next-line: variable-name
    public added_at: string;
    public downloaded: boolean;
    public path: string;
    public quality: string;
    public options: NewVideoOption[];
}
