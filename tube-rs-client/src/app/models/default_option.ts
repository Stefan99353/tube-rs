export class DefaultOption {
    public id: number;
    public flag: string;
    public val?: string;
    // tslint:disable-next-line: variable-name
    public group_id: number;
}

export class InsertDefaultOption {
    public flag: string;
    public val?: string;
    // tslint:disable-next-line: variable-name
    public group_id: number;
}

export class NewDefaultOption {
    public flag: string;
    public val?: string;
}
