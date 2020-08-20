import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { DefaultOptionsListComponent } from './default-options-list.component';

describe('DefaultOptionsListComponent', () => {
  let component: DefaultOptionsListComponent;
  let fixture: ComponentFixture<DefaultOptionsListComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ DefaultOptionsListComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(DefaultOptionsListComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
