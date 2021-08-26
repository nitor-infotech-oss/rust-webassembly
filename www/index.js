import * as wasm from "wasm-game-of-life";

const addButton = document.getElementById("addition");
const subButton = document.getElementById("subtraction");
const multButton = document.getElementById("multiplication");
const divButton = document.getElementById("division");

const value1 = document.getElementById("no1");
const value2 = document.getElementById("no2");
const resultContainer = document.getElementById("result");


addButton.addEventListener("click", event => {
    const a = value1.value;
    const b = value2.value;
  
    console.log({a, b});
  
    const result = wasm.add_two_numbers(a, b);
  
    resultContainer.innerHTML = result;
  });


subButton.addEventListener("click", event => {
    const a = value1.value;
    const b = value2.value;
  
    console.log({a, b});
  
    const result = wasm.sub_two_numbers(a, b);
  
    resultContainer.innerHTML = result;
  });


multButton.addEventListener("click", event => {
    const a = value1.value;
    const b = value2.value;
  
    console.log({a, b});
  
    const result = wasm.mul_two_numbers(a, b);
  
    resultContainer.innerHTML = result;
  });

divButton.addEventListener("click", event => {
    const a = value1.value;
    const b = value2.value;
  
    console.log({a, b});
  
    const result = wasm.div_two_numbers(a, b);
  
    resultContainer.innerHTML = result;
  }); 

// wasm.greet();
// wasm.add_two_numbers(2,3);
// wasm.sub_two_numbers(10,5);
// wasm.mul_two_numbers(2,5);
// wasm.div_two_numbers(10,5);