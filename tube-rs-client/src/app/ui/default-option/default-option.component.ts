import { Component, OnInit, Input, Output, EventEmitter } from '@angular/core';

@Component({
    selector: 'app-default-option',
    templateUrl: './default-option.component.html',
    styleUrls: ['./default-option.component.scss'],
})
export class DefaultOptionComponent implements OnInit {
    @Input() option: any;
    @Input() readOnly = true;

    @Output() action: EventEmitter<any> = new EventEmitter<any>();

    constructor() {}

    ngOnInit(): void {}

    onAction(): void {
        this.action.emit(this.option);
    }
}
