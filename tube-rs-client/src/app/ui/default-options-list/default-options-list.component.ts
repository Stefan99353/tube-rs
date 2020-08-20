import { Component, OnInit, Input, Output, EventEmitter } from '@angular/core';

@Component({
    selector: 'app-default-options-list',
    templateUrl: './default-options-list.component.html',
    styleUrls: ['./default-options-list.component.scss'],
})
export class DefaultOptionsListComponent implements OnInit {
    @Input() options: any[] = [];
    public newOption: any = { id: 0, flag: '', val: '', group_id: 0 };

    @Output() add: EventEmitter<any> = new EventEmitter<any>();
    @Output() delete: EventEmitter<any> = new EventEmitter<any>();

    constructor() {}

    ngOnInit(): void {}

    addOption($event): void {
        this.add.emit(JSON.parse(JSON.stringify($event)));
        this.options.push($event);
        this.newOption = { id: 0, flag: '', val: '', group_id: 0 };
    }
    deleteOption($event): void {
        this.delete.emit(JSON.parse(JSON.stringify($event)));
        this.options.splice(this.options.indexOf($event), 1);
    }
}
