function binarySearch(searchList: number[], value: number): number | null {
  let low = 0;
  let high = searchList.length - 1;

  while (low <= high) {
    const mid = Math.floor((low + high) / 2);
    const guess = searchList[mid];

    if (guess === value) {
      return mid;
    }

    if (guess < value) {
      low = mid + 1;
    } else {
      high = mid - 1;
    }
  }

  return null;
}

function main() {
  const searchList: number[] = [1, 2, 3, 4, 5, 6, 7, 8];
  const searchValue: number = 1;

  console.log("Binary Search");
  console.log("Search value: ", searchValue);

  const result = binarySearch(searchList, searchValue);
  const wrong = binarySearch(searchList, 12);

  console.log("Result: ", result);
  console.log("Wrong: ", wrong);
}

main();
