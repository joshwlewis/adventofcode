export function parseFish(input) {
  return input.split(',').map((s) => parseInt(s));
};

function simulateDay(fish) {
  let newFish = 0;
  fish.forEach((f,i) => {
    if (f === 0) {
      newFish++;
      fish[i] = 6;
    } else {
      fish[i]--;
    }
  });
  for (let i = 0; i < newFish; i++) {
    fish.push(8);
  }
}

export function simulateDays(fish, n) {
  for (let i=0; i < n; i++) {
    simulateDay(fish);
  }
}
