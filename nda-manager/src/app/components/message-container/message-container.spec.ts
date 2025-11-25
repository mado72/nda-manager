import { ComponentFixture, TestBed } from '@angular/core/testing';
import { DebugElement } from '@angular/core';
import { By } from '@angular/platform-browser';

import { MessageContainer } from './message-container';

describe('MessageContainer', () => {
  let component: MessageContainer;
  let fixture: ComponentFixture<MessageContainer>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MessageContainer]
    })
    .compileComponents();

    fixture = TestBed.createComponent(MessageContainer);
    component = fixture.componentInstance;
  });

  it('should create', () => {
    // Set required input
    fixture.componentRef.setInput('message', 'Test message');
    fixture.detectChanges();
    expect(component).toBeTruthy();
  });

  it('should display success message with correct styling', () => {
    fixture.componentRef.setInput('message', 'Contract created successfully');
    fixture.detectChanges();

    const messageContainer = fixture.debugElement.query(By.css('.message-container'));
    expect(messageContainer.nativeElement.classList).toContain('success-message');
    expect(messageContainer.nativeElement.textContent.trim()).toContain('Contract created successfully');
  });

  it('should display error message with correct styling', () => {
    fixture.componentRef.setInput('message', 'Error: Something went wrong');
    fixture.detectChanges();

    const messageContainer = fixture.debugElement.query(By.css('.message-container'));
    expect(messageContainer.nativeElement.classList).toContain('error-message');
    expect(messageContainer.nativeElement.textContent.trim()).toContain('Error: Something went wrong');
  });

  it('should display info message as default type', () => {
    fixture.componentRef.setInput('message', 'Some information');
    fixture.detectChanges();

    const messageContainer = fixture.debugElement.query(By.css('.message-container'));
    expect(messageContainer.nativeElement.classList).toContain('info-message');
  });

  it('should use explicit type when provided', () => {
    fixture.componentRef.setInput('message', 'Some message');
    fixture.componentRef.setInput('type', 'success');
    fixture.detectChanges();

    const messageContainer = fixture.debugElement.query(By.css('.message-container'));
    expect(messageContainer.nativeElement.classList).toContain('success-message');
  });
});
