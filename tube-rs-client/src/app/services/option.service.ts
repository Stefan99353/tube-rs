import { Injectable } from '@angular/core';
import { HttpClient, HttpParams } from '@angular/common/http';
import { Observable } from 'rxjs';
import { NewOptionGroup, OptionGroup } from '../models/option-group';
import { InsertDefaultOption, DefaultOption } from '../models/default_option';

@Injectable({
    providedIn: 'root',
})
export class OptionService {
    baseUrl = 'http://localhost:8080/api/';

    constructor(private httpClient: HttpClient) {
        // Get BaseURL
        // this.baseUrl = window.location.href + 'api/';
    }

    public getAllOptionGroups(): Observable<any[]> {
        return this.httpClient.get<any[]>(this.baseUrl + 'options/all');
    }

    public getOptionGroupById(id: string): Observable<OptionGroup> {
        const params = new HttpParams().set('id', id);
        return this.httpClient.get<OptionGroup>(this.baseUrl + 'options/id', {
            params,
        });
    }

    public addOptionGroup(
        group: OptionGroup,
        options: DefaultOption[]
    ): Observable<OptionGroup> {
        return this.httpClient.post<OptionGroup>(this.baseUrl + 'options/add', {
            group,
            options,
        });
    }

    public updateOptionGroup(group: OptionGroup): Observable<OptionGroup> {
        return this.httpClient.put<OptionGroup>(
            this.baseUrl + 'options/update',
            group
        );
    }

    public deleteOptionGroup(id: string): Observable<boolean> {
        const params = new HttpParams().set('id', id);
        return this.httpClient.delete<boolean>(
            this.baseUrl + 'options/delete',
            {
                params,
            }
        );
    }

    // options/group
    public addDefaultOption(option: DefaultOption): Observable<boolean> {
        return this.httpClient.post<boolean>(
            this.baseUrl + 'options/group/add',
            option
        );
    }

    public updateDefaultOption(option: DefaultOption): Observable<boolean> {
        return this.httpClient.put<boolean>(
            this.baseUrl + 'options/group/update',
            option
        );
    }

    public deleteDefaultOption(id: number): Observable<boolean> {
        const params = new HttpParams().set('id', id.toString());
        return this.httpClient.delete<boolean>(
            this.baseUrl + 'options/group/delete',
            {
                params,
            }
        );
    }
}
