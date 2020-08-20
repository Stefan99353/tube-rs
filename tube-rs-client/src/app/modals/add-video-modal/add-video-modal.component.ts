import { Component, OnInit } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { NewVideo } from '../../models/video';
import { OptionGroup } from '../../models/option-group';
import { VideoService } from '../../services/video.service';
import { MatSnackBar } from '@angular/material/snack-bar';
import { OptionService } from 'src/app/services/option.service';

@Component({
    selector: 'app-add-video-modal',
    templateUrl: './add-video-modal.component.html',
    styleUrls: ['./add-video-modal.component.scss'],
})
export class AddVideoModalComponent implements OnInit {
    public video = new NewVideo();

    public optionGroups: any[] = [];

    public currentOption: any[] = [];

    constructor(
        public dialogRef: MatDialogRef<AddVideoModalComponent>,
        // tslint:disable-next-line: variable-name
        private _backendService: VideoService,
        // tslint:disable-next-line: variable-name
        private _snackBar: MatSnackBar,
        // tslint:disable-next-line: variable-name
        private _optionService: OptionService
    ) {}

    ngOnInit(): void {
        this._optionService.getAllOptionGroups().subscribe((groups) => {
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

    OptionGroupChanged($event): void {
        this.video.options = this.optionGroups[$event.index].options;
        console.log(this.video);
    }

    onNoClick(): void {
        this.dialogRef.close();
    }

    onOkClick(): void {
        if (this.Validate()) {
            this.dialogRef.close(this.video);
        } else {
            this._snackBar.open('Invalid Input', 'Ok', {
                duration: 2000,
            });
        }
    }

    Validate(): boolean {
        if (this.isBlank(this.video.id)) {
            return false;
        }
        if (this.isBlank(this.video.title)) {
            return false;
        }
        if (this.isBlank(this.video.quality)) {
            return false;
        }
        return true;
    }

    onIdChange(addVideoForm: any): void {
        const regexp = new RegExp('[^s?&"]{11}');
        if (addVideoForm.value.id && regexp.test(addVideoForm.value.id)) {
            this._snackBar.open('Searching for Video Title...', 'Ok', {
                duration: 2000,
            });

            this._backendService
                .getVideoInfoById(addVideoForm.value.id)
                .subscribe((result) => {
                    console.log(result);
                    this._snackBar.open('Found Video Title', 'Ok', {
                        duration: 2000,
                    });

                    this.video.title = result.title;
                });
        }
    }

    isBlank(str): boolean {
        return !str || /^\s*$/.test(str);
    }
}
