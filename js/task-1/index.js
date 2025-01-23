(async function () {
    try {
      const arr = [1,2,3,4,5];
      const ele = arr.shift();
      const result = await helper(arr,ele);
      console.log(result);
    } catch (error) {
      console.log(error);
    }
  })();

  async function helper (arr,ele){
    const arr2 = [ele,6,7,-8,-9,...arr];
 
    let total = 0;
    for (let i = 0; i < arr2.length; i++) {
        total += arr2[i];
    }
    const myPromise = new Promise((resolve, reject) => {
        if (total > 30) {
            resolve("more than 30");
        } else {
            reject("Less than 30.");
        }
      });
      return myPromise;
}