import { NewDefaultOption } from './default_option';

export class OptionGroup {
    public id: number;
    public name: string;
}

export class InsertOptionGroup {
    public name: string;
}

export class NewOptionGroup {
    public name: string;
    // tslint:disable-next-line: variable-name
    public default_options: NewDefaultOption[];
}
