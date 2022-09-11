import { foo } from './index';

describe('Hello', () => {
  it('World', () => {
    console.log('Hello, World');

    expect(foo()).toBe(2);
  });
});