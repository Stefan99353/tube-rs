import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { DaemonSettingsComponent } from './daemon-settings.component';

describe('DaemonSettingsComponent', () => {
  let component: DaemonSettingsComponent;
  let fixture: ComponentFixture<DaemonSettingsComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ DaemonSettingsComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(DaemonSettingsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
