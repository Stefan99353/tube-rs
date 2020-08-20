import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';
import { HttpClientModule } from '@angular/common/http';
import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { HomeComponent } from './home/home.component';
import { DaemonSettingsComponent } from './daemon-settings/daemon-settings.component';
import { SettingsComponent } from './settings/settings.component';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';

// Angular Material
import { MatToolbarModule } from '@angular/material/toolbar';
import { MatIconModule } from '@angular/material/icon';
import { MatButtonModule } from '@angular/material/button';
import { MatSidenavModule } from '@angular/material/sidenav';
import { MainNavComponent } from './main-nav/main-nav.component';
import { LayoutModule } from '@angular/cdk/layout';
import { MatExpansionModule } from '@angular/material/expansion';
import { MatListModule } from '@angular/material/list';
import { MatSnackBarModule } from '@angular/material/snack-bar';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { AddVideoModalComponent } from './modals/add-video-modal/add-video-modal.component';
import { MatDialogModule } from '@angular/material/dialog';
import { MatTabsModule } from '@angular/material/tabs';
import { AddOptionGroupModalComponent } from './modals/add-option-group-modal/add-option-group-modal.component';
import { DefaultOptionComponent } from './ui/default-option/default-option.component';
import { MatCardModule } from '@angular/material/card';
import { DefaultOptionsListComponent } from './ui/default-options-list/default-options-list.component';

@NgModule({
    declarations: [
        AppComponent,
        HomeComponent,
        DaemonSettingsComponent,
        MainNavComponent,
        SettingsComponent,
        AddVideoModalComponent,
        AddOptionGroupModalComponent,
        DefaultOptionComponent,
        DefaultOptionsListComponent,
    ],
    imports: [
        BrowserModule,
        AppRoutingModule,
        BrowserAnimationsModule,
        HttpClientModule,
        MatToolbarModule,
        MatIconModule,
        MatButtonModule,
        MatSidenavModule,
        FormsModule,
        ReactiveFormsModule,
        LayoutModule,
        MatListModule,
        MatExpansionModule,
        MatSnackBarModule,
        MatFormFieldModule,
        MatInputModule,
        MatDialogModule,
        MatTabsModule,
        MatCardModule,
    ],
    providers: [],
    bootstrap: [AppComponent],
})
export class AppModule {}
