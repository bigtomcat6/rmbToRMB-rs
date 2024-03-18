import { rmbToRmb } from '../index';
import { describe, test, expect } from '@jest/globals';

describe('rmbToRmb', () => {
    test('should convert 0 to 零元整', () => {
        expect(rmbToRmb(0)).toBe('零元整');
    });

    test('should convert 1 to 壹元整', () => {
        expect(rmbToRmb(1)).toBe('壹元整');
    });

    test('should convert 10 to 壹拾元整', () => {
        expect(rmbToRmb(10)).toBe('壹拾元整');
    });

    test('should convert 100 to 壹佰元整', () => {
        expect(rmbToRmb(100)).toBe('壹佰元整');
    });

    test('should convert 1000 to 壹仟元整', () => {
        expect(rmbToRmb(1000)).toBe('壹仟元整');
    });

    test('should convert 10000 to 壹万元整', () => {
        expect(rmbToRmb(10000)).toBe('壹万元整');
    });

    test('should convert 100000 to 壹拾万元整', () => {
        expect(rmbToRmb(100000)).toBe('壹拾万元整');
    });

    // Add more test cases here to cover different scenarios and edge cases
    test('should convert 0.01 to 壹分', () => {
        expect(rmbToRmb(0.01)).toBe('零壹分');
    });
    test('should convert 0.1 to 壹角', () => {
        expect(rmbToRmb(0.1)).toBe('壹角');
    });

});
