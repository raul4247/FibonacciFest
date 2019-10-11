function fibonacci() {
    
    n = prompt();

    array = [0, 1];

    console.log("Hello HacktoberFest!");

    for (let i = 0; i < n; i++) {
        if (i > 1) {
            number = array[i-2] + array[i-1];
            array.push(number);
        } else {
            number = array[i];
        }
        console.log(number + " ");
    }
    
}