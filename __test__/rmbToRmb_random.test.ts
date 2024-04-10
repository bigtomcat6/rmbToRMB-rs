import { after } from 'node:test';
import { rmbToRmb } from '../index';
import nzh from 'nzh';

let timeoutId: NodeJS.Timeout;

let numbers: number[] = [];

for (let i = 0; i < 5000; i++) {
  numbers.push(Number((Math.random() * 10000000000).toFixed(2)));
}

afterEach(() => {
  if (timeoutId)  clearTimeout(timeoutId);
});

describe('rmbToRmb', () => {
  jest.setTimeout(15000);
  
  // const nzhcn = require("nzh/cn");
  // test.each(numbers)(`should convert %s`, (n) => {
  //     expect(rmbToRmb(n)).toBe(nzh.cn.toMoney(n, {outSymbol:false}));
  // });

  test.each(numbers)(`AutoTest: %s`, async (n) => {
    await Promise.race([
      Promise.resolve(rmbToRmb(n)).then(result => {
        // 这里是函数成功返回后的处理
        expect(result).toBe(nzh.cn.toMoney(n, { outSymbol: false }));
        return result; // 返回结果以满足Promise.race的类型要求
      }),
      new Promise((_, reject) => timeoutId = setTimeout(() => reject(new Error('Timeout')), 10000)) // 10秒超时
    ]);
  });
});
