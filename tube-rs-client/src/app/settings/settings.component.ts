import { Component, OnInit } from '@angular/core';
import { OptionService } from '../services/option.service';
import { OptionGroup } from '../models/option-group';

import { MatDialog } from '@angular/material/dialog';
import { AddOptionGroupModalComponent } from '../modals/add-option-group-modal/add-option-group-modal.component';
import { Observable } from 'rxjs';

@Component({
    selector: 'app-settings',
    templateUrl: './settings.component.html',
    styleUrls: ['./settings.component.scss'],
})
export class SettingsComponent implements OnInit {
    public optionGroups: any[] = [];

    public observablesToExecuteOnSave: Observable<any>[] = [];

    constructor(
        private optionService: OptionService,
        public dialog: MatDialog
    ) {}

    ngOnInit(): void {
        this.optionService.getAllOptionGroups().subscribe((groups) => {
            if (groups) {
                this.optionGroups = groups.map((group) => {
                    return {
                        id: group[0].id,
                        name: group[0].name,
                        options: group[1],
                    };
                });
            }
        });
    }

    ShowAddOptionGroupModal(): void {
        const dialogRef = this.dialog.open(AddOptionGroupModalComponent, {
            width: '1280px',
        });

        dialogRef.afterClosed().subscribe((result) => {
            if (result) {
                console.log(result);
            }
        });
    }

    SaveOptionGroups(): void {
        this.observablesToExecuteOnSave.forEach((observable) => {
            observable.subscribe((result) => {
                if (!result) {
                    console.log('Something went wrong. Please reload page!');
                }
            });
        });
    }

    addOptionToGroup($event: any, groupId: any): void {
        $event.group_id = groupId;
        this.observablesToExecuteOnSave.push(
            this.optionService.addDefaultOption($event)
        );
    }

    deleteOptionToGroup($event: any, groupId: any): void {
        $event.group_id = groupId;
        this.observablesToExecuteOnSave.push(
            this.optionService.deleteDefaultOption($event.id)
        );
    }
}
