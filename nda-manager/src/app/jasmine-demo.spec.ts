import { TestBed } from '@angular/core/testing';

describe('Jasmine 5.10.0 Features Demo', () => {
  let testService: any;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    
    // Mock service for demonstration
    testService = {
      getValue: () => 'test value',
      getAsyncValue: () => Promise.resolve('async value'),
      throwError: () => { throw new Error('Test error'); }
    };
  });

  it('should demonstrate basic Jasmine functionality', () => {
    expect(testService.getValue()).toBe('test value');
    expect(testService.getValue()).not.toBe('other value');
    expect(testService.getValue()).toContain('test');
  });

  it('should handle async operations', async () => {
    const result = await testService.getAsyncValue();
    expect(result).toBe('async value');
  });

  it('should test error handling', () => {
    expect(() => testService.throwError()).toThrowError('Test error');
  });

  it('should use spies for mocking', () => {
    const spy = spyOn(testService, 'getValue').and.returnValue('mocked value');
    
    const result = testService.getValue();
    
    expect(result).toBe('mocked value');
    expect(spy).toHaveBeenCalled();
    expect(spy).toHaveBeenCalledTimes(1);
  });

  it('should demonstrate custom matchers and asymmetric matchers', () => {
    const data = {
      id: 1,
      name: 'Test',
      timestamp: new Date(),
      tags: ['angular', 'testing']
    };

    expect(data).toEqual(jasmine.objectContaining({
      id: jasmine.any(Number),
      name: jasmine.any(String),
      tags: jasmine.arrayContaining(['angular'])
    }));
  });

  it('should test with done callback for async operations', (done) => {
    setTimeout(() => {
      expect(true).toBe(true);
      done();
    }, 10);
  });

  describe('Nested describe blocks', () => {
    it('should work in nested contexts', () => {
      expect('Jasmine 5.10.0').toMatch(/^Jasmine \d+\.\d+\.\d+$/);
    });

    it('should support pending tests', () => {
      pending('This test is pending implementation');
    });
  });
});