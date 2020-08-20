import { Injectable } from '@angular/core';
import { HttpClient, HttpParams } from '@angular/common/http';
import { Observable } from 'rxjs';

import { NewVideo, Video } from '../models/video';
import { VideoOption } from '../models/video-option';

@Injectable({
    providedIn: 'root',
})
export class VideoService {
    baseUrl = 'http://localhost:8080/api/';

    constructor(private httpClient: HttpClient) {
        // Get BaseURL
        // this.baseUrl = window.location.href + 'api/';
    }

    public getAllVideos(): Observable<Video[]> {
        return this.httpClient.get<Video[]>(this.baseUrl + 'videos/all');
    }

    public getQueuedVideos(): Observable<Video[]> {
        return this.httpClient.get<Video[]>(this.baseUrl + 'videos/queued');
    }

    public getFinishedVideos(): Observable<Video[]> {
        return this.httpClient.get<Video[]>(this.baseUrl + 'videos/finished');
    }

    public getVideoById(id: string): Observable<Video> {
        const params = new HttpParams().set('id', id);
        return this.httpClient.get<Video>(this.baseUrl + 'videos/id', {
            params,
        });
    }

    public getVideoInfoById(id: string): Observable<Video> {
        const params = new HttpParams().set('id', id);
        return this.httpClient.get<Video>(this.baseUrl + 'videos/info', {
            params,
        });
    }

    public addVideo(video: Video, options: VideoOption[]): Observable<Video> {
        return this.httpClient.post<Video>(this.baseUrl + 'videos/add', [
            video,
            options,
        ]);
    }

    public updateVideo(video: Video): Observable<Video> {
        return this.httpClient.put<Video>(
            this.baseUrl + 'videos/update',
            video
        );
    }

    public deleteVideo(id: string): Observable<boolean> {
        const params = new HttpParams().set('id', id);
        return this.httpClient.delete<boolean>(this.baseUrl + 'videos/delete', {
            params,
        });
    }
}
