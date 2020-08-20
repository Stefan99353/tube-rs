import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { HomeComponent } from './home/home.component';
import { DaemonSettingsComponent } from './daemon-settings/daemon-settings.component';
import { SettingsComponent } from './settings/settings.component';

const routes: Routes = [
    { path: 'home', component: HomeComponent },
    { path: 'daemon', component: DaemonSettingsComponent },
    { path: 'settings', component: SettingsComponent},
    { path: '**', component: HomeComponent },
];

@NgModule({
    imports: [RouterModule.forRoot(routes)],
    exports: [RouterModule],
})
export class AppRoutingModule {}
