import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { AddOptionGroupModalComponent } from './add-option-group-modal.component';

describe('AddOptionGroupModalComponent', () => {
  let component: AddOptionGroupModalComponent;
  let fixture: ComponentFixture<AddOptionGroupModalComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ AddOptionGroupModalComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(AddOptionGroupModalComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
