import { Component, OnInit } from '@angular/core';
import { Video, NewVideo } from '../models/video';
import { VideoService } from '../services/video.service';

// Angular Material
import { MatSnackBar } from '@angular/material/snack-bar';
import { MatDialog } from '@angular/material/dialog';
import { AddVideoModalComponent } from '../modals/add-video-modal/add-video-modal.component';
import { VideoOption } from '../models/video-option';

@Component({
    selector: 'app-home',
    templateUrl: './home.component.html',
    styleUrls: ['./home.component.scss'],
})
export class HomeComponent implements OnInit {
    public queuedVideos: Video[] = [];
    public finishedVideos: Video[] = [];

    // Add Video Values
    public addVideoObject: Video;
    public addVideoOptions: VideoOption[];

    constructor(
        // tslint:disable-next-line: variable-name
        private _backendService: VideoService,
        // tslint:disable-next-line: variable-name
        private _snackBar: MatSnackBar,
        public dialog: MatDialog
    ) {}

    ngOnInit(): void {
        this._backendService.getQueuedVideos().subscribe((videos) => {
            this.queuedVideos = videos;
        });
        this._backendService.getFinishedVideos().subscribe((videos) => {
            this.finishedVideos = videos;
        });
    }

    ShowAddModal(): void {
        const dialogRef = this.dialog.open(AddVideoModalComponent, {
            width: '1280px',
        });

        dialogRef.afterClosed().subscribe((result) => {
            if (result) {
                console.log(result);
                this.addVideoObject = {
                    id: result.id,
                    title: result.title,
                    added_at: '',
                    downloaded: false,
                    path: '',
                    quality: result.quality,
                };
                this.addVideoOptions = result.options;
                this.AddVideo();
            }
        });
    }

    AddVideo(): void {
        const today = new Date();
        const date =
            today.getDate() +
            '-' +
            (today.getMonth() + 1) +
            '-' +
            today.getFullYear();

        this.addVideoObject.downloaded = false;
        this.addVideoObject.added_at = date;
        this.addVideoOptions = [];
        this.addVideoObject.path = '';

        this._backendService
            .addVideo(this.addVideoObject, this.addVideoOptions)
            .subscribe((result) => {
                if (result) {
                    this._snackBar.open('Video added', 'Ok', {
                        duration: 2000,
                    });
                    this.queuedVideos.push(this.addVideoObject);
                    this.addVideoObject = new NewVideo();
                } else {
                    this._snackBar.open('Video already added', 'Ok', {
                        duration: 2000,
                    });
                }
            });
    }
}
