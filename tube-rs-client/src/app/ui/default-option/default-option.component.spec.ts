import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { DefaultOptionComponent } from './default-option.component';

describe('DefaultOptionComponent', () => {
  let component: DefaultOptionComponent;
  let fixture: ComponentFixture<DefaultOptionComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ DefaultOptionComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(DefaultOptionComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
