const sum_of_array = (element, arr) => {
    var arr_2 = [element, Symbol("99"), Symbol("1"), Symbol("1"), ...arr]
    let sum = 0;
    for (let i = 0; i < arr_2.length; i++) {
        sum += Number(arr_2[i].description);
    }
    console.log(sum)
    new Promise((resolve, reject) => {
        if (sum > 30) {
            resolve("Sum is greater than 30");
        } else {
            reject("Sum is less than 30");
        }
    }).then((value) => {
        console.log(value);
    }).catch((error) => {
        console.log(error);
    });
}
(() => {
    var arr_1 = [Symbol(1), Symbol(2), Symbol("3"), Symbol("4"), Symbol("5")];
    const element = arr_1.shift();
    sum_of_array(element, arr_1);
})();