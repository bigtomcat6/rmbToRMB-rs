const { rmbToRmb } = require("../index.js");

describe('rmbToRmb', () => {

    let numbers = [];
    for (let i = 0; i < 1000; i++) {
        numbers.push(Number((Math.random() * 10000000000).toFixed(2)));
    }

    const nzhcn = require("nzh/cn");
    test.each(numbers)(`should convert %s`, (n) => {
        expect(rmbToRmb(n)).toBe(nzhcn.toMoney(n, {outSymbol:false}));
    })

});