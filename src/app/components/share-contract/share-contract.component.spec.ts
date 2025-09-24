import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ShareContractComponent } from './share-contract.component';

describe('ShareContractComponent', () => {
  let component: ShareContractComponent;
  let fixture: ComponentFixture<ShareContractComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ShareContractComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ShareContractComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
