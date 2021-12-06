export function parseFish(input) {
  let fish = {};
  for (let i=0; i <= 8; i++) {
    fish[i] = 0;
  }
  input.split(',').map((s) => parseInt(s)).forEach((f) => {
    fish[f]++
  })
  return fish;
};

function simulateDay(oldFish) {
  let newFish = {};
  newFish[8] = oldFish[0]
  newFish[7] = oldFish[8]
  newFish[6] = oldFish[0] + oldFish[7]
  for (let i=0; i <= 5; i++) {
    newFish[i] = oldFish[i+1]
  }
  return newFish;
}

export function simulateDays(fish, n) {
  for (let i=0; i < n; i++) {
    fish = simulateDay(fish);
  }
  return fish;
}

export function countFish(fish) {
  let sum = 0;
  for (let i=0; i <= 8; i++) {
    sum += fish[i];
  }
  return sum;
}

